use hyper::Uri;

use std::collections::HashMap;

use crate::config::HipchatNotifierConfig;
use crate::notifiers::{Notification, Notifier};
use crate::reactor::State;

pub struct HipchatNotifier {
    http_client: ::reqwest::Client,
    base_url: Uri,
    token: String,
    room_id: String
}

impl HipchatNotifier {
    pub fn from_config(config: &HipchatNotifierConfig) -> Self {
        Self {
            http_client: reqwest::Client::new(),
            base_url: config.base_url.clone(),
            token: config.token.clone(),
            room_id: config.room_id.clone()
        }
    }
}

impl Notifier for HipchatNotifier {
    fn notify(&self, notification: &Notification) -> Result<(), ()> {
        let payload = build_payload(notification);
        let url = format!("{}/v2/room/{}/notification?auth_token={}", self.base_url, self.room_id, self.token);

        let res = self.http_client.post(&url).json(&payload).send();
        res.map(|_| ()).map_err(|_| ())
    }
}

fn build_payload(notification: &Notification) -> HashMap<&'static str, String> {
    let color = match notification.state {
        State::Up => "green".to_owned(),
        State::Down => "red".to_owned()
    };

    let message = match notification.state {
        State::Up => format!("{} is up (dealwithit)\n{}", notification.checker_id, notification.checker_url),
        State::Down => format!("{} is down (boom)\n{}", notification.checker_id, notification.checker_url),
    };

    let mut payload = HashMap::new();
    payload.insert("color", color);
    payload.insert("message", message);
    payload.insert("message_format", "text".to_owned());

    payload
}

