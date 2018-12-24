use yaml_rust::yaml::Yaml;

use crate::config::CommandNotifierConfig;
use crate::error::ConfigError;
use super::common::{Result, parse_key, parse_yaml_to_vec};

pub fn parse(id: &str, body: &Yaml) -> Result<CommandNotifierConfig> {
    let mut config_opt: Option<CommandNotifierConfig> = None;

    match body {
        Yaml::Hash(hash) => {
            for (attr_yaml_key, attr_yaml_val) in hash {
                let attr_key = parse_key(&attr_yaml_key)?;

                match attr_key.as_ref() {
                    "type" => (),
                    "command" => match parse_yaml_to_vec(&attr_yaml_val) {
                        Ok(vals) => {
                            if let Some((command, arguments)) = vals.split_first() {
                                let config = CommandNotifierConfig {
                                    command: command.clone(),
                                    arguments: arguments.to_vec(),
                                };
                                config_opt = Some(config);
                            } else {
                                let message = format!(
                                    "`notifiers.{}.command` must have a command specified",
                                    id
                                );
                                return Err(ConfigError::GeneralError { message });
                            }
                        }
                        Err(_) => {
                            let message = format!("`notifiers.{}.command` must be an array.", id);
                            return Err(ConfigError::GeneralError { message });
                        }
                    },
                    _ => {
                        let e = ConfigError::UnknownNotifierAttribute {
                            notifier_id: id.to_string(),
                            notifier_type: "command".to_owned(),
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

    let config = config_opt.ok_or(ConfigError::FieldMissing {
        path: format!("notifiers.{}.command", id),
    })?;
    Ok(config)
}
