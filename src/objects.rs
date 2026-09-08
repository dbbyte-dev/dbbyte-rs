use reqwest::Method;
use serde_json::json;

use crate::client::Client;
use crate::error::DBByteError;
use crate::types::{
    DeleteResponse, DownloadResult, ListObjectsOptions, ListObjectsResponse,
    ListVersionsOptions, ListVersionsResponse, SignOptions, SignedUrlResponse, UploadOptions,
    UploadedObject,
};
use crate::utils::encode_key;

/// Groups every object-level operation (upload, list, download, delete,
/// sign, versions) under one bucket-scoped namespace, matching the Node
/// and Python SDKs.
pub struct ObjectsResource<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> ObjectsResource<'a> {
    /// Uploads a single object to `bucket`.
    pub fn upload(&self, bucket: &str, opts: UploadOptions) -> Result<UploadedObject, DBByteError> {
        let overwrite_str = if opts.overwrite { "true" } else { "" };
        let query = [("overwrite", overwrite_str)];
        self.client.request_form(
            &format!("/storage/{bucket}/upload"),
            &query,
            &opts.key,
            opts.body,
            opts.content_type.as_deref(),
        )
    }

    /// Returns a page of objects in `bucket`.
    pub fn list(
        &self,
        bucket: &str,
        opts: ListObjectsOptions,
    ) -> Result<ListObjectsResponse, DBByteError> {
        let prefix = opts.prefix.unwrap_or_default();
        let cursor = opts.cursor.unwrap_or_default();
        let limit_str = opts.limit.map(|l| l.to_string()).unwrap_or_default();
        let query = [
            ("prefix", prefix.as_str()),
            ("cursor", cursor.as_str()),
            ("limit", limit_str.as_str()),
        ];
        self.client
            .request_json(Method::GET, &format!("/storage/{bucket}/list"), &query, None::<&()>)
    }

    /// Fetches a single object's bytes. Pass `version` (empty string for
    /// "latest") to fetch a specific version instead.
    pub fn download(
        &self,
        bucket: &str,
        key: &str,
        version: &str,
    ) -> Result<DownloadResult, DBByteError> {
        let query = [("version", version)];
        self.client.request_binary(
            Method::GET,
            &format!("/storage/{bucket}/download/{}", encode_key(key)),
            &query,
        )
    }

    /// Returns a zip of every version of `key`.
    pub fn download_all_versions(
        &self,
        bucket: &str,
        key: &str,
    ) -> Result<DownloadResult, DBByteError> {
        let query = [("version", "all")];
        self.client.request_binary(
            Method::GET,
            &format!("/storage/{bucket}/download/{}", encode_key(key)),
            &query,
        )
    }

    /// Deletes an object. Pass `version` (empty string for "latest") to
    /// delete a specific version instead.
    pub fn delete(
        &self,
        bucket: &str,
        key: &str,
        version: &str,
    ) -> Result<DeleteResponse, DBByteError> {
        let query = [("version", version)];
        self.client.request_json(
            Method::DELETE,
            &format!("/storage/{bucket}/delete/{}", encode_key(key)),
            &query,
            None::<&()>,
        )
    }

    /// Permanently deletes every version of `key`.
    pub fn delete_all_versions(
        &self,
        bucket: &str,
        key: &str,
    ) -> Result<DeleteResponse, DBByteError> {
        let query = [("version", "all")];
        self.client.request_json(
            Method::DELETE,
            &format!("/storage/{bucket}/delete/{}", encode_key(key)),
            &query,
            None::<&()>,
        )
    }

    /// Creates a signed, time-limited URL for `key`.
    pub fn sign(
        &self,
        bucket: &str,
        key: &str,
        opts: SignOptions,
    ) -> Result<SignedUrlResponse, DBByteError> {
        let body = json!({
            "version": opts.version,
            "expiresIn": opts.expires_in,
        });
        self.client.request_json(
            Method::POST,
            &format!("/storage/{bucket}/sign/{}", encode_key(key)),
            &[],
            Some(&body),
        )
    }

    /// Lists the version history of `key`.
    pub fn versions(
        &self,
        bucket: &str,
        key: &str,
        opts: ListVersionsOptions,
    ) -> Result<ListVersionsResponse, DBByteError> {
        let cursor = opts.cursor.unwrap_or_default();
        let limit_str = opts.limit.map(|l| l.to_string()).unwrap_or_default();
        let query = [("cursor", cursor.as_str()), ("limit", limit_str.as_str())];
        self.client.request_json(
            Method::GET,
            &format!("/storage/{bucket}/versions/{}", encode_key(key)),
            &query,
            None::<&()>,
        )
    }
}
