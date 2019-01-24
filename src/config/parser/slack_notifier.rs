use hyper::Uri;
use yaml_rust::yaml::Yaml;

use super::common::{parse_key, parse_yaml_to_string, Result};
use crate::config::SlackNotifierConfig;
use crate::error::ConfigError;

pub fn parse(id: &str, body: &Yaml) -> Result<SlackNotifierConfig> {
    let mut webhook_url_opt: Option<Uri> = None;

    match body {
        Yaml::Hash(hash) => {
            for (attr_yaml_key, attr_yaml_val) in hash {
                let attr_key = parse_key(&attr_yaml_key)?;

                match attr_key.as_ref() {
                    "type" => (),
                    "webhook_url" => {
                        let val = parse_yaml_to_string(&attr_yaml_val)?;
                        let url: Uri = val.parse().map_err(|_| {
                            let message = format!(
                                "`{}` in `notifiers.{}.webhook_url` is not a valid URL",
                                val, id
                            );
                            ConfigError::GeneralError { message }
                        })?;
                        webhook_url_opt = Some(url);
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

    let webhook_url = webhook_url_opt.ok_or(ConfigError::FieldMissing {
        path: format!("notifiers.{}.webhook_url", id),
    })?;

    let config = SlackNotifierConfig { webhook_url };
    Ok(config)
}
