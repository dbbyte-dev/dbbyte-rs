use std::fmt;

/// The DBByte API returns `{"error": string, "hint"?: string}` on failure.
/// There is no `code` field in the current API - do not rely on one.
///
/// A 429 can mean two different things and they are NOT both retryable:
/// - Burst rate limit: `retry_after` is `Some(_)`, transient, safe to retry.
/// - Plan API request quota exceeded: `retry_after` is `None`, retrying
///   does nothing until the billing period resets or the plan is upgraded.
///
/// Check `retry_after` rather than the message text to tell them apart.
#[derive(Debug, Clone)]
pub struct DBByteError {
    pub message: String,
    pub status: u16,
    pub hint: Option<String>,
    pub retry_after: Option<u64>,
}

impl DBByteError {
    pub(crate) fn new(
        message: impl Into<String>,
        status: u16,
        hint: Option<String>,
        retry_after: Option<u64>,
    ) -> Self {
        Self {
            message: message.into(),
            status,
            hint,
            retry_after,
        }
    }

    pub(crate) fn transport(message: impl Into<String>) -> Self {
        Self::new(message, 0, None, None)
    }
}

impl fmt::Display for DBByteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.hint {
            Some(hint) => write!(
                f,
                "dbbyte: {} (status {}, hint: {})",
                self.message, self.status, hint
            ),
            None => write!(f, "dbbyte: {} (status {})", self.message, self.status),
        }
    }
}

impl std::error::Error for DBByteError {}
