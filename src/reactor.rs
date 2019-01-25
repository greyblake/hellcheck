use std::collections::HashMap;
use std::sync::mpsc;

use crate::config::{CheckerConfig, FileConfig, NotifierConfig};
use crate::notifiers::Notifier as NotifierTrait;
use crate::notifiers::{
    CommandNotifier, HipchatNotifier, Notification, SlackNotifier, TelegramNotifier,
};

#[derive(Debug, Clone, PartialEq)]
pub enum State {
    Up,
    Down,
}

#[derive(Debug)]
pub struct StateMessage {
    pub checker_id: String,
    pub state: State,
}

pub fn spawn(receiver: mpsc::Receiver<StateMessage>, config: FileConfig) {
    ::std::thread::spawn(move || {
        let mut states = build_initial_states(&config);
        let notifiers = build_notifiers(&config);

        loop {
            let msg = receiver.recv().unwrap();
            let checker = config.get_checker_by_id(&msg.checker_id).unwrap();

            // unwrap is safe here, because `states` was initialized with all possible checker ids.
            let prev_state = &states[&msg.checker_id];

            // Send a message if state was changed
            if msg.state != *prev_state {
                for notifier_id in checker.notifiers.iter() {
                    // unwrap is safe here, because notifiers were validated by config_validator.
                    let notifier = &notifiers[notifier_id];
                    let notification = build_notification(&checker, msg.state.clone());
                    info!("Sending a notification to {}", notifier_id);
                    let res = notifier.notify(&notification);
                    match res {
                        Ok(_) => {}
                        Err(_) => {
                            eprintln!(
                                "ERROR: Notifier `{}` failed to notify that {} is {:?}",
                                notifier_id, checker.id, msg.state
                            );
                        }
                    }
                }
            }

            states.insert(msg.checker_id, msg.state);
        }
    });
}

fn build_notification(checker: &CheckerConfig, state: State) -> Notification {
    Notification {
        checker_id: checker.id.clone(),
        checker_url: format!("{}", checker.url),
        state,
    }
}

fn build_initial_states(config: &FileConfig) -> HashMap<String, State> {
    let mut states: HashMap<String, State> = HashMap::new();
    for checker in config.checkers.iter() {
        states.insert(checker.id.clone(), State::Up);
    }
    states
}

fn build_notifiers(config: &FileConfig) -> HashMap<String, Box<NotifierTrait>> {
    let mut notifiers: HashMap<String, Box<NotifierTrait>> = HashMap::new();
    for notifier_config in config.notifiers.iter() {
        let notifier: Box<NotifierTrait> = match &notifier_config.config {
            NotifierConfig::Telegram(telegram_config) => {
                Box::new(TelegramNotifier::from_config(telegram_config))
            }
            NotifierConfig::Command(command_config) => {
                Box::new(CommandNotifier::from_config(command_config))
            }
            NotifierConfig::Hipchat(hipchat_config) => {
                Box::new(HipchatNotifier::from_config(hipchat_config))
            }
            NotifierConfig::Slack(slack_config) => {
                Box::new(SlackNotifier::from_config(slack_config))
            }
        };
        notifiers.insert(notifier_config.id.clone(), notifier);
    }

    notifiers
}
