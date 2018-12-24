use yaml_rust::yaml::Yaml;

use crate::error::ConfigError;

pub type Result<T> = std::result::Result<T, ConfigError>;

pub fn parse_key(key: &Yaml) -> Result<String> {
    match key {
        Yaml::String(s) => Ok(s.to_owned()),
        Yaml::Integer(num) => Ok(num.to_string()),
        _ => {
            let message = format!("Key must be a string. Got {:?}", key);
            Err(ConfigError::GeneralError { message })
        }
    }
}

pub fn parse_yaml_to_string(val: &Yaml) -> Result<String> {
    match val {
        Yaml::String(s) => Ok(s.to_owned()),
        Yaml::Integer(num) => Ok(num.to_string()),
        _ => {
            let message = format!("Key must be a string. Got {:?}", val);
            Err(ConfigError::GeneralError { message })
        }
    }
}

pub fn parse_yaml_to_vec(val: &Yaml) -> Result<Vec<String>> {
    let mut items: Vec<String> = vec![];

    match val {
        Yaml::Array(arr) => {
            for yaml_item in arr.iter() {
                let item = parse_yaml_to_string(yaml_item)?;
                items.push(item);
            }
        }
        _ => {
            let message = format!("Value must be an array. Got {:?}", val);
            return Err(ConfigError::GeneralError { message });
        }
    }

    Ok(items)
}

pub fn parse_yaml_to_hash(val: &Yaml) -> Result<&yaml_rust::yaml::Hash> {
    match val {
        Yaml::Hash(hash) => Ok(hash),
        _ => {
            let message = format!("Value must be a hash. Got {:?}", val);
            Err(ConfigError::GeneralError { message })
        }
    }
}
