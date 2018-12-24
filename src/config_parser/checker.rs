use yaml_rust::yaml::Yaml;
use hyper::Uri;

use std::time::Duration;

use crate::config::CheckerConfig;
use crate::error::ConfigError;
use super::common::{Result, parse_key, parse_yaml_to_string, parse_yaml_to_vec};

pub fn parse(key: &Yaml, body: &Yaml) -> Result<CheckerConfig> {
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
                            }
                            Err(_) => {
                                let e = ConfigError::InvalidCheckerInterval {
                                    interval: attr_val,
                                    checker_id: id,
                                };
                                return Err(e);
                            }
                        }
                    }
                    "url" => {
                        let attr_val = parse_yaml_to_string(&attr_yaml_val)?;
                        match attr_val.parse::<Uri>() {
                            Ok(url) => {
                                url_opt = Some(url);
                            }
                            Err(_) => {
                                let e = ConfigError::InvalidCheckerUrl {
                                    checker_id: id,
                                    url: attr_val,
                                };
                                return Err(e);
                            }
                        };
                    }
                    "notifiers" => {
                        notifiers = parse_yaml_to_vec(&attr_yaml_val)?;
                    }
                    _ => {
                        let err = ConfigError::UnknownCheckerAttribute {
                            checker_id: id.clone(),
                            attr_name: attr_key,
                        };
                        return Err(err);
                    }
                }
            }
        }
        _ => {
            let message = format!("`checkers.{}` must be a hash. Got {:?}", id, body);
            return Err(ConfigError::GeneralError { message });
        }
    };

    let url = url_opt.ok_or(ConfigError::FieldMissing {
        path: format!("checkers.{}.url", id),
    })?;

    let cf = CheckerConfig {
        id,
        interval,
        url,
        notifiers,
    };
    Ok(cf)
}
