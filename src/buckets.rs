use reqwest::Method;

use crate::client::Client;
use crate::error::DBByteError;
use crate::types::{
    BucketDeleteResponse, BucketListResponse, BucketResponse, CreateBucketOptions,
    UpdateBucketOptions,
};
use crate::utils::encode_key;

/// Account-level bucket management.
pub struct BucketsResource<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> BucketsResource<'a> {
    /// Lists every bucket on the account.
    pub fn list(&self) -> Result<BucketListResponse, DBByteError> {
        self.client
            .request_json(Method::GET, "/buckets", &[], None::<&()>)
    }

    /// Returns one bucket by slug.
    pub fn get(&self, slug: &str) -> Result<BucketResponse, DBByteError> {
        self.client.request_json(
            Method::GET,
            &format!("/buckets/{}", encode_key(slug)),
            &[],
            None::<&()>,
        )
    }

    /// Creates a bucket.
    pub fn create(&self, opts: CreateBucketOptions) -> Result<BucketResponse, DBByteError> {
        self.client
            .request_json(Method::POST, "/buckets", &[], Some(&opts))
    }

    /// Changes versioning, public access, or CDN caching on a bucket.
    pub fn update(
        &self,
        slug: &str,
        opts: UpdateBucketOptions,
    ) -> Result<BucketResponse, DBByteError> {
        self.client.request_json(
            Method::PATCH,
            &format!("/buckets/{}", encode_key(slug)),
            &[],
            Some(&opts),
        )
    }

    /// Deletes a non-default bucket and its contents.
    pub fn delete(&self, slug: &str) -> Result<BucketDeleteResponse, DBByteError> {
        self.client.request_json(
            Method::DELETE,
            &format!("/buckets/{}", encode_key(slug)),
            &[],
            None::<&()>,
        )
    }
}
