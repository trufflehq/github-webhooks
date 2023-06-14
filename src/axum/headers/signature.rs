use headers::{Header, HeaderName, HeaderValue};

/// The `X-Hub-Signature-256` header.
pub static X_HUB_SIGNATURE_256: HeaderName = HeaderName::from_static("x-hub-signature-256");

/// An axum-style `TypedHeader` for the `X-Hub-Signature-256` header.
/// Example:
/// ```rs,no_run
/// fn handle(
///     TypedHeader(XHubSignature256(signature)): TypedHeader<XHubSignature256>,
/// ) -> impl IntoResponse {
///     // ...
/// }
/// ```
pub struct XHubSignature256(pub String);

impl Header for XHubSignature256 {
	fn name() -> &'static HeaderName {
		&X_HUB_SIGNATURE_256
	}

	fn decode<'i, I>(values: &mut I) -> Result<Self, headers::Error>
	where
		I: Iterator<Item = &'i HeaderValue>,
	{
		let value = values
			.next()
			.and_then(|h| HeaderValue::to_str(h).ok())
			.and_then(|s| s.get(7..))
			.map(|s| s.to_string())
			.ok_or_else(headers::Error::invalid)?;

		Ok(Self(value))
	}

	fn encode<E>(&self, _values: &mut E)
	where
		E: Extend<HeaderValue>,
	{
		// unnecessary, since we're only decoding
		unreachable!()
	}
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use axum::{response::IntoResponse, routing::get, Router, TypedHeader};
//     use reqwest::Client;

//     #[tokio::test]
//     async fn typed_header() {
//         async fn handle(
//             TypedHeader(signature): TypedHeader<XHubSignature256>,
//         ) -> impl IntoResponse {
//             let signature = signature.0.as_str();
//             format!("X-Hub-Signature-256={signature:?}")
//         }

//         let app = Router::new().route("/", get(handle));

//         let handle = tokio::spawn(async move {
//             axum::Server::bind(&"127.0.0.1:11632".parse().unwrap())
//                 .serve(app.into_make_service())
//                 .await
//                 .unwrap();
//         });
//         let client = Client::new();

//         let res = client
//             .get("http://127.0.0.1:11632/")
//             .header("x-hub-signature-256", "sha256=foobar")
//             .send()
//             .await.unwrap();
//         let body = res.text().await.unwrap();
//         assert_eq!(
//             body,
//             r#"X-Hub-Signature-256=sha256=foobar"#
//         );

//         handle.abort();
//     }
// }
