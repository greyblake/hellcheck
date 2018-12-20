use std::time::Duration;
use hyper::Uri;

#[derive(Debug, Clone)]
pub struct FileConfig {
    pub checkers: Vec<CheckerConfig>,
    pub notifiers: Vec<Notifier>
}

#[derive(Debug, Clone)]
pub struct TelegramNotifierConfig {
    pub token: String,
    pub chat_id: String
}

#[derive(Debug, Clone)]
pub struct Notifier {
    pub id: String,
    pub config: NotifierConfig,
}

#[derive(Debug, Clone)]
pub enum NotifierConfig {
    Telegram(TelegramNotifierConfig)
}

#[derive(Debug, Clone)]
pub struct CheckerConfig {
    pub id: String,
    pub url: Uri,
    pub interval: Duration,
    pub notifiers: Vec<String>
}
