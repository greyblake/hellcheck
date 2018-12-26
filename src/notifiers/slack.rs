use std::collections::HashMap;

use hyper::Uri;

use crate::config::SlackNotifierConfig;
use crate::notifiers::{Notification, Notifier};
use crate::reactor::State;

pub struct SlackNotifier {
    http_client: ::reqwest::Client,
    webhook_url: Uri,
}

impl SlackNotifier {
    pub fn from_config(config: &SlackNotifierConfig) -> Self {
        Self {
            http_client: reqwest::Client::new(),
            webhook_url: config.webhook_url.clone(),
        }
    }
}

impl Notifier for SlackNotifier {
    fn notify(&self, notification: &Notification) -> Result<(), ()> {
        let payload = build_payload(notification);
        let url = format!("{}", self.webhook_url);
        let res = self.http_client.post(&url).json(&payload).send();
        res.map(|_| ()).map_err(|_| ())
    }
}

fn build_payload(
    notification: &Notification,
) -> HashMap<&'static str, Vec<HashMap<&'static str, String>>> {
    let fallback = match notification.state {
        State::Up => format!(
            "{} is up :thumbsup:\n{}",
            notification.checker_id, notification.checker_url
        ),
        State::Down => format!(
            "{} is down :fire:\n{}",
            notification.checker_id, notification.checker_url
        ),
    };
    let color = match notification.state {
        State::Up => "good".to_owned(),
        State::Down => "danger".to_owned(),
    };
    let title = match notification.state {
        State::Up => format!("{} is up :thumbsup:", notification.checker_id),
        State::Down => format!("{} is down :fire:", notification.checker_id),
    };
    let title_link = notification.checker_url.clone();

    let mut attachment = HashMap::new();
    attachment.insert("fallback", fallback);
    attachment.insert("color", color);
    attachment.insert("title", title);
    attachment.insert("title_link", title_link);

    let attachments = vec![attachment];

    let mut payload = HashMap::new();
    payload.insert("attachments", attachments);

    payload
}
