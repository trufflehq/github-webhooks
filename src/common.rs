use github::{Event, EventType};
use ring::hmac::{Key, self};

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
		println!("verify: {:#?}", verify);

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
	use ring::hmac;
	use super::*;

	static SECRET: &'static str = "fgm6bKEBEwZPKnbp+ftJo7FIY0vs4UX9uI0fZrpeDY4=";

	#[test]
	fn test_ping() {
		let webhook = GithubWebhook {
			event_type: "ping".to_string(),
			signature: "b3119bae191e46484412e10814aff545bef55d0d0f5e2fd319da0906fbca008f".to_string(),
			payload: include_bytes!("../events/ping.json").to_vec()
		};

		let key = ring::hmac::Key::new(hmac::HMAC_SHA256, SECRET.as_bytes());

		let event_res = webhook.to_event(&key);
		println!("{:#?}", event_res);
		assert!(event_res.is_ok());
		let event = event_res.unwrap();

		match event {
			Event::Ping(e) => {
				assert_eq!(e.repository.id, 653308803)
			},
			_ => panic!("Expected Ping Event")
		}
	}
}
