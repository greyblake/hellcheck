use hyper::rt::{Future, Stream};
use hyper::Client;
use hyper_tls::HttpsConnector;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::sync::mpsc;

mod config;
mod config_parser;
mod config_validator;
mod error;
mod notifiers;
mod reactor;

use crate::config::{CheckerConfig, FileConfig, BasicAuth};
use crate::config_parser::parse_config;
use crate::config_validator::validate_config;

use crate::reactor::{State, StateMessage};

type HttpsClient = hyper::Client<hyper_tls::HttpsConnector<hyper::client::connect::HttpConnector>>;

pub fn load_config() -> FileConfig {
    let file = File::open("./hellcheck.yml").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut content = String::new();
    buf_reader
        .read_to_string(&mut content)
        .expect("Failed to read from file");

    let config = match parse_config(&content) {
        Ok(config) => config,
        Err(err) => {
            eprintln!("ERROR: {}", err);
            std::process::exit(1);
        }
    };

    match validate_config(&config) {
        Ok(warnings) => {
            for warning in warnings {
                eprintln!("WARNING: {}", warning);
            }
        }
        Err(err) => {
            eprintln!("ERROR: {}", err);
            std::process::exit(1);
        }
    }

    config
}

pub fn run() {
    let config = load_config();

    let (sender, receiver) = mpsc::channel::<StateMessage>();
    reactor::spawn(receiver, config.clone());

    let check_runner = CheckRunner { config, sender };
    let checkers_futures = check_runner
        .config
        .checkers
        .iter()
        .map(|c| check_runner.build_future(c));
    let f = futures::future::select_all(checkers_futures);

    let mut core = tokio_core::reactor::Core::new().unwrap();

    // Run core forever
    let res = core.run(f);

    match res {
        Ok(_) => {}
        Err(_) => {
            eprintln!("ERROR: looks likes hellcheck crashed");
            std::process::exit(1);
        }
    }
}

struct CheckRunner {
    config: FileConfig,
    sender: mpsc::Sender<StateMessage>,
}

impl CheckRunner {
    fn build_future<'a>(
        &'a self,
        service: &'a CheckerConfig,
    ) -> Box<Future<Item = (), Error = tokio_timer::Error> + 'a> {
        let stream = tokio_timer::Interval::new_interval(service.interval);
        let client = build_client();

        let id = service.id.clone();

        let f = stream.for_each(move |_| {
            let checker_id = id.clone();

            let req = build_request(&service);

            client.request(req).then(move |r| {
                let state = match r {
                    Ok(resp) => {
                        if resp.status() == 200 {
                            State::Up
                        } else {
                            State::Down
                        }
                    }
                    Err(_err) => State::Down,
                };
                let msg = StateMessage {
                    checker_id: checker_id.clone(),
                    state,
                };
                self.sender.send(msg).unwrap();

                Ok(())
            })
        });
        Box::new(f)
    }
}

fn build_request(service: &CheckerConfig) -> hyper::Request<hyper::Body> {
    let mut builder = hyper::Request::get(service.url.clone());

    if let Some(ref basic_auth) = service.basic_auth {
        let authorization_header_value = build_authorization_header_value(basic_auth);
        builder.header(
            hyper::header::AUTHORIZATION,
            authorization_header_value
        );
    }

    builder
        .body(hyper::Body::empty())
        .unwrap()
}

fn build_authorization_header_value(auth: &BasicAuth) -> hyper::header::HeaderValue {
    let credentials = format!("{}:{}", auth.username, auth.password);
    let encoded_credentials = base64::encode(&credentials);
    let value = format!("Basic {}", encoded_credentials);
    hyper::header::HeaderValue::from_str(&value).unwrap()
}

fn build_client() -> HttpsClient {
    let https = HttpsConnector::new(1).expect("TLS initialization failed");
    Client::builder().build::<_, hyper::Body>(https)
}
