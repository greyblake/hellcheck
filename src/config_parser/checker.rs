use hyper::Uri;
use yaml_rust::yaml::Yaml;

use std::time::Duration;

use super::common::{parse_key, parse_yaml_to_string, parse_yaml_to_vec, Result};
use crate::config::{CheckerConfig, BasicAuth};
use crate::error::ConfigError;

pub fn parse(key: &Yaml, body: &Yaml) -> Result<CheckerConfig> {
    let id = parse_key(key)?;
    let mut notifiers: Vec<String> = vec![];
    let mut basic_auth: Option<BasicAuth> = None;

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
                    },
                    "basic_auth" => {
                        let raw_basic_auth = parse_basic_auth(&id, &attr_yaml_val)?;
                        basic_auth = Some(raw_basic_auth);
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
        basic_auth,
        notifiers,
    };
    Ok(cf)
}

fn parse_basic_auth(checker_id: &str, val: &Yaml) -> Result<BasicAuth> {
    match val {
        Yaml::Hash(hash) => {
            let mut username_opt: Option<String> = None;
            let mut password_opt: Option<String> = None;

            for (attr_yaml_key, attr_yaml_val) in hash {
                let attr_key = parse_key(&attr_yaml_key)?;

                match attr_key.as_ref() {
                    "username" => {
                        let attr_val = parse_key(&attr_yaml_val)?;
                        username_opt = Some(attr_val);
                    },
                    "password" => {
                        let attr_val = parse_key(&attr_yaml_val)?;
                        password_opt = Some(attr_val);
                    }
                    _ => {
                        let message = format!("Unknown attribute checkers.{}.basic_auth.{}", checker_id, attr_key);
                        return Err(ConfigError::GeneralError { message });
                    }
                }
            }

            let username = username_opt.ok_or(ConfigError::FieldMissing {
                path: format!("checkers.{}.basic_auth.username", checker_id),
            })?;

            let password = password_opt.ok_or(ConfigError::FieldMissing {
                path: format!("checkers.{}.basic_auth.password", checker_id),
            })?;

            let basic_auth = BasicAuth { username, password };
            Ok(basic_auth)
        },
        _ => {
            let message = format!("basic_auth must be a hash. Got {:?}", val);
            Err(ConfigError::GeneralError { message })
        }
    }
}
