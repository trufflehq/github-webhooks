pub type Result<T, E = ToEventError> = std::result::Result<T, E>;

/// Errors that can occur when converting a `GithubWebhook` into an `Event`.
#[derive(thiserror::Error, Debug)]
pub enum ToEventError {
	/// The `X-Hub-Signature-256` header is invalid &mdash; it couldn't be decoded.
	#[error("X-Hub-Signature-256 header is invalid")]
	InvalidSignature(#[from] hex::FromHexError),

	/// When parsing the `X-Github-Event` header, into an `EventType`, it fails.
	#[error("Failed to parse payload, type: {r#type}")]
	InvalidEventType { r#type: String },

	/// When parsing the payload into an `Event`, it fails.
	#[error("Failed to parse the payload into `github::Event`")]
	ParseFailed,

	/// When verifying the signature, it fails.
	#[error("Failed to verify the signature")]
	VerifySignatureFailed,
}
