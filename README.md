# Hellcheck

HTTP health checker.


## Getting started

DISCLAIMER: This is an MVP version, the project is ongoing development.

### Install

With cargo:

```
cargo install hellcheck
```

## Configuration

Create `hellcheck.yml` file:

```yaml
checkers:
  greyblake:
    url: https://www.greyblake.com
    interval: 10s
    notifiers: [me]
notifiers:
  me:
    type: telegram
    token: <BOT-TOKEN>
    chat_id: <CHAT-ROOM-ID>
```

## License

[MIT](https://github.com/greyblake/whatlang-rs/blob/master/LICENSE) © [Sergey Potapov](http://greyblake.com/)

## Contributors

- [greyblake](https://github.com/greyblake) Potapov Sergey - creator, maintainer.
