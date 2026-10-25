use serde::{Deserialize, Serialize};

/// Mirrors GET /health.
#[derive(Debug, Clone, Deserialize)]
pub struct HealthResponse {
    pub status: String, // "ok" | "degraded"
    pub timestamp: String,
    #[serde(rename = "uptimeSeconds")]
    pub uptime_seconds: u64,
    pub checks: HealthChecks,
    #[serde(rename = "responseTimeMs")]
    pub response_time_ms: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HealthChecks {
    pub api: HealthCheckStatus,
    pub database: HealthDatabaseStatus,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HealthCheckStatus {
    pub status: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HealthDatabaseStatus {
    pub status: String,
    #[serde(rename = "latencyMs")]
    pub latency_ms: Option<u64>,
}

/// Returned on a successful upload. Matches the storage server's real
/// response shape exactly - not inferred.
#[derive(Debug, Clone, Deserialize)]
pub struct UploadedObject {
    pub success: bool,
    /// Server-generated storage key, not the name you passed.
    pub key: String,
    /// The name you uploaded it as.
    pub name: String,
    pub size: u64,
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    #[serde(rename = "fileId")]
    pub file_id: String,
}

/// Options for [`crate::ObjectsResource::upload`].
#[derive(Debug, Clone, Default)]
pub struct UploadOptions {
    pub key: String,
    pub body: Vec<u8>,
    pub content_type: Option<String>,
    pub overwrite: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FileVersionPreview {
    pub key: String,
    pub size: u64,
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    #[serde(rename = "isLatest")]
    pub is_latest: bool,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FileListItem {
    pub key: String,
    pub name: String,
    pub size: u64,
    #[serde(rename = "isPublic")]
    pub is_public: bool,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(rename = "versionCount")]
    pub version_count: u64,
    pub versions: Vec<FileVersionPreview>,
    #[serde(rename = "hasMoreVersions")]
    pub has_more_versions: bool,
}

/// Options for [`crate::ObjectsResource::list`].
///
/// NOTE: as of the current API, `prefix` filters on the storage-generated
/// `key` field (a timestamp-prefixed, slash-flattened server ID), not the
/// human-readable `name` you uploaded with. It will rarely match what you
/// expect - confirmed via round-trip testing against a real server. Treat
/// as unreliable until the backend filters on `name` instead.
#[derive(Debug, Clone, Default)]
pub struct ListObjectsOptions {
    pub prefix: Option<String>,
    pub cursor: Option<String>,
    pub limit: Option<u32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListObjectsResponse {
    pub files: Vec<FileListItem>,
    #[serde(rename = "nextCursor")]
    pub next_cursor: Option<String>,
    #[serde(rename = "hasMore")]
    pub has_more: bool,
}

/// Options for [`crate::ObjectsResource::sign`].
#[derive(Debug, Clone, Default, Serialize)]
pub struct SignOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "expiresIn", skip_serializing_if = "Option::is_none")]
    pub expires_in: Option<u64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SignedUrlResponse {
    pub url: String,
    #[serde(rename = "expiresIn")]
    pub expires_in: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VersionEntry {
    #[serde(rename = "versionKey")]
    pub version_key: String,
    pub size: u64,
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    #[serde(rename = "isLatest")]
    pub is_latest: bool,
    #[serde(rename = "isDeleted")]
    pub is_deleted: bool,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

/// Options for [`crate::ObjectsResource::versions`].
#[derive(Debug, Clone, Default)]
pub struct ListVersionsOptions {
    pub limit: Option<u32>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListVersionsResponse {
    pub versions: Vec<VersionEntry>,
    #[serde(rename = "nextCursor")]
    pub next_cursor: Option<String>,
    #[serde(rename = "hasMore")]
    pub has_more: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteResponse {
    pub success: bool,
    /// Only present when deleting every version at once.
    #[serde(rename = "deletedVersions")]
    pub deleted_versions: Option<u64>,
}

/// Holds a downloaded object's bytes and metadata. `filename` is `None`
/// when the server didn't send a Content-Disposition filename.
#[derive(Debug, Clone)]
pub struct DownloadResult {
    pub data: Vec<u8>,
    pub content_type: String,
    pub filename: Option<String>,
}

/// A bucket as returned by the bucket endpoints.
#[derive(Debug, Clone, Deserialize)]
pub struct Bucket {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub region: String,
    #[serde(rename = "replicaCount", default)]
    pub replica_count: i64,
    #[serde(rename = "isPublic", default)]
    pub is_public: bool,
    #[serde(default)]
    pub cdn: bool,
    #[serde(rename = "versioningEnabled", default)]
    pub versioning_enabled: bool,
    #[serde(rename = "fileCount", default)]
    pub file_count: i64,
    #[serde(default)]
    pub size: i64,
    #[serde(rename = "isDefault", default)]
    pub is_default: bool,
    #[serde(rename = "createdAt", default)]
    pub created_at: String,
    #[serde(rename = "updatedAt", default)]
    pub updated_at: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BucketListResponse {
    pub buckets: Vec<Bucket>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BucketResponse {
    #[serde(default)]
    pub success: bool,
    pub bucket: Bucket,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BucketDeleteResponse {
    pub success: bool,
}

/// Options for [`crate::BucketsResource::create`].
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateBucketOptions {
    pub name: String,
    pub region: String,
    #[serde(rename = "replicaCount", skip_serializing_if = "Option::is_none")]
    pub replica_count: Option<u32>,
    #[serde(rename = "isPublic", skip_serializing_if = "Option::is_none")]
    pub is_public: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdn: Option<bool>,
}

/// Options for [`crate::BucketsResource::update`]. `None` leaves a setting unchanged.
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateBucketOptions {
    #[serde(rename = "versioningEnabled", skip_serializing_if = "Option::is_none")]
    pub versioning_enabled: Option<bool>,
    #[serde(rename = "isPublic", skip_serializing_if = "Option::is_none")]
    pub is_public: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdn: Option<bool>,
}

/// Selects a single-file or whole-bucket cache purge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PurgeScope {
    File,
    Bucket,
}

/// Options for [`crate::CdnResource::purge`]. `path` is required for file purges.
#[derive(Debug, Clone, Serialize)]
pub struct PurgeOptions {
    pub slug: String,
    pub scope: PurgeScope,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PurgeResponse {
    pub success: bool,
    #[serde(default)]
    pub purged: i64,
    #[serde(default)]
    pub failed: Vec<String>,
}

/// A record in a Serve data API.
pub type DataRecord = serde_json::Map<String, serde_json::Value>;

/// Options for [`crate::DataResource::get`]. Filters are exact string matches.
#[derive(Debug, Clone, Default)]
pub struct DataQueryOptions {
    pub fields: Vec<String>,
    pub cursor: Option<String>,
    pub limit: Option<u32>,
    pub filters: Vec<(String, String)>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DataPage {
    pub data: Vec<DataRecord>,
    #[serde(default)]
    pub next_cursor: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DataRecordResponse {
    pub data: DataRecord,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AppendTextResponse {
    pub success: bool,
    #[serde(rename = "appendedCharacters", default)]
    pub appended_characters: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DataDeleteResponse {
    pub success: bool,
}

/// Options for [`crate::ObjectsResource::upload_url`].
#[derive(Debug, Clone, Default, Serialize)]
pub struct UploadUrlOptions {
    /// Object name the upload will create (required, flat, no slashes).
    pub name: String,
    #[serde(rename = "contentType", skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// Maximum bytes the link accepts (up to 50 MB).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,
    #[serde(skip_serializing_if = "is_false")]
    pub overwrite: bool,
    #[serde(rename = "expiresIn", skip_serializing_if = "Option::is_none")]
    pub expires_in: Option<u32>,
}

fn is_false(b: &bool) -> bool {
    !*b
}

/// A short-lived direct-upload link. Send `method` to `url` with `headers`
/// and the file bytes as the body.
#[derive(Debug, Clone, Deserialize)]
pub struct UploadUrlResponse {
    pub url: String,
    pub method: String,
    #[serde(default)]
    pub headers: std::collections::HashMap<String, String>,
    #[serde(rename = "expiresIn", default)]
    pub expires_in: u32,
}
