use std::collections::HashMap;

use crate::config::TelegramNotifierConfig;
use crate::notifiers::{Notification, Notifier};
use crate::reactor::State;

pub struct TelegramNotifier {
    http_client: ::reqwest::Client,
    token: String,
    chat_id: String,
}

impl TelegramNotifier {
    pub fn from_config(config: &TelegramNotifierConfig) -> Self {
        Self {
            http_client: reqwest::Client::new(),
            token: config.token.clone(),
            chat_id: config.chat_id.clone(),
        }
    }
}

impl Notifier for TelegramNotifier {
    fn notify(&self, notification: &Notification) -> Result<(), ()> {
        let text = match notification.state {
            State::Up => {
                let emoji_baloon = '\u{1F388}';
                format!(
                    "{} is up {}\n{}",
                    notification.checker_id,
                    emoji_baloon,
                    notification.checker_url
                )
            }
            State::Down => {
                let emoji_fire = '\u{1F525}';
                format!(
                    "{} is down {}\n{}",
                    notification.checker_id,
                    emoji_fire,
                    notification.checker_url
                )
            }
        };
        let mut payload = HashMap::new();
        payload.insert("chat_id", self.chat_id.clone());
        payload.insert("text", text);

        let url = format!("https://api.telegram.org/bot{}/sendMessage", self.token);

        let res = self.http_client.post(&url).json(&payload).send();
        res.map(|_| ()).map_err(|_| ())
    }
}
