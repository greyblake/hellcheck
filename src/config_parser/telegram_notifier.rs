use yaml_rust::yaml::Yaml;

use crate::config::TelegramNotifierConfig;
use crate::error::ConfigError;
use super::common::{Result, parse_key, parse_yaml_to_string};

pub fn parse(id: &str, body: &Yaml) -> Result<TelegramNotifierConfig> {
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
                    }
                    "chat_id" => {
                        let attr_val = parse_yaml_to_string(&attr_yaml_val)?;
                        chat_id_opt = Some(attr_val);
                    }
                    _ => {
                        let e = ConfigError::UnknownNotifierAttribute {
                            notifier_id: id.to_string(),
                            notifier_type: "telegram".to_owned(),
                            attr_name: attr_key,
                        };
                        return Err(e);
                    }
                }
            }
        }
        _ => {
            let message = format!("`notifiers.{}` must be a hash. Got {:?}", id, body);
            return Err(ConfigError::GeneralError { message });
        }
    };

    let token = token_opt.ok_or(ConfigError::FieldMissing {
        path: format!("notifiers.{}.token", id),
    })?;
    let chat_id = chat_id_opt.ok_or(ConfigError::FieldMissing {
        path: format!("notifiers.{}.chat_id", id),
    })?;

    let config = TelegramNotifierConfig { token, chat_id };
    Ok(config)
}
