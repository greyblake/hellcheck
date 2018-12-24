use hyper::Uri;

use crate::config::HipchatNotifierConfig;
use crate::notifiers::{Notification, Notifier};
use crate::reactor::State;

pub struct HipchatNotifier {
    http_client: ::reqwest::Client,
    base_url: Uri,
    auth_token: String,
    room_id: String
}

impl HipchatNotifier {
    pub fn from_config(config: &HipchatNotifierConfig) -> Self {
        Self {
            http_client: reqwest::Client::new(),
            base_url: config.base_url.clone(),
            auth_token: config.auth_token.clone(),
            room_id: config.room_id.clone()
        }
    }
}

impl Notifier for HipchatNotifier {
    fn notify(&self, _notification: &Notification) -> Result<(), ()> {
        println!("TODO: send notification to HipChat");
        Ok(())
    }
}
