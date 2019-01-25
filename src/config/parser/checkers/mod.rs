use yaml_rust::yaml::Yaml;

use crate::config::CheckerConfig;
use crate::error::ConfigError;

use super::common;
use super::common::Result;

mod http;

pub fn parse_checkers(checker_configs: &Yaml) -> Result<Vec<CheckerConfig>> {
    let mut checkers = vec![];

    match checker_configs {
        Yaml::Hash(hash) => {
            for (yaml_key, val) in hash.iter() {
                let checker = http::parse(yaml_key, val)?;
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
