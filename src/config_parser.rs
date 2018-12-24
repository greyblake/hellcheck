use yaml_rust::{yaml::Yaml, YamlLoader};

use crate::config::{CheckerConfig, FileConfig, Notifier, NotifierConfig};
use crate::error::ConfigError;

mod checker;
mod command_notifier;
mod common;
mod hipchat_notifier;
mod telegram_notifier;

use self::common::{parse_key, parse_yaml_to_hash, parse_yaml_to_string, Result};

pub fn parse_config(yaml: &str) -> Result<FileConfig> {
    let mut checkers = vec![];
    let mut notifiers = vec![];

    let docs = YamlLoader::load_from_str(yaml).map_err(|err| ConfigError::InvalidYaml { err })?;

    for doc in docs.iter() {
        match doc {
            Yaml::Hash(root) => {
                for (yaml_key, val) in root.iter() {
                    let key = parse_key(yaml_key)?;

                    match key.as_ref() {
                        "checkers" => {
                            checkers = parse_checkers(val)?;
                        }
                        "notifiers" => {
                            notifiers = parse_notifiers(val)?;
                        }
                        _ => {
                            return Err(ConfigError::UnkownRootElement { name: key });
                        }
                    };
                }
            }
            _ => {
                return Err(ConfigError::GeneralError {
                    message: "Root element of YAML must be Hash".to_owned(),
                });
            }
        }
    }

    Ok(FileConfig {
        checkers,
        notifiers,
    })
}

fn parse_notifiers(notifier_configs: &Yaml) -> Result<Vec<Notifier>> {
    let mut notifiers = vec![];

    match notifier_configs {
        Yaml::Hash(hash) => {
            for (yaml_key, val) in hash.iter() {
                let checker = parse_notifier(yaml_key, val)?;
                notifiers.push(checker);
            }
        }
        _ => {
            let message = format!("`notifiers` must be a hash. Got {:?}", notifier_configs);
            return Err(ConfigError::GeneralError { message });
        }
    }

    Ok(notifiers)
}

fn parse_notifier(key: &Yaml, body: &Yaml) -> Result<Notifier> {
    let id = parse_key(key)?;
    let config = parse_notifier_config(&id, body)?;

    let notifier = Notifier { id, config };
    Ok(notifier)
}

fn parse_notifier_config(id: &str, body: &Yaml) -> Result<NotifierConfig> {
    let hash = parse_yaml_to_hash(body)?;
    let type_val_yaml =
        hash.get(&Yaml::String("type".to_owned()))
            .ok_or(ConfigError::FieldMissing {
                path: format!("notifiers.{}.type", id),
            })?;

    let type_val = parse_yaml_to_string(type_val_yaml)?;

    match type_val.as_ref() {
        "telegram" => {
            let config = telegram_notifier::parse(id, body)?;
            Ok(NotifierConfig::Telegram(config))
        }
        "command" => {
            let config = command_notifier::parse(id, body)?;
            Ok(NotifierConfig::Command(config))
        }
        "hipchat" => {
            let config = hipchat_notifier::parse(id, body)?;
            Ok(NotifierConfig::Hipchat(config))
        }
        _ => {
            let e = ConfigError::InvalidNotifierType {
                notifier_id: id.to_owned(),
                type_value: type_val,
            };
            Err(e)
        }
    }
}

fn parse_checkers(checker_configs: &Yaml) -> Result<Vec<CheckerConfig>> {
    let mut checkers = vec![];

    match checker_configs {
        Yaml::Hash(hash) => {
            for (yaml_key, val) in hash.iter() {
                let checker = checker::parse(yaml_key, val)?;
                checkers.push(checker);
            }
        }
        _ => {
            let message = format!("`checkers` must be a hash. Got {:?}", checker_configs);
            return Err(ConfigError::GeneralError { message });
        }
    }

    Ok(checkers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_yaml() {
        let yaml = r#"
            checkers:
              greyblake:
                url: https://www.greyblake.com/
                interval: 1s
                notifiers: [telebot]

            notifiers:
              telebot:
                type: telegram
                token: TOKENGOESHERE
                chat_id: 8677112
        "#;
        let config = parse_config(yaml).unwrap();
        assert_eq!(config.checkers.len(), 1);
        assert_eq!(config.notifiers.len(), 1);
    }

    #[test]
    fn test_token_is_missing_for_telegram_notifier() {
        let yaml = r#"
            checkers:
              greyblake:
                url: https://www.greyblake.com/
                interval: 1s
                notifiers: [telebot]

            notifiers:
              telebot:
                type: telegram
                chat_id: 8677112
        "#;
        let err = parse_config(yaml).unwrap_err();
        assert_eq!(
            err,
            ConfigError::FieldMissing {
                path: "notifiers.telebot.token".to_owned()
            }
        )
    }

    #[test]
    fn test_checker_has_no_url() {
        let yaml = r#"
            checkers:
              greyblake:
                interval: 1s
                notifiers: [telebot]

            notifiers:
              telebot:
                type: telegram
                token: TOKEN
                chat_id: 8677112
        "#;
        let err = parse_config(yaml).unwrap_err();
        assert_eq!(
            err,
            ConfigError::FieldMissing {
                path: "checkers.greyblake.url".to_owned()
            }
        )
    }
}
