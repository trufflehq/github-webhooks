// use self::headers::{XGithubEvent, XHubSignature256};
// use crate::error::{Result, ToEventError};
// use axum::TypedHeader;
// use axum::{
// 	body::Bytes,
// 	extract::FromRequest,
// 	http::Request,
// 	response::{IntoResponse, Response},
// };
// use github::{Event, EventType};
// use ring::hmac::{self, Key};

pub mod headers;

// TODO: make public when funcitonal
// struct GithubWebhook {
// 	pub payload: Bytes,
// 	pub signature: String,
// 	pub event_type: String,
// }

// impl GithubWebhook {
// 	/// Verify the signature and parse the payload into an `Event`.
// 	pub fn to_event(&self, key: &Key) -> Result<Event> {
// 		let sha = hex::decode(&self.signature)?;

// 		if hmac::verify(key, &self.payload, &sha).is_ok() {
// 			let event_type = self.event_type.parse::<EventType>().map_err(|_| {
// 				ToEventError::InvalidEventType {
// 					r#type: self.event_type.clone(),
// 				}
// 			})?;

// 			Ok(Event::from_json(event_type, &self.payload)
// 				.map_err(|_| ToEventError::ParseFailed)?)
// 		} else {
// 			Err(ToEventError::VerifySignatureFailed)
// 		}
// 	}
// }

// #[async_trait::async_trait]
// impl<S, B> FromRequest<S, B> for GithubWebhook
// where
// 	S: Send + Sync,
// 	B: Send + 'static,
// 	Bytes: FromRequest<S, B>,
// {
// 	type Rejection = Response;

// 	async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
// 		let TypedHeader(XHubSignature256(signature)) =
// 			TypedHeader::<XHubSignature256>::from_request(req, state)
// 				.await
// 				.map_err(|err| err.into_response())?;

// 		let TypedHeader(XGithubEvent(event_type)) =
// 			TypedHeader::<XGithubEvent>::from_request(req, state)
// 				.await
// 				.map_err(|err| err.into_response())?;

// 		let payload = Bytes::from_request(req, state)
// 			.await
// 			.map_err(IntoResponse::into_response)?;

// 		Ok(Self {
// 			payload,
// 			signature,
// 			event_type,
// 		})
// 	}
// }
