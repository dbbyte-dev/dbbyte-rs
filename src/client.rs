use std::time::Duration;

use reqwest::blocking::{multipart, Client as HttpClient};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, USER_AGENT};
use serde::Serialize;
use serde_json::Value;

use crate::error::DBByteError;
use crate::types::DownloadResult;

const DEFAULT_BASE_URL: &str = "https://api.dbbyte.com";
const DEFAULT_TIMEOUT_SECS: u64 = 30;
const SDK_USER_AGENT: &str = "dbbyte-rust/0.1.0";

/// Configures a [`crate::DBByte`] client.
pub struct ClientOptions {
    pub api_key: String,
    /// Defaults to `https://api.dbbyte.com`. In production the Cloudflare
    /// Worker gateway in front of that host inserts `/api/v1` itself, so
    /// paths used internally by this SDK do NOT include `/v1`. If you
    /// point `base_url` at a Next.js dev server directly (bypassing the
    /// Worker), include `/api/v1` in `base_url` instead.
    pub base_url: String,
    pub timeout: Duration,
}

impl Default for ClientOptions {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            base_url: DEFAULT_BASE_URL.to_string(),
            timeout: Duration::from_secs(DEFAULT_TIMEOUT_SECS),
        }
    }
}

impl ClientOptions {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            ..Default::default()
        }
    }

    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }
}

pub(crate) struct Client {
    base_url: String,
    http: HttpClient,
}

impl Client {
    pub(crate) fn new(opts: ClientOptions) -> Result<Self, DBByteError> {
        if opts.api_key.is_empty() {
            return Err(DBByteError::transport("dbbyte: api_key is required"));
        }

        let mut headers = HeaderMap::new();
        let auth_value = format!("Bearer {}", opts.api_key);
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&auth_value)
                .map_err(|e| DBByteError::transport(format!("invalid api_key: {e}")))?,
        );
        headers.insert(USER_AGENT, HeaderValue::from_static(SDK_USER_AGENT));

        let http = HttpClient::builder()
            .timeout(opts.timeout)
            .default_headers(headers)
            .build()
            .map_err(|e| DBByteError::transport(e.to_string()))?;

        Ok(Self {
            base_url: opts.base_url.trim_end_matches('/').to_string(),
            http,
        })
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    fn map_transport_err(e: reqwest::Error) -> DBByteError {
        if e.is_timeout() {
            DBByteError::transport("Request timed out")
        } else {
            DBByteError::transport(e.to_string())
        }
    }

    fn error_from_response(res: reqwest::blocking::Response) -> DBByteError {
        let status = res.status().as_u16();
        let retry_after = res
            .headers()
            .get("Retry-After")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse::<u64>().ok());

        let default_message = res
            .status()
            .canonical_reason()
            .unwrap_or("Request failed")
            .to_string();

        match res.json::<Value>() {
            Ok(body) => {
                let message = body
                    .get("error")
                    .and_then(|v| v.as_str())
                    .map(str::to_string)
                    .unwrap_or(default_message);
                let hint = body
                    .get("hint")
                    .and_then(|v| v.as_str())
                    .map(str::to_string);
                DBByteError::new(message, status, hint, retry_after)
            }
            Err(_) => DBByteError::new(default_message, status, None, retry_after),
        }
    }

    pub(crate) fn request_json<T, B>(
        &self,
        method: reqwest::Method,
        path: &str,
        query: &[(&str, &str)],
        body: Option<&B>,
    ) -> Result<T, DBByteError>
    where
        T: for<'de> serde::Deserialize<'de>,
        B: Serialize + ?Sized,
    {
        let mut req = self.http.request(method, self.url(path));
        req = req.query(&non_empty(query));
        if let Some(b) = body {
            req = req.json(b);
        }

        let res = req.send().map_err(Self::map_transport_err)?;
        if !res.status().is_success() {
            return Err(Self::error_from_response(res));
        }
        res.json::<T>()
            .map_err(|e| DBByteError::transport(e.to_string()))
    }

    pub(crate) fn request_form<T: for<'de> serde::Deserialize<'de>>(
        &self,
        path: &str,
        query: &[(&str, &str)],
        filename: &str,
        file_bytes: Vec<u8>,
        content_type: Option<&str>,
    ) -> Result<T, DBByteError> {
        let mut part = multipart::Part::bytes(file_bytes).file_name(filename.to_string());
        if let Some(ct) = content_type {
            part = part
                .mime_str(ct)
                .map_err(|e| DBByteError::transport(e.to_string()))?;
        }
        let form = multipart::Form::new().part("file", part);

        let req = self
            .http
            .post(self.url(path))
            .query(&non_empty(query))
            .multipart(form);

        let res = req.send().map_err(Self::map_transport_err)?;
        if !res.status().is_success() {
            return Err(Self::error_from_response(res));
        }
        res.json::<T>()
            .map_err(|e| DBByteError::transport(e.to_string()))
    }

    pub(crate) fn request_binary(
        &self,
        method: reqwest::Method,
        path: &str,
        query: &[(&str, &str)],
    ) -> Result<DownloadResult, DBByteError> {
        let mut req = self.http.request(method, self.url(path));
        req = req.query(&non_empty(query));

        let res = req.send().map_err(Self::map_transport_err)?;
        if !res.status().is_success() {
            return Err(Self::error_from_response(res));
        }

        let content_type = res
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("application/octet-stream")
            .to_string();

        let filename = res
            .headers()
            .get(reqwest::header::CONTENT_DISPOSITION)
            .and_then(|v| v.to_str().ok())
            .and_then(parse_disposition_filename);

        let data = res
            .bytes()
            .map_err(|e| DBByteError::transport(e.to_string()))?
            .to_vec();

        Ok(DownloadResult {
            data,
            content_type,
            filename,
        })
    }
}

fn non_empty<'a>(query: &[(&'a str, &'a str)]) -> Vec<(&'a str, &'a str)> {
    query.iter().filter(|(_, v)| !v.is_empty()).cloned().collect()
}

fn parse_disposition_filename(disposition: &str) -> Option<String> {
    let marker = "filename=\"";
    let start = disposition.find(marker)? + marker.len();
    let rest = &disposition[start..];
    let end = rest.find('"')?;
    Some(rest[..end].to_string())
}
