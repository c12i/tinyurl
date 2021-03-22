# tinyurl

<a href="https://crates.io/crates/tinyurl" target="_blank">
     <img alt="Version" src="https://img.shields.io/crates/v/tinyurl" />
</a>
<a href="https://travis-ci.com/collinsmuriuki/tinyurl" target="_blank">
    <img alt="tinyurl travis-ci" src="https://travis-ci.com/collinsmuriuki/tinyurl.svg?branch=master" />
</a>
<a href="https://docs.rs/tinyurl" target="_blank">
    <img alt="Documentation" src="https://docs.rs/tinyurl/badge.svg" />
</a>

An abstraction on top the [tinyurl](https://tinyurl.com) API in rust for quickly generating short urls.

## Library

-   Builder

```rust
use tinyurl::TinyURL;
 // without alias
let short = TinyURL::new("https://example.com").build();
assert!(short.is_ok());

 // with alias
let aliased = TinyURL::new("https://example.com")
            .alias("some-random-nickname")
            .build();
assert!(aliased.is_ok());
```

-   Macro

```rust
use tinyurl::tiny;

 // without alias
let short = tiny!("https://github.com");
assert!(short.is_ok());

// with alias
let short = tiny!("https://github.com", alias = "random-random-alias");
assert!(short.is_ok());
```

## CLI

-   Install

```shell
cargo install tinyurl
```

-   Usage

```shell
tinyurl 0.1.0
Collins Muriuki <murerwacollins@gmail.com>
An abstraction on top the tinyurl API in rust for quickly generating short urls.

USAGE:
    tinyurl [OPTIONS] <uri>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -a, --alias <alias>    Optional unique url alias

ARGS:
    <uri>    The uri to be shortened

```
