use headers::{Header, HeaderName, HeaderValue};

/// The `X-Github-Delivery` header.
pub static X_GITHUB_DELIVERY: HeaderName = HeaderName::from_static("x-github-delivery");

/// An axum-style `TypedHeader` for the `X-Github-Delivery` header.
/// Example:
/// ```rs,no_run
/// fn handle(
///     TypedHeader(XGithubDelivery(delivery)): TypedHeader<XGithubDelivery>,
/// ) -> impl IntoResponse {
///     // ...
/// }
/// ```
pub struct XGithubDelivery(pub String);

impl Header for XGithubDelivery {
	fn name() -> &'static HeaderName {
		&X_GITHUB_DELIVERY
	}

	fn decode<'i, I>(values: &mut I) -> Result<Self, headers::Error>
	where
		I: Iterator<Item = &'i HeaderValue>,
	{
		let value = values
			.next()
			.and_then(|h| HeaderValue::to_str(h).ok())
			.and_then(|s| Some(s.to_string()))
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
