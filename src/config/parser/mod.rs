use yaml_rust::{yaml::Yaml, YamlLoader};

use crate::config::FileConfig;
use crate::error::ConfigError;

mod checkers;
mod common;
mod notifiers;

use self::common::{parse_key, Result};

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
                            checkers = checkers::parse_checkers(val)?;
                        }
                        "notifiers" => {
                            notifiers = notifiers::parse_notifiers(val)?;
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
