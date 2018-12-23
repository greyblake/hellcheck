use std::time::Duration;

use hyper::Uri;
use yaml_rust::{YamlLoader, yaml::Yaml};

use crate::config::{FileConfig, CheckerConfig, NotifierConfig, Notifier, TelegramNotifierConfig};
use crate::error::ConfigError;

type Result<T> = std::result::Result<T, ConfigError>;

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
                        },
                        "notifiers" => {
                            notifiers = parse_notifiers(val)?;
                        },
                        _ => {
                            return Err(ConfigError::UnkownRootElement { name: key });
                        }
                    };
                }
            },
            _ => {
                return Err(ConfigError::GeneralError { message: "Root element of YAML must be Hash".to_owned() });
            }
        }
    }


    Ok(FileConfig { checkers, notifiers })
}

fn parse_key(key: &Yaml) -> Result<String> {
    match key {
        Yaml::String(s) => Ok(s.to_owned()),
        Yaml::Integer(num) => Ok(num.to_string()),
        _ => {
            let message = format!("Key must be a string. Got {:?}", key);
            Err(ConfigError::GeneralError { message: message})
        }
    }
}

fn parse_yaml_to_string(val: &Yaml) -> Result<String> {
    match val {
        Yaml::String(s) => Ok(s.to_owned()),
        Yaml::Integer(num) => Ok(num.to_string()),
        _ => {
            let message = format!("Key must be a string. Got {:?}", val);
            Err(ConfigError::GeneralError { message: message})
        }
    }
}

fn parse_yaml_to_vec(val: &Yaml) -> Result<Vec<String>> {
    let mut items: Vec<String> = vec![];

    match val {
        Yaml::Array(arr) => {
            for yaml_item in arr.iter() {
                let item = parse_yaml_to_string(yaml_item)?;
                items.push(item);
            }
        },
        _ => {
            let message = format!("Value must be an array. Got {:?}", val);
            return Err(ConfigError::GeneralError { message: message});
        }
    }

    Ok(items)
}

fn parse_yaml_to_hash(val: &Yaml) -> Result<&yaml_rust::yaml::Hash> {
    match val {
        Yaml::Hash(hash) => Ok(hash),
        _ => {
            let message = format!("Value must be a hash. Got {:?}", val);
            return Err(ConfigError::GeneralError { message: message});
        }
    }
}

