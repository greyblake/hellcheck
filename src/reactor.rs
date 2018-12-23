use std::sync::mpsc;
use crate::config::{FileConfig, CheckerConfig, Notifier, NotifierConfig};

use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum State {
    Up,
    Down
}

#[derive(Debug)]
pub struct StateMessage {
    pub checker_id: String,
    pub state: State
}



pub fn spawn(receiver: mpsc::Receiver<StateMessage>, config: FileConfig) {
    ::std::thread::spawn(move || {
        let mut states = build_initial_states(&config);

        loop {
            let msg = receiver.recv().unwrap();
            let checker = config.get_checker_by_id(&msg.checker_id).unwrap();

            // unwrap is safe here, because `states` was initialized with all possible checker ids.
            let prev_state = states.get(&msg.checker_id).unwrap();

            // Send a message if state was changed
            if msg.state != *prev_state {
                 for notifier_id in checker.notifiers.iter() {
                     println!("{}", notifier_id);
                     let notifier = config.get_notifier_by_id(notifier_id).expect(&format!("Can not find identifier by id={}", notifier_id));
                     notify(notifier, &checker, &msg.state);
                 }
            }

            states.insert(msg.checker_id, msg.state);


        }
    });
}

fn build_initial_states(config: &FileConfig) -> HashMap<String, State> {
    let mut states: HashMap<String, State> = HashMap::new();
    for checker in config.checkers.iter() {
        states.insert(checker.id.clone(), State::Up);
    }
    states
}


use hyper::Method;


fn notify(notifier: Notifier, checker: &CheckerConfig, state: &State) {
    match notifier.config {
        NotifierConfig::Telegram(config) => {
            let client = build_client();
            let mut core = tokio_core::reactor::Core::new().unwrap();

            let text = match state {
                State::Up => {
                    let emoji_baloon = '\u{1F388}';
                    format!("{} is back! {}{}{}\n{}", checker.id, emoji_baloon, emoji_baloon, emoji_baloon, checker.url)
                },
                State::Down => {
                    let emoji_fire = '\u{1F525}';
                    format!("{} is on fire! {}{}{}\n{}", checker.id, emoji_fire, emoji_fire, emoji_fire, checker.url)
                }

            };
            let chat_id = config.chat_id;

            let url = format!("https://api.telegram.org/bot{}/sendMessage", config.token);
            // TODO: use serde to build json
            let json = format!("{{\"chat_id\":\"{}\",\"text\":\"{}\"}}", chat_id, text);

            let uri: hyper::Uri = url.parse().unwrap();
            let mut request = hyper::Request::new(hyper::Body::from(json));
            *request.method_mut() = Method::POST;
            *request.uri_mut() = uri.clone();
            request.headers_mut().insert(
                hyper::header::CONTENT_TYPE,
                hyper::header::HeaderValue::from_static("application/json")
            );

            let f = client.request(request).and_then(|res| {
                res.into_body().concat2()
            });
            core.run(f).unwrap();
        }
    }
}

use hyper::Client;
use hyper::rt::{Future, Stream};
use hyper_tls::HttpsConnector;

type HttpsClient = hyper::Client<hyper_tls::HttpsConnector<hyper::client::connect::HttpConnector>>;
fn build_client() -> HttpsClient {
    let https = HttpsConnector::new(1).expect("TLS initialization failed");
    let client = Client::builder()
        .build::<_, hyper::Body>(https);

    client
}
