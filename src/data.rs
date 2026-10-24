use reqwest::Method;
use serde_json::json;

use crate::client::Client;
use crate::error::DBByteError;
use crate::types::{
    AppendTextResponse, DataDeleteResponse, DataPage, DataQueryOptions, DataRecord,
    DataRecordResponse,
};
use crate::utils::encode_key;

/// Serve data APIs published from CSV, JSON, or Google Docs.
pub struct DataResource<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> DataResource<'a> {
    /// Reads a page from a published data API. Filters are exact matches.
    pub fn get(&self, slug: &str, opts: DataQueryOptions) -> Result<DataPage, DBByteError> {
        let fields = opts.fields.join(",");
        let cursor = opts.cursor.unwrap_or_default();
        let limit = opts.limit.map(|l| l.to_string()).unwrap_or_default();
        let mut query: Vec<(&str, &str)> = vec![
            ("fields", fields.as_str()),
            ("cursor", cursor.as_str()),
            ("limit", limit.as_str()),
        ];
        for (k, v) in &opts.filters {
            query.push((k.as_str(), v.as_str()));
        }
        self.client.request_json(
            Method::GET,
            &format!("/v1/data/{}", encode_key(slug)),
            &query,
            None::<&()>,
        )
    }

    /// Inserts a JSON record into a write-enabled data API.
    pub fn create(
        &self,
        slug: &str,
        record: DataRecord,
    ) -> Result<DataRecordResponse, DBByteError> {
        self.client.request_json(
            Method::POST,
            &format!("/v1/data/{}", encode_key(slug)),
            &[],
            Some(&record),
        )
    }

    /// Appends text to a write-enabled Google Docs data API.
    pub fn append_text(&self, slug: &str, text: &str) -> Result<AppendTextResponse, DBByteError> {
        let body = json!({ "appendText": text });
        self.client.request_json(
            Method::POST,
            &format!("/v1/data/{}", encode_key(slug)),
            &[],
            Some(&body),
        )
    }

    /// Patches writable fields on a record.
    pub fn update(
        &self,
        slug: &str,
        record_id: &str,
        patch: DataRecord,
    ) -> Result<DataRecordResponse, DBByteError> {
        self.client.request_json(
            Method::PATCH,
            &format!("/v1/data/{}/{}", encode_key(slug), encode_key(record_id)),
            &[],
            Some(&patch),
        )
    }

    /// Deletes a record from a write-enabled JSON data API.
    pub fn delete(&self, slug: &str, record_id: &str) -> Result<DataDeleteResponse, DBByteError> {
        self.client.request_json(
            Method::DELETE,
            &format!("/v1/data/{}/{}", encode_key(slug), encode_key(record_id)),
            &[],
            None::<&()>,
        )
    }
}
