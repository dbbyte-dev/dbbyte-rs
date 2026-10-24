use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;

use dbbyte::{
    ClientOptions, CreateBucketOptions, DBByte, DataQueryOptions, PurgeOptions, PurgeScope,
    UpdateBucketOptions, UploadUrlOptions,
};

/// Serves one canned JSON response and returns the raw request text.
fn mock(response: &'static str) -> (String, mpsc::Receiver<String>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let (mut sock, _) = listener.accept().unwrap();
        let mut buf = Vec::new();
        let mut chunk = [0u8; 4096];
        loop {
            let n = sock.read(&mut chunk).unwrap();
            buf.extend_from_slice(&chunk[..n]);
            let text = String::from_utf8_lossy(&buf).to_string();
            if let Some(i) = text.find("\r\n\r\n") {
                let len = text
                    .lines()
                    .find_map(|l| {
                        l.to_lowercase()
                            .strip_prefix("content-length:")
                            .map(|v| v.trim().parse::<usize>().unwrap())
                    })
                    .unwrap_or(0);
                if buf.len() >= i + 4 + len {
                    break;
                }
            }
        }
        tx.send(String::from_utf8_lossy(&buf).to_string()).unwrap();
        let out = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            response.len(),
            response
        );
        sock.write_all(out.as_bytes()).unwrap();
    });
    (base, rx)
}

fn client(base: String) -> DBByte {
    DBByte::new(ClientOptions::new("dbk_test").base_url(base)).unwrap()
}

const BUCKET: &str = r#"{"success":true,"bucket":{"id":"1","name":"Media","slug":"media","region":"us-east","replicaCount":1,"isPublic":true,"cdn":true,"versioningEnabled":false,"fileCount":0,"size":0,"isDefault":false,"createdAt":"","updatedAt":""}}"#;

#[test]
fn buckets_create_and_update() {
    let (base, rx) = mock(BUCKET);
    let res = client(base)
        .buckets()
        .create(CreateBucketOptions {
            name: "Media".into(),
            region: "us-east".into(),
            is_public: Some(true),
            ..Default::default()
        })
        .unwrap();
    assert_eq!(res.bucket.slug, "media");
    let req = rx.recv().unwrap();
    assert!(req.starts_with("POST /buckets "));
    assert!(req.contains(r#""isPublic":true"#) && !req.contains("replicaCount"));

    let (base, rx) = mock(BUCKET);
    client(base)
        .buckets()
        .update(
            "media",
            UpdateBucketOptions {
                cdn: Some(false),
                ..Default::default()
            },
        )
        .unwrap();
    let req = rx.recv().unwrap();
    assert!(req.starts_with("PATCH /buckets/media "));
    assert!(req.contains(r#""cdn":false"#) && !req.contains("isPublic"));
}

#[test]
fn bucket_delete_and_list() {
    let (base, rx) = mock(r#"{"success":true}"#);
    assert!(client(base).buckets().delete("media").unwrap().success);
    assert!(rx.recv().unwrap().starts_with("DELETE /buckets/media "));

    let (base, _rx) = mock(r#"{"buckets":[]}"#);
    assert!(client(base).buckets().list().unwrap().buckets.is_empty());
}

#[test]
fn cdn_purge() {
    let (base, rx) = mock(r#"{"success":true,"purged":1,"failed":[]}"#);
    let res = client(base)
        .cdn()
        .purge(PurgeOptions {
            slug: "media".into(),
            scope: PurgeScope::File,
            path: Some("a.png".into()),
        })
        .unwrap();
    assert_eq!(res.purged, 1);
    let req = rx.recv().unwrap();
    assert!(req.starts_with("POST /purge "));
    assert!(req.contains(r#""scope":"file""#) && req.contains(r#""path":"a.png""#));
}

#[test]
fn data_get_and_append() {
    let (base, rx) = mock(r#"{"data":[{"a":"1"}],"next_cursor":null}"#);
    let page = client(base)
        .data()
        .get(
            "people",
            DataQueryOptions {
                limit: Some(5),
                filters: vec![("city".into(), "Oslo".into())],
                ..Default::default()
            },
        )
        .unwrap();
    assert_eq!(page.data.len(), 1);
    let req = rx.recv().unwrap();
    assert!(req.starts_with("GET /v1/data/people?"));
    assert!(req.contains("limit=5") && req.contains("city=Oslo"));

    let (base, rx) = mock(r#"{"success":true,"appendedCharacters":2}"#);
    client(base).data().append_text("notes", "hi").unwrap();
    assert!(rx.recv().unwrap().contains(r#""appendText":"hi""#));
}

#[test]
fn upload_url() {
    let (base, rx) = mock(
        r#"{"url":"https://x/y","method":"PUT","headers":{"Content-Type":"image/png"},"expiresIn":900}"#,
    );
    let res = client(base)
        .objects()
        .upload_url(
            "media",
            UploadUrlOptions {
                name: "a.png".into(),
                content_type: Some("image/png".into()),
                size: Some(10),
                ..Default::default()
            },
        )
        .unwrap();
    assert_eq!(res.method, "PUT");
    assert_eq!(res.expires_in, 900);
    let req = rx.recv().unwrap();
    assert!(req.starts_with("POST /storage/media/upload-url "));
    assert!(req.contains(r#""contentType":"image/png""#) && !req.contains("overwrite"));
}
