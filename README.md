# I Say

Drop desktop notifications through HTTP API.

``` sh
curl "localhost:4829/?q=hello"
```

[![crates.io](https://badgen.net/crates/v/isay)](https://crates.io/crates/isay) &nbsp;
[![GitHub Actions - Linter Status](https://github.com/outloudvi/isay/workflows/Linters/badge.svg)](https://github.com/outloudvi/isay/actions?query=workflow%3ALinters)

## Installation

```bash
cargo install isay
```

## Usage

```text
isay 0.2.0

USAGE:
    isay [OPTIONS] --app-name <app-name>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --app-name <app-name>    App name for the notification
    -p, --port <port>            HTTP port to listen on [default: 4829]
    -t, --timeout <timeout>      Timeout for the notification. Set it to 0 makes the notification
                                 never disappear. [default: -1]
```

## License

MIT