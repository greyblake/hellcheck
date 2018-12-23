use crate::reactor::State;

mod telegram;
pub use self::telegram::TelegramNotifier;

#[derive(Debug)]
pub struct Notification {
    pub checker_id: String,
    pub checker_url: String,
    pub state: State
}

pub trait Notifier {
    fn notify(&self, notification: &Notification) -> Result<(), ()>;
}
