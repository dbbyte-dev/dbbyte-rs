# dbbyte (Rust)

Official Rust SDK for the DBByte storage/CDN API. Synchronous (blocking)
client — no async runtime required.

## Install

```toml
[dependencies]
dbbyte = "0.1"
```

## Usage

```rust
use dbbyte::{DBByte, ClientOptions, UploadOptions};

let dbb = DBByte::new(ClientOptions::new("dbk_..."))?;

let uploaded = dbb.objects().upload("my-bucket", UploadOptions {
    key: "images/logo.png".to_string(),
    body: std::fs::read("logo.png")?,
    content_type: Some("image/png".to_string()),
    overwrite: false,
})?;
```

See `examples/basic.rs` for a fuller walkthrough (`cargo run --example basic`).

## More operations

```rust
use dbbyte::*;

// Buckets
dbb.buckets().list()?;
dbb.buckets().create(CreateBucketOptions {
    name: "Media".into(),
    region: "us-east".into(),
    is_public: Some(true),
    ..Default::default()
})?;
dbb.buckets().update("media", UpdateBucketOptions { versioning_enabled: Some(true), ..Default::default() })?;
dbb.buckets().delete("media")?;

// Cache purge (API key needs write permission)
dbb.cdn().purge(PurgeOptions { slug: "media".into(), scope: PurgeScope::File, path: Some("logo.png".into()) })?;

// Signed upload URL for direct browser uploads
let link = dbb.objects().upload_url("media", UploadUrlOptions {
    name: "photo.jpg".into(),
    content_type: Some("image/jpeg".into()),
    ..Default::default()
})?;

// Serve data APIs
dbb.data().get("people", DataQueryOptions { limit: Some(20), ..Default::default() })?;
```

## Errors

Every failed request returns a `DBByteError` with `message`, `status`,
`hint`, and `retry_after`. The API has no `code` field — don't rely on one.

A `429` can mean two different things, and only one is retryable:

- **Burst rate limit** — `retry_after` is `Some(_)` (seconds). Transient,
  safe to retry after that delay.
- **Plan API request quota exceeded** — `retry_after` is `None`. Retrying
  immediately won't help until the billing period resets or the plan is
  upgraded.

Check `retry_after`, not the message text, to tell them apart.

## Base URL / gateway note

By default the client talks to `https://api.dbbyte.com`. In production the
Cloudflare Worker gateway in front of that host inserts `/api/v1` itself,
so paths used internally by this SDK do **not** include `/v1`. If you point
`base_url` at a Next.js dev server directly (bypassing the Worker), include
`/api/v1` in `base_url` instead:

```rust
let dbb = DBByte::new(
    ClientOptions::new("dbk_...").base_url("http://localhost:3000/api/v1")
)?;
```
