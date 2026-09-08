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
