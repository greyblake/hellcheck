use std::io::{self, Write};
use hyper::Client;
use hyper::rt::{self, Future, Stream};
use hyper::Uri;

use std::time::Duration;
use humantime::Duration as HumanDuration;

use hyper_tls::HttpsConnector;


type HttpsClient = hyper::Client<hyper_tls::HttpsConnector<hyper::client::connect::HttpConnector>>;


#[derive(Debug)]
struct ServiceConfig {
    id: String,
    uri: Uri,
    interval: Duration
}

pub fn run() {
    // Necessary to make OpenSSL work in a static build.
    // See: https://github.com/emk/rust-musl-builder#making-openssl-work
    openssl_probe::init_ssl_cert_env_vars();

    let items = vec![
        ("ip", "http://httpbin.org/ip", "1s"),
        ("404", "http://httpbin.org/ip21", "10s"),
        ("greyblake", "https://www.greyblake.com", "2s")
    ];


    let fs = items.into_iter()
        .map(build_service_config)
        .map(build_fff);

    let f = futures::future::select_all(fs);

    let mut core = tokio_core::reactor::Core::new().unwrap();
    core.run(f);
}

fn build_service_config((id, url, iterval):(&'static str, &'static str, &'static str)) -> ServiceConfig {
    let sc = ServiceConfig {
        id: id.to_owned(),
        uri: url.parse().unwrap(),
        interval: iterval.parse::<humantime::Duration>().unwrap().into()
    };
    sc
}

fn build_fff(service: ServiceConfig) -> Box<Future<Item=(), Error=tokio_timer::Error>> {
    let stream = tokio_timer::Interval::new_interval(service.interval);
    let client = build_client();

    let id = service.id.clone();

    let f = stream.for_each(move |_| {
        let idd = id.clone();
        client
            .get(service.uri.clone())
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
