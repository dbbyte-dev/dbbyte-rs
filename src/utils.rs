/// Encodes an object key for use in a URL path. Object keys can contain
/// slashes (e.g. "images/logo.png"). The API's catch-all routes expect
/// each path segment encoded individually, then rejoined with "/" -
/// encoding the whole key would also encode the slashes themselves.
pub(crate) fn encode_key(key: &str) -> String {
    key.split('/')
        .map(|segment| percent_encode_segment(segment))
        .collect::<Vec<_>>()
        .join("/")
}

// Minimal percent-encoding for a single path segment, matching
// `encodeURIComponent`/Python's `urllib.parse.quote(safe="")` behavior:
// everything except unreserved characters (A-Z a-z 0-9 - _ . ~) is encoded.
fn percent_encode_segment(segment: &str) -> String {
    let mut out = String::with_capacity(segment.len());
    for byte in segment.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(byte as char);
            }
            _ => {
                out.push('%');
                out.push_str(&format!("{:02X}", byte));
            }
        }
    }
    out
}
