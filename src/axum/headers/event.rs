use headers::{Header, HeaderName, HeaderValue};

/// The `X-Github-Event` header.
pub static X_GITHUB_EVENT: HeaderName = HeaderName::from_static("x-github-event");

/// An axum-style `TypedHeader` for the `X-Github-Event` header.
/// Example:
/// ```rs,no_run
/// fn handle(
///     TypedHeader(XGithubEvent(event)): TypedHeader<XGithubEvent>,
/// ) -> impl IntoResponse {
///     // ...
/// }
/// ```
pub struct XGithubEvent(pub String);

impl Header for XGithubEvent {
	fn name() -> &'static HeaderName {
		&X_GITHUB_EVENT
	}

	fn decode<'i, I>(values: &mut I) -> Result<Self, headers::Error>
	where
		I: Iterator<Item = &'i HeaderValue>,
	{
		let value = values
			.next()
			.and_then(|h| HeaderValue::to_str(h).ok())
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
