use hyper::Uri;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct FileConfig {
    pub checkers: Vec<CheckerConfig>,
    pub notifiers: Vec<Notifier>,
}

#[derive(Debug, Clone)]
pub struct CheckerConfig {
    pub id: String,
    pub url: Uri,
    pub interval: Duration,
    pub notifiers: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Notifier {
    pub id: String,
    pub config: NotifierConfig,
}

#[derive(Debug, Clone)]
pub enum NotifierConfig {
    Telegram(TelegramNotifierConfig),
    Command(CommandNotifierConfig),
    Hipchat(HipchatNotifierConfig),
}

#[derive(Debug, Clone)]
pub struct TelegramNotifierConfig {
    pub token: String,
    pub chat_id: String,
}

#[derive(Debug, Clone)]
pub struct HipchatNotifierConfig {
    pub base_url: Uri,
    pub token: String,
    pub room_id: String,
}

#[derive(Debug, Clone)]
pub struct CommandNotifierConfig {
    pub command: String,
    pub arguments: Vec<String>,
}

impl FileConfig {
    pub fn get_checker_by_id(&self, id: &str) -> Option<CheckerConfig> {
        self.checkers.iter().find(|c| c.id == id).cloned()
    }

    pub fn get_notifier_by_id(&self, id: &str) -> Option<Notifier> {
        self.notifiers.iter().find(|n| n.id == id).cloned()
    }
}