fn parse_notifiers(notifier_configs: &Yaml) -> Result<Vec<Notifier>> {
    let mut notifiers = vec![];

    match notifier_configs {
        Yaml::Hash(hash) => {
            for (yaml_key, val) in hash.iter() {
                let checker = parse_notifier(yaml_key, val)?;
                notifiers.push(checker);
            }
        },
        _ => {
            let message = format!("`notifiers` must be a hash. Got {:?}", notifier_configs);
            return Err(ConfigError::GeneralError { message: message});
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
    let type_val_yaml = hash.get(&Yaml::String("type".to_owned()))
        .ok_or(ConfigError::FieldMissing { path: format!("notifiers.{}.type", id) } )?;

    let type_val = parse_yaml_to_string(type_val_yaml)?;

    match type_val.as_ref() {
        "telegram" => {
            let config = parse_telegram_notifier_config(id, body)?;
            Ok(NotifierConfig::Telegram(config))
        },
        _ => {
            let e = ConfigError::InvalidNotifierType { notifier_id: id.to_owned(), type_value: type_val };
            Err(e)
        }
    }
}

fn parse_telegram_notifier_config(id: &str, body: &Yaml) -> Result<TelegramNotifierConfig> {
    let mut token_opt: Option<String> = None;
    let mut chat_id_opt: Option<String> = None;

    match body {
        Yaml::Hash(hash) => {
            for (attr_yaml_key, attr_yaml_val) in hash {
                let attr_key = parse_key(&attr_yaml_key)?;

                match attr_key.as_ref() {
                    "type" => (),
                    "token" => {
                        let attr_val = parse_yaml_to_string(&attr_yaml_val)?;
                        token_opt = Some(attr_val);
                    },
                    "chat_id" => {
                        let attr_val = parse_yaml_to_string(&attr_yaml_val)?;
                        chat_id_opt = Some(attr_val);
                    },
                    _ => {
                        let e = ConfigError::UnknownNotifierAttribute {
                            notifier_id: id.to_string(),
                            notifier_type: "telegram".to_owned(),
                            attr_name: attr_key
                        };
                        return Err(e);
                    }
                }
            }
        },
        _ => {
            let message = format!("`notifiers.{}` must be a hash. Got {:?}", id, body);
            return Err(ConfigError::GeneralError { message: message });
        }
    };

    let token = token_opt.ok_or(ConfigError::FieldMissing { path: format!("notifiers.{}.token", id) } )?;
    let chat_id = chat_id_opt.ok_or(ConfigError::FieldMissing { path: format!("notifiers.{}.chat_id", id) } )?;

    let config = TelegramNotifierConfig { token, chat_id };
    Ok(config)
}



fn parse_checkers(checker_configs: &Yaml) -> Result<Vec<CheckerConfig>> {
    let mut checkers = vec![];

    match checker_configs {
        Yaml::Hash(hash) => {
            for (yaml_key, val) in hash.iter() {
                let checker = parse_checker(yaml_key, val)?;
                checkers.push(checker);
            }
        },
        _ => {
            let message = format!("`checkers` must be a hash. Got {:?}", checker_configs);
            return Err(ConfigError::GeneralError { message: message });
        }
    }

    Ok(checkers)
}


fn parse_checker(key: &Yaml, body: &Yaml) -> Result<CheckerConfig> {
    let id = parse_key(key)?;
    let mut notifiers: Vec<String> = vec![];

    // Default interval is 10 sec
    let mut interval = Duration::new(10, 0);
    let mut url_opt: Option<Uri> = None;

    match body {
        Yaml::Hash(hash) => {
            for (attr_yaml_key, attr_yaml_val) in hash {
                let attr_key = parse_key(&attr_yaml_key)?;

                match attr_key.as_ref() {
                    "interval" => {
                        let attr_val = parse_key(&attr_yaml_val)?;
                        match attr_val.parse::<humantime::Duration>() {
                            Ok(val) => {
                                interval = val.into();
                            },
                            Err(_) => {
                                let e = ConfigError::InvalidCheckerInterval { interval: attr_val, checker_id: id };
                                return Err(e);
                            }
                        }
                    },
                    "url" => {
                        let attr_val = parse_yaml_to_string(&attr_yaml_val)?;
                        match attr_val.parse::<Uri>() {
                            Ok(url) => {
                                url_opt = Some(url);
                            },
                            Err(_) => {
                                let e = ConfigError::InvalidCheckerUrl { checker_id: id, url: attr_val };
                                return Err(e);
                            }
                        };
                    },
                    "notifiers" => {
                        notifiers = parse_yaml_to_vec(&attr_yaml_val)?;
                    }
                    _ => {
                        let err = ConfigError::UnknownCheckerAttribute { checker_id: id.clone(), attr_name: attr_key };
                        return Err(err);

                    }
                }
            }
        },
        _ => {
            let message = format!("`checkers.{}` must be a hash. Got {:?}", id, body);
            return Err(ConfigError::GeneralError { message: message });
        }
    };


    let url = url_opt.ok_or(ConfigError::FieldMissing { path: format!("checkers.{}.url", id) } )?;

    let cf = CheckerConfig { id, interval, url, notifiers };
    Ok(cf)
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
        assert_eq!(err, ConfigError::FieldMissing { path: "notifiers.telebot.token".to_owned() } )
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
        assert_eq!(err, ConfigError::FieldMissing { path: "checkers.greyblake.url".to_owned() } )
    }
}
