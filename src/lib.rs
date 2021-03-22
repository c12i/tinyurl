//! # tinyurl
//! An abstraction on top the [tinyurl](https://tinyurl.com) API in rust for quickly generating short urls.
//!
//!# Example
//! - Builder
//!```rust
//!use tinyurl::TinyURL;
//!// without alias
//!let short = TinyURL::new("https://example.com").build();
//!assert!(short.is_ok());
//!
//!// with alias
//!let aliased = TinyURL::new("https://example.com")
//!            .alias("some-random-nickname")
//!            .build();
//!assert!(aliased.is_ok());
//!```
//!
//! - Macro
//!```rust
//!use tinyurl::tiny;
//!
//! // without alias
//!let short = tiny!("https://github.com");
//!assert!(short.is_ok());
//!
//! // with alias
//!let short = tiny!("https://github.com", alias = "random-random-alias");
//!assert!(short.is_ok());
//!```
//!
//! ## CLI
//! - Install
//!```shell
//!cargo install tinyurl
//!```
//!
//! - Usage
//!```shell
//! tinyurl 0.1.0
//! Collins Muriuki <murerwacollins@gmail.com>
//! An abstraction on top the tinyurl API in rust for quickly generating short urls.
//! USAGE:
//!     tinyurl [OPTIONS] <uri>
//! FLAGS:
//!     -h, --help       Prints help information
//!     -V, --version    Prints version information
//! OPTIONS:
//!     -a, --alias <alias>    Optional unique url alias
//! ARGS:
//!     <uri>    The uri to be shortened
//!```

use reqwest::blocking::Client;
use std::error::Error;
use url::Url;

static BASE_URL: &str = "https://tinyurl.com/api-create.php";

#[derive(Debug)]
pub struct TinyURL<'u> {
    uri: &'u str,
    alias: Option<&'u str>,
}

impl<'u> TinyURL<'u> {
    pub fn new(uri: &'u str) -> TinyURL<'u> {
        TinyURL {
            uri,
            ..Default::default()
        }
    }

    pub fn alias(mut self, alias: &'u str) -> TinyURL<'u> {
        self.alias = Some(alias);
        self
    }

    pub fn build(self) -> Result<String, Box<dyn Error>> {
        let mut base = Url::parse(BASE_URL)?;

        base.query_pairs_mut().append_pair("url", self.uri);

        if let Some(a) = self.alias {
            base.query_pairs_mut().append_pair("alias", a);
        }

        let response = Client::new().get(base).send()?;
        Ok(response.text()?)
    }
}

impl<'u> Default for TinyURL<'u> {
    fn default() -> Self {
        TinyURL {
            uri: "https://google.com",
            alias: None,
        }
    }
}

#[macro_export]
macro_rules! tiny {
    ($uri:expr) => {
        tinyurl::TinyURL::new($uri).build()
    };

    ($uri:expr, alias=$alias:expr) => {
        tinyurl::TinyURL::new($uri).alias($alias).build()
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder() {
        let short = TinyURL::new("https://example.com").build();
        assert!(short.is_ok());
    }

    #[test]
    fn test_with_alias() {
        let short = TinyURL::new("https://example.com")
            .alias("some-random-nickname")
            .build();
        assert!(short.is_ok());
    }
}
