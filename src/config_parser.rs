use std::time::Duration;

use hyper::Uri;
use yaml_rust::{YamlLoader, yaml::Yaml};

use crate::config::{FileConfig, CheckerConfig, NotifierConfig, Notifier, TelegramNotifierConfig};

pub fn parse_config(yaml: &str) -> FileConfig {
    let mut checkers = vec![];
    let mut notifiers = vec![];

    let docs = YamlLoader::load_from_str(yaml).unwrap();

    for doc in docs.iter() {
        match doc {
            Yaml::Hash(root) => {
                for (yaml_key, val) in root.iter() {
                    let key = parse_key(yaml_key);

                    match key.as_ref() {
                        "checkers" => {
                            checkers = parse_checkers(val);
                        },
                        "notifiers" => {
                            notifiers = parse_notifiers(val);
                        },
                        _ => panic!("Unknown element {}", key)
                    };
                }
            },
            _ => panic!("Root element of YAML must be Hash")
        }
    }


    FileConfig { checkers, notifiers }
}

fn parse_key(key: &Yaml) -> String {
    match key {
        Yaml::String(s) => s.to_owned(),
        _ => panic!("Key must be a string. Got {:?}", key)
    }
}

fn parse_yaml_to_string(val: &Yaml) -> String {
    match val {
        Yaml::String(s) => s.to_owned(),
        Yaml::Integer(num) => num.to_string(),
        _ => panic!("Value must be a string. Got {:?}", val)
    }
}

fn parse_yaml_to_vec(val: &Yaml) -> Vec<String> {
    let mut items: Vec<String> = vec![];

    match val {
        Yaml::Array(arr) => {
            for yaml_item in arr.iter() {
                let item = parse_yaml_to_string(yaml_item);
                items.push(item);
            }
        }
        _ => panic!("Value must be an array. Got {:?}", val)
    }

    items
}

fn parse_yaml_to_hash(val: &Yaml) -> &yaml_rust::yaml::Hash {
    match val {
        Yaml::Hash(hash) => hash,
        _ => panic!("Value must be a hash. Got {:?}", val)
    }
}

fn parse_notifiers(notifier_configs: &Yaml) -> Vec<Notifier> {
    let mut notifiers = vec![];

    match notifier_configs {
        Yaml::Hash(hash) => {
            for (yaml_key, val) in hash.iter() {
                let checker = parse_notifier(yaml_key, val);
                notifiers.push(checker);
            }
        },
        _ => panic!("notifiers must be a hash")
    }

    notifiers
}

fn parse_notifier(key: &Yaml, body: &Yaml) -> Notifier {
    let id = parse_key(key);
    let config = parse_notifier_config(&id, body);

    Notifier { id, config }
}

fn parse_notifier_config(id: &str, body: &Yaml) -> NotifierConfig {
    let hash = parse_yaml_to_hash(body);
    let type_val_yaml = hash.get(&Yaml::String("type".to_owned()))
        .expect(&format!("notifiers.{}.type is missing", id));

    let type_val = parse_yaml_to_string(type_val_yaml);
    println!("{:?}", type_val);

    match type_val.as_ref() {
        "telegram" => parse_telegram_notifier_config(id, body),
        _ => panic!(format!("notifiers.{}.type has invalid value `{}`", id, type_val))
    }
}

fn parse_telegram_notifier_config(id: &str, body: &Yaml) -> NotifierConfig {
    let mut token_opt: Option<String> = None;
    let mut chat_id_opt: Option<String> = None;

    match body {
        Yaml::Hash(hash) => {
            for (attr_yaml_key, attr_yaml_val) in hash {
                let attr_key = parse_key(&attr_yaml_key);

                match attr_key.as_ref() {
                    "type" => (),
                    "token" => {
                        let attr_val = parse_yaml_to_string(&attr_yaml_val);
                        token_opt = Some(attr_val);
                    },
                    "chat_id" => {
                        let attr_val = parse_yaml_to_string(&attr_yaml_val);
                        chat_id_opt = Some(attr_val);
                    },
                    _ => panic!("Unknown telegram notifier attribute: notifiers.{}.{}", id, attr_key)
                }
            }
        },
        _ => panic!("notifiers.{} must be a hash", id)
    };

    let token = token_opt.expect(&format!("Missing notifiers.{}.token", id));
    let chat_id = chat_id_opt.expect(&format!("Missing notifiers.{}.chat_id", id));

    let telegram_config = TelegramNotifierConfig { token, chat_id };
    NotifierConfig::Telegram(telegram_config)
}



fn parse_checkers(checker_configs: &Yaml) -> Vec<CheckerConfig> {
    let mut checkers = vec![];

    match checker_configs {
        Yaml::Hash(hash) => {
            for (yaml_key, val) in hash.iter() {
                let checker = parse_checker(yaml_key, val);
                checkers.push(checker);
            }
        },
        _ => panic!("checkers must be a hash")
    }

    checkers
}

fn parse_checker(key: &Yaml, body: &Yaml) -> CheckerConfig {
    let id = parse_key(key);
    let mut notifiers: Vec<String> = vec![];

    // Default interval is 10 sec
    let mut interval = Duration::new(10, 0);
    let mut url_opt: Option<Uri> = None;

    match body {
        Yaml::Hash(hash) => {
            for (attr_yaml_key, attr_yaml_val) in hash {
                let attr_key = parse_key(&attr_yaml_key);

                match attr_key.as_ref() {
                    "interval" => {
                        let attr_val = parse_key(&attr_yaml_val);
                        interval = attr_val.parse::<humantime::Duration>().unwrap().into();
                    },
                    "url" => {
                        let attr_val = parse_yaml_to_string(&attr_yaml_val);
                        let url: Uri = attr_val.parse().unwrap();
                        url_opt = Some(url);
                    },
                    "notifiers" => {
                        notifiers = parse_yaml_to_vec(&attr_yaml_val);
                    }
                    _ => panic!("Unknown checker attribute: checkers.{}.{}", id, attr_key)
                }
            }
        },
        _ => panic!("checkers.{} must be a hash", id)
    };


    let url = url_opt.expect(&format!("checkers.{}.url is missing", id));

    CheckerConfig { id, interval, url, notifiers }
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
        let config = parse_config(yaml);
        assert_eq!(config.checkers.len(), 1);
        assert_eq!(config.notifiers.len(), 1);
    }

    #[test]
    #[should_panic]
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
        parse_config(yaml);
    }

    #[test]
    #[should_panic]
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
        parse_config(yaml);
    }
}
