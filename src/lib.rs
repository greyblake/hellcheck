use std::io::{self, Write};
use hyper::Client;
use hyper::rt::{self, Future, Stream};
use hyper::Uri;

use std::time::Duration;
use humantime::Duration as HumanDuration;

use hyper_tls::HttpsConnector;

type HttpsClient = hyper::Client<hyper_tls::HttpsConnector<hyper::client::connect::HttpConnector>>;

mod config;
mod config_parser;

use crate::config::{FileConfig, CheckerConfig, NotifierConfig, Notifier, TelegramNotifierConfig};
use crate::config_parser::parse_config;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn load_config() -> FileConfig {
    let file = File::open("./hellcheck.yml").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut content = String::new();
    buf_reader.read_to_string(&mut content).expect("Failed to read from file");
    parse_config(&content)
}

pub fn run() {
    let config = load_config();

    println!("{:#?}", config);

    let fs = config.checkers.clone().into_iter().map(build_fff);
    let f = futures::future::select_all(fs);

    let mut core = tokio_core::reactor::Core::new().unwrap();
    core.run(f);
}

fn build_fff(service: CheckerConfig) -> Box<Future<Item=(), Error=tokio_timer::Error>> {
    let stream = tokio_timer::Interval::new_interval(service.interval);
    let client = build_client();

    let id = service.id.clone();

    let f = stream.for_each(move |_| {
        let idd = id.clone();
        client
            .get(service.url.clone())
            .map(move |res| {
                let iddd = idd.clone();
                println!("Response: {} {}", res.status(), iddd);
            })
            .map_err(|err| {
                println!("Error: {}", err);
            }).then(|r| {
                Ok(())
            })
    });
    Box::new(f)
}


fn build_client() -> HttpsClient {
    let https = HttpsConnector::new(1).expect("TLS initialization failed");
    let client = Client::builder()
        .build::<_, hyper::Body>(https);

    client
}
