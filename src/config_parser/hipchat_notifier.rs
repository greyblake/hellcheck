use hyper::Uri;
use yaml_rust::yaml::Yaml;

use super::common::{parse_key, parse_yaml_to_string, Result};
use crate::config::HipchatNotifierConfig;
use crate::error::ConfigError;

pub fn parse(id: &str, body: &Yaml) -> Result<HipchatNotifierConfig> {
    let mut token_opt: Option<String> = None;
    let mut room_id_opt: Option<String> = None;
    let mut base_url_opt: Option<Uri> = None;

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
                    "room_id" => {
                        let attr_val = parse_yaml_to_string(&attr_yaml_val)?;
                        room_id_opt = Some(attr_val);
                    }
                    "base_url" => {
                        let val = parse_yaml_to_string(&attr_yaml_val)?;
                        let url: Uri = val.parse().map_err(|_| {
                            let message = format!(
                                "`{}` in `notifiers.{}.base_url` is not a valid URL",
                                val, id
                            );
                            ConfigError::GeneralError { message }
                        })?;
                        base_url_opt = Some(url);
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
    let room_id = room_id_opt.ok_or(ConfigError::FieldMissing {
        path: format!("notifiers.{}.room_id", id),
    })?;
    let base_url = base_url_opt.ok_or(ConfigError::FieldMissing {
        path: format!("notifiers.{}.base_url", id),
    })?;

    let config = HipchatNotifierConfig {
        token,
        room_id,
        base_url,
    };
    Ok(config)
}
