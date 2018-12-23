use std::process::Command;

use crate::notifiers::{Notification, Notifier};
use crate::config::CommandNotifierConfig;
use crate::reactor::State;

pub struct CommandNotifier {
    command: String,
    arguments: Vec<String>
}

impl CommandNotifier {
    pub fn from_config(config: &CommandNotifierConfig) -> Self {
        Self {
            command: config.command.clone(),
            arguments: config.arguments.clone()
        }
    }
}

impl Notifier for CommandNotifier {
    fn notify(&self, notification: &Notification) -> Result<(), ()> {
        let ok = match notification.state {
            State::Up => "true".to_owned(),
            State::Down => "false".to_owned()
        };

        let res = Command::new(&self.command)
            .args(self.arguments.iter())
            .env("HELLCHECK_ID", notification.checker_id.clone())
            .env("HELLCHECK_URL", notification.checker_url.clone())
            .env("HELLCHECK_OK", ok)
            .status();

        match res {
            Ok(status) => {
                if status.success() {
                    Ok(())
                } else {
                    Err(())
                }
            },
            Err(_) => Err(())
        }
    }
}
