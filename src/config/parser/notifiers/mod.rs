use yaml_rust::yaml::Yaml;

use crate::config::{Notifier, NotifierConfig};
use crate::error::ConfigError;

use super::common;
use super::common::{parse_key, parse_yaml_to_hash, parse_yaml_to_string, Result};

mod command;
mod hipchat;
mod slack;
mod telegram;

pub fn parse_notifiers(notifier_configs: &Yaml) -> Result<Vec<Notifier>> {
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
            let config = telegram::parse(id, body)?;
            Ok(NotifierConfig::Telegram(config))
        }
        "command" => {
            let config = command::parse(id, body)?;
            Ok(NotifierConfig::Command(config))
        }
        "hipchat" => {
            let config = hipchat::parse(id, body)?;
            Ok(NotifierConfig::Hipchat(config))
        }
        "slack" => {
            let config = slack::parse(id, body)?;
            Ok(NotifierConfig::Slack(config))
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
