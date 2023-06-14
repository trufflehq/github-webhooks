use github::{Event, EventType};
use ring::hmac::{self, Key};

use crate::error::{Result, WebhooksError};

pub struct GithubWebhook {
	pub payload: Vec<u8>,
	pub signature: String,
	pub event_type: String,
}

impl GithubWebhook {
	/// Verify the signature and parse the payload into an `Event`.
	pub fn to_event(&self, key: &Key) -> Result<Event> {
		let sha = hex::decode(&self.signature)?;

		let verify = hmac::verify(key, &self.payload, &sha);

		if verify.is_ok() {
			let event_type = self.event_type.parse::<EventType>().map_err(|_| {
				WebhooksError::InvalidEventType {
					r#type: self.event_type.clone(),
				}
			})?;

			Ok(Event::from_json(event_type, &self.payload)
				.map_err(|_| WebhooksError::ParseFailed)?)
		} else {
			Err(WebhooksError::VerifySignatureFailed)
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use ring::hmac;

	static SECRET: &'static str = include_str!("../test/secret.txt");

	#[test]
	fn test_ping() {
		let webhook = GithubWebhook {
			event_type: "ping".to_string(),
			signature: include_str!("../test/events/ping_signature.txt").to_string(),
			payload: include_bytes!("../test/events/ping.json").to_vec(),
		};

		let key = ring::hmac::Key::new(hmac::HMAC_SHA256, SECRET.as_bytes());

		let event = match webhook.to_event(&key) {
			Ok(e) => e,
			Err(e) => panic!("Error: {:#?}", e),
		};

		match event {
			Event::Ping(e) => {
				assert!(e.zen.is_ascii())
			}
			_ => panic!("Expected Ping Event"),
		}
	}
}
