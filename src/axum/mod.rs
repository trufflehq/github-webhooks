use self::headers::event::X_GITHUB_EVENT;
use self::headers::signature::X_HUB_SIGNATURE_256;
use crate::common::GithubWebhook;
use crate::error::{Result, WebhooksError};
use axum::{
	body::Bytes,
	extract::FromRequest,
	http::Request,
	response::{IntoResponse, Response},
};

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
		let headers = req.headers();

		let event_type = headers
			.get(X_GITHUB_EVENT.clone())
			.and_then(|value| value.to_str().ok())
			.map(|s| s.to_string())
			.ok_or_else(|| {
				WebhooksError::MissingHeader {
					header: X_GITHUB_EVENT.to_string(),
				}
				.into_response()
			})?;

		let signature = headers
			.get(X_HUB_SIGNATURE_256.clone())
			.and_then(|value| value.to_str().ok())
			.and_then(|s| s.get(7..))
			.map(|s| s.to_string())
			.ok_or_else(|| {
				WebhooksError::MissingHeader {
					header: X_GITHUB_EVENT.to_string(),
				}
				.into_response()
			})?;

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

// #[cfg(test)]
// mod test {
// 	use super::*;
// 	use axum::{routing::post, Router};
// 	use once_cell::sync::Lazy;
// 	use std::{net::SocketAddr, str::FromStr};

// 	static CLIENT: Lazy<reqwest::Client> = Lazy::new(|| {
// 		reqwest::Client::builder()
// 			.redirect(reqwest::redirect::Policy::none())
// 			.build()
// 			.unwrap()
// 	});

// 	#[tokio::test]
// 	async fn test_axum_extractor() {
// 		println!("here!");
// 		async fn handler(webhook: GithubWebhook) -> impl IntoResponse {
// 			format!("event_type={:?}", webhook.event_type)
// 		}

// 		let router: Router = Router::new().route("/", post(handler));
// 		let addr = SocketAddr::from_str("172.0.0.1:8845").unwrap();
// 		println!("here!");
// 		let handle = tokio::spawn(async move {
// 			axum::Server::bind(&addr)
// 				.serve(router.into_make_service())
// 				.await
// 				.expect("server failed");
// 		});
// 		println!("here!");

// 		let body = include_bytes!("../../test/events/ping.json").to_vec();
// 		let signature = include_str!("../../test/events/ping_signature.txt");
// 		let signature = format!("sha256={}", signature);

// 		// create a Reqwest request for `POST` `/` with the body as `body`,
// 		// and the header X_HUB_SIGNATURE_256 as `signature`
// 		// and the header X_GITHUB_EVENT as `ping`
// 		let req = CLIENT
// 			.post(format!("http://{}/", addr))
// 			.header(X_HUB_SIGNATURE_256.to_string(), signature)
// 			.header(X_GITHUB_EVENT.to_string(), "ping")
// 			.body(body);
// 		println!("here!");

// 		let res = req.send().await.unwrap();
// 		let text = res.text().await.unwrap();

// 		assert_eq!(text, "event_type=ping");

// 		handle.abort();
// 	}
// }
