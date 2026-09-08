//! Official Rust SDK for the DBByte storage/CDN API.
//!
//! ```no_run
//! use dbbyte::{DBByte, ClientOptions, UploadOptions};
//!
//! let dbb = DBByte::new(ClientOptions::new("dbk_...")).unwrap();
//!
//! let uploaded = dbb.objects().upload("my-bucket", UploadOptions {
//!     key: "images/logo.png".to_string(),
//!     body: std::fs::read("logo.png").unwrap(),
//!     content_type: Some("image/png".to_string()),
//!     overwrite: false,
//! }).unwrap();
//! ```

mod client;
mod error;
mod objects;
mod types;
mod utils;

pub use client::ClientOptions;
pub use error::DBByteError;
pub use objects::ObjectsResource;
pub use types::{
    DeleteResponse, DownloadResult, FileListItem, FileVersionPreview, HealthChecks,
    HealthCheckStatus, HealthDatabaseStatus, HealthResponse, ListObjectsOptions,
    ListObjectsResponse, ListVersionsOptions, ListVersionsResponse, SignOptions,
    SignedUrlResponse, UploadOptions, UploadedObject, VersionEntry,
};

use client::Client;

/// Top-level client. Construct with [`DBByte::new`].
pub struct DBByte {
    client: Client,
}

impl DBByte {
    /// Constructs a DBByte client. `api_key` is required on
    /// [`ClientOptions`]; all other options have sane defaults.
    pub fn new(opts: ClientOptions) -> Result<Self, DBByteError> {
        Ok(Self {
            client: Client::new(opts)?,
        })
    }

    /// Returns the bucket-scoped resource for object operations (upload,
    /// list, download, delete, sign, versions).
    pub fn objects(&self) -> ObjectsResource<'_> {
        ObjectsResource {
            client: &self.client,
        }
    }

    /// Calls `GET /health`.
    pub fn health(&self) -> Result<HealthResponse, DBByteError> {
        self.client
            .request_json(reqwest::Method::GET, "/health", &[], None::<&()>)
    }
}
