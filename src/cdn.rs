use reqwest::Method;

use crate::client::Client;
use crate::error::DBByteError;
use crate::types::{PurgeOptions, PurgeResponse};

/// Cache invalidation. The API key needs write permission.
pub struct CdnResource<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> CdnResource<'a> {
    /// Purges one public file (`PurgeScope::File`, with `path`) or every
    /// public file in a bucket (`PurgeScope::Bucket`).
    pub fn purge(&self, opts: PurgeOptions) -> Result<PurgeResponse, DBByteError> {
        self.client
            .request_json(Method::POST, "/purge", &[], Some(&opts))
    }
}
