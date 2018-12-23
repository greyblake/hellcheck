// Validates FileConfig for inconsistencies.

use crate::config::{FileConfig, NotifierConfig};
use crate::error::ConfigValidationError;

type Result<T> = ::std::result::Result<T, ConfigValidationError>;

pub fn validate_config(config: &FileConfig) -> Result<Vec<String>> {
    verify_checker_notifiers(config)?;
    verify_command_notifiers(config)?;

    let mut warnings: Vec<String> = vec![];
    verify_empty_notifiers(config, &mut warnings);
    verify_unused_notifiers(config, &mut warnings);

    Ok(warnings)
}

// Ensure, that all checkers refer to declared notifiers.
fn verify_checker_notifiers(config: &FileConfig) -> Result<()> {
    for checker in config.checkers.iter() {
        for notifier_id in checker.notifiers.iter() {
            match config.get_notifier_by_id(notifier_id) {
                Some(_) => {}
                None => {
                    let err = ConfigValidationError::UnknownNotifier {
                        checker_id: checker.id.to_owned(),
                        notifier_id: notifier_id.to_owned(),
                    };
                    return Err(err);
                }
            }
        }
    }

    Ok(())
}

// Ensure all CommandNotifier refers to an existing command
fn verify_command_notifiers(config: &FileConfig) -> Result<()> {
    for notifier in config.notifiers.iter() {
        match &notifier.config {
            NotifierConfig::Command(c) => {
                if !command_exists(&c.command) {
                    let err = ConfigValidationError::CommandNotFound {
                        notifier_id: notifier.id.clone(),
                        command: c.command.clone(),
                    };
                    return Err(err);
                }
            }
            _ => {}
        }
    }
    Ok(())
}

fn command_exists(command: &str) -> bool {
    let res = std::process::Command::new("which").arg(command).output();

    match res {
        Ok(output) => output.status.success(),
        // if `which` command does not exist on the current system, we just return true
        Err(_) => true,
    }
}

fn verify_empty_notifiers(config: &FileConfig, warnings: &mut Vec<String>) {
    if config.notifiers.is_empty() {
        warnings.push("Notifiers are not declared. You will not get notifications.".to_owned());
        return;
    }

    for checker in config.checkers.iter() {
        if checker.notifiers.is_empty() {
            let msg = format!(
                "`checkers.{}.notifiers` is empty. You will not get notifications",
                checker.id
            );
            warnings.push(msg);
        }
    }
}

fn verify_unused_notifiers(config: &FileConfig, warnings: &mut Vec<String>) {
    let used_notifier_ids: Vec<&String> = config
        .checkers
        .iter()
        .map(|c| c.notifiers.iter())
        .flatten()
        .collect();

    for notifier in config.notifiers.iter() {
        if !used_notifier_ids.contains(&&notifier.id) {
            let msg = format!(
                "Notifier `{}` is not used by any of the checkers.",
                notifier.id
            );
            warnings.push(msg);
        }
    }
}
