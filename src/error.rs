use std::fmt::Debug;

use axum::response::{IntoResponse, Response};
use http::StatusCode;

pub type Result<T, E = WebhooksError> = std::result::Result<T, E>;

/// Errors that can occur when converting a `GithubWebhook` into an `Event`.
#[derive(thiserror::Error, Debug)]
pub enum WebhooksError {
	/// The `X-Hub-Signature-256` header is invalid &mdash; it couldn't be decoded.
	#[error("X-Hub-Signature-256 header is invalid")]
	InvalidSignature(#[from] hex::FromHexError),

	/// When parsing the `X-Github-Event` header, into an `EventType`, it fails.
	#[error("Failed to parse payload, type: {r#type:?}")]
	InvalidEventType { r#type: String },

	/// When parsing the payload into an `Event`, it fails.
	#[error("Failed to parse the payload into `github::Event`")]
	ParseFailed,

	/// When verifying the signature, it fails.
	#[error("Failed to verify the signature")]
	VerifySignatureFailed,

	/// A header is missing
	#[error("The header, {header:?} is missing")]
	MissingHeader { header: String },
}

impl WebhooksError {
	fn status_code(&self) -> StatusCode {
		match self {
			Self::InvalidSignature { .. } | Self::VerifySignatureFailed => StatusCode::UNAUTHORIZED,
			Self::InvalidEventType { .. } | Self::MissingHeader { .. } | Self::ParseFailed => {
				StatusCode::BAD_REQUEST
			}
		}
	}
}

/// Axum allows you to return `Result` from handler functions, but the error type
/// also must be some sort of response type.
///
/// By default, the generated `Display` impl is used to return a plaintext error message
/// to the client.
impl IntoResponse for WebhooksError {
	fn into_response(self) -> Response {
		match self {
			Self::InvalidSignature(ref e) => {
				return (
                    self.status_code(),
                    format!("{:?}", e)
                ).into_response()
			}
			// Other errors get mapped normally.
			_ => (),
		}

		(self.status_code(), self.to_string()).into_response()
	}
}
