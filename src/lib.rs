use hyper::Client;
use hyper::rt::{Future, Stream};
use hyper_tls::HttpsConnector;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::sync::mpsc;

mod config;
mod config_parser;
mod reactor;

use crate::config::{FileConfig, CheckerConfig};
use crate::config_parser::parse_config;
use crate::reactor::{StateMessage, State};


type HttpsClient = hyper::Client<hyper_tls::HttpsConnector<hyper::client::connect::HttpConnector>>;

pub fn load_config() -> FileConfig {
    let file = File::open("./hellcheck.yml").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut content = String::new();
    buf_reader.read_to_string(&mut content).expect("Failed to read from file");
    parse_config(&content)
}



pub fn run() {
    let config = load_config();


    let (sender, receiver) = mpsc::channel::<StateMessage>();

    reactor::spawn(receiver, config.clone());

    let check_runner = CheckRunner { config, sender  };


    let fs = check_runner.config.checkers.iter().map(|c| check_runner.build_fff(c));
    let f = futures::future::select_all(fs);

    let mut core = tokio_core::reactor::Core::new().unwrap();
    core.run(f);
}

struct CheckRunner {
    config: FileConfig,
    sender: mpsc::Sender<StateMessage>
}

impl CheckRunner {
    fn build_fff<'a>(&'a self, service: &'a CheckerConfig) -> Box<Future<Item=(), Error=tokio_timer::Error> + 'a> {
        let stream = tokio_timer::Interval::new_interval(service.interval);
        let client = build_client();

        let id = service.id.clone();

        let f = stream.for_each(move |_| {
            let checker_id = id.clone();
            client
                .get(service.url.clone())
                .then(move |r| {
                    let state = match r {
                        Ok(resp) => {
                            if resp.status() == 200 {
                                State::Up
                            } else {
                                State::Down
                            }
                        },
                        Err(_err) => State::Down
                    };
                    let msg = StateMessage { checker_id: checker_id.clone(), state };
                    self.sender.send(msg).unwrap();

                    Ok(())
                })
        });
        Box::new(f)
    }
}


fn build_client() -> HttpsClient {
    let https = HttpsConnector::new(1).expect("TLS initialization failed");
    let client = Client::builder()
        .build::<_, hyper::Body>(https);

    client
}
