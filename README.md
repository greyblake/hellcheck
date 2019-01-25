# Hellcheck

HTTP health checker.


* [Installation](#installation)
   * [Install with cargo](#install-with-cargo)
* [Configuration](#configuration)
   * [Basic configuration example](#basic-configuration-example)
   * [Notifiers](#notifiers)
      * [Slack notifier](#slack-notifier)
      * [Telegram notifier](#telegram-notifier)
      * [HipChat notifier](#hipchat-notifier)
      * [Command notifier](#command-notifier)
* [Start](#start)
* [Roadmap](#roadmap)
* [License](#license)
* [Contributors](#contributors)


## Installation

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

## Configuration

### Basic configuration example

Configuration file `hellcheck.yml` may have the following format:

```yaml
# Declare what has to be watched
checkers:
  example:
    url: https://www.example.com
    notifiers: [my_team]
  localhost8000:
    url: http://localhost:8000
    interval: 1500ms
    notifiers: [my_team, sound_alarm]
    basic_auth:
      username: "foo"
      password: "bar"

# Declare notification channels
notifiers:
  my_team:
    type: slack
    token: <WEBHOOK_URL>
  sound_alarm:
    type: command
    command: ["./custom.sh", "arg1", "arg2"]
```

### Notifiers

#### Slack notifier

Create an [incoming webhook](https://api.slack.com/incoming-webhooks) in Slack.
Then define your notifier with type `slack` and `webhook_url`:

```yaml
notifiers:
  notifier_name:
    type: slack
    webhook_url: <WEBHOOK_URL>
```

#### Telegram notifier

For telegram notifier you have to create a bot with [BotFather](https://telegram.me/BotFather) and
obtain the bot token.

Chat ID can be found out with [GetidsBot](https://telegram.me/getidsbot).

```yaml
notifiers:
  notifier_name:
    type: telegram
    token: <BOT-TOKEN>
    chat_id: <CHAT-ID>
```

#### HipChat notifier

```yaml
notifiers:
  notifier_name:
    type: hipchat
    base_url: https://hipchat.com
    token: <AUTH_TOKEN>
    room_id: <ROOM_NAME_OR_ID>
```

#### Command notifier

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
hellcheck watch --file ./hellcheck.yml
```

## Roadmap

* [x] Support notifiers
  * [x] Command customer notifier
  * [x] Telegram
  * [x] HipChat
  * [x] Slack
* [ ] Checkers
  * [x] Custom intervals
  * [ ] Verify body (presence of some given text)
  * [ ] Custom OKish HTTP status
* [x] Use structopt/clap for nice command line interface
* [ ] Implement `hellcheck test` command to test notifiers
* [x] Configure CI
  * [x] Run build/tests
  * [x] Setup clippy lint
  * [x] Setup rusmft
* [x] Ensure endpoints with http basic authentication can be health checked
* [ ] Inject credentials with env variables into yaml file
* [ ] Allow customizable messages for notifiers
* [ ] Allow custom scripts as checkers
* [ ] Make pretty colorized output for console
* [ ] Validate for unexpected panics in the code (unwrap, panic, expect, etc..)
* [ ] Add logging

## License

[MIT](https://github.com/greyblake/whatlang-rs/blob/master/LICENSE) © [Sergey Potapov](http://greyblake.com/)

## Contributors

- [greyblake](https://github.com/greyblake) Potapov Sergey - creator, maintainer.
