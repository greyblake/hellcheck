# Hellcheck

HTTP health checker.

## Getting started

DISCLAIMER: This is a prove of concept, the project is ongoing development.

## Roadmap

* [ ] Support notifiers
  * [x] command customer notifier
  * [x] telegram
  * [ ] Slack
  * [ ] HipChat
* [ ] Checkers
  * [x] Custom intervals
  * [ ] Verify body (presence of some given text)
  * [ ] Custom OKish HTTP status
* [ ] Use structopt/clap for nice command line interface
* [ ] Implement `hellcheck test` command to test notifiers
* [x] Configure CI
  * [x] Run build/tests
  * [x] Setup clippy lint
  * [x] Setup rusmft
* [ ] Ensure endpoints with http basic authentication can be health checked
* [ ] Inject credentials with env variables into yaml file
* [ ] Allow customizable messages for notifiers
* [ ] Allow custom scripts as checkers
* [ ] Make pretty colorized output for console
* [ ] Validate for unexpected panics in the code (unwrap, panic, expect, etc..)

### Install

### Install with cargo

Install system dependencies.

On Debian/Ubuntu:

```sh
apt-get install libssl-dev pkg-config
```

Install hellcheck crate:

```sh
cargo install hellcheck
```

### Configuration file

Create `hellcheck.yml` file:

```yaml
checkers:
  greyblake:
    url: https://www.greyblake.com
    interval: 10s
    notifiers: [me]
  localhost8000:
    url: http://localhost:8000
    interval: 1500ms
    notifiers: [me, custom]
notifiers:
  me:
    type: telegram
    token: <BOT-TOKEN>
    chat_id: <CHAT-ID>
  custom:
    type: command
    command: ["./custom.sh", "arg1", "arg2"]
```

## Notifiers

### Telegram notifier

For telegram notifier you have to create a bot with [BotFather](https://telegram.me/BotFather) and
obtain the bot token.

Chat ID can be found out with [GetidsBot](https://telegram.me/getidsbot).

```yaml
notifiers:
  me:
    type: telegram
    token: <BOT-TOKEN>
    chat_id: <CHAT-ID>
```

### Command notifier

Command notifier allows you to invoke any shell command or custom script as notifier.

Example:

```yaml
notifiers:
  custom:
    type: command
    command: ["/path/to/custom-notifier.sh", "arg1", "arg2"]
```

Within the script the following environment variables are accessible:

* `HELLCHECK_ID` - checker id
* `HELLCHECK_URL` - checker URL
* `HELLCHECK_OK`
  * `true` - when service is up
  * `false` - when service is down


## Start

Assuming, you have `./hellcheck.yml` in your current directory, this will start monitoring of the services,
described in `checkers` configuration sections:

```
hellcheck
```

## License

[MIT](https://github.com/greyblake/whatlang-rs/blob/master/LICENSE) © [Sergey Potapov](http://greyblake.com/)

## Contributors

- [greyblake](https://github.com/greyblake) Potapov Sergey - creator, maintainer.
