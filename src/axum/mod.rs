use self::headers::event::X_GITHUB_EVENT;
use self::headers::signature::X_HUB_SIGNATURE_256;
use self::headers::{XGithubEvent, XHubSignature256};
use crate::common::GithubWebhook;
use crate::error::{Result, WebhooksError};
use axum::{TypedHeader, RequestExt};
use axum::{
	body::Bytes,
	extract::FromRequest,
	http::Request,
	response::{IntoResponse, Response},
};
use github::{Event, EventType};
use ring::hmac::{self, Key};

pub mod headers;

#[async_trait::async_trait]
impl<S, B> FromRequest<S, B> for GithubWebhook
where
	S: Send + Sync,
	B: Send + 'static,
	Bytes: FromRequest<S, B>,
{
	type Rejection = Response;

	async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let headers = req
            .headers();
        
        let event_type = headers.get(X_GITHUB_EVENT.clone())
            .and_then(|value| value.to_str().ok())
            .and_then(|s| Some(s.to_string()))
            .ok_or_else(|| WebhooksError::MissingHeader { header: X_GITHUB_EVENT.to_string() }.into_response())?;

        let signature = headers.get(X_HUB_SIGNATURE_256.clone())
            .and_then(|value| value.to_str().ok())
            .and_then(|s| s.get(7..))
			.and_then(|s| Some(s.to_string()))
            .ok_or_else(|| WebhooksError::MissingHeader { header: X_GITHUB_EVENT.to_string() }.into_response())?;

		let payload = Bytes::from_request(req, state)
			.await
			.map_err(IntoResponse::into_response)?
            .to_vec();

		Ok(Self {
			payload,
			signature,
			event_type,
		})
	}
}
