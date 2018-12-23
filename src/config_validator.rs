// Validates FileConfig for inconsistencies.

use crate::config::FileConfig;
use crate::error::ConfigValidationError;

type Result<T> = ::std::result::Result<T, ConfigValidationError>;

pub fn validate_config(config: &FileConfig) -> Result<Vec<String>> {
    verify_checker_notifiers(config)?;

    let mut warnings: Vec<String> = vec![];
    verify_empty_notifiers(config, &mut warnings);

    Ok(warnings)
}


// Ensure, that all checkers refer to declared notifiers.
fn verify_checker_notifiers(config: &FileConfig) -> Result<()> {
    for checker in config.checkers.iter() {
        for notifier_id in checker.notifiers.iter() {
            match config.get_notifier_by_id(notifier_id) {
                Some(_) => {},
                None => {
                    let err = ConfigValidationError::UnknownNotifier {
                        checker_id: checker.id.to_owned(),
                        notifier_id: notifier_id.to_owned()
                    };
                    return Err(err)
                }
            }
        }
    }

    Ok(())
}

fn verify_empty_notifiers(config: &FileConfig, warnings: &mut Vec<String>) {
    if config.notifiers.is_empty() {
        warnings.push("Notifiers are not declared. You will not get notifications.".to_owned());
        return;
    }

    for checker in config.checkers.iter() {
        if checker.notifiers.is_empty() {
            let msg = format!("`checkers.{}.notifiers` is empty. You will not get notifications", checker.id);
            warnings.push(msg);
        }
    }
}
