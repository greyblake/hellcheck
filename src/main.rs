
use std::time::Duration;
use humantime::Duration as HumanDuration;
use hyper::Uri;

use yaml_rust::{YamlLoader, YamlEmitter, yaml::Yaml};

const CONFIG: &'static str = "
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
";

#[derive(Debug)]
struct FileConfig {
    checkers: Vec<CheckerConfig>,
    notifiers: Vec<NotifierConfig>
}

#[derive(Debug)]
struct TelegramNotifierConfig {
    token: String,
    chat_id: u64
}

#[derive(Debug)]
enum NotifierConfig {
    Telegram(TelegramNotifierConfig)
}

#[derive(Debug)]
struct CheckerConfig {
    id: String,
    url: Uri,
    interval: Duration,
    notifiers: Vec<String>
}

fn parse_config(yaml: &str) -> FileConfig {
    let mut checkers = vec![];
    let mut notifiers = vec![];

    let docs = YamlLoader::load_from_str(CONFIG).unwrap();

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

fn parse_notifiers(notifier_configs: &Yaml) -> Vec<NotifierConfig> {
    vec![]
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



fn main() {
    // hellcheck::run();
    let config = parse_config(CONFIG);

    println!("{:#?}", config);
}
