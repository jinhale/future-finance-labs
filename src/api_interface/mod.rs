use yahoo_finance_api as yahoo;
use std::time::{Duration, UNIX_EPOCH};
use chrono::{Utc,TimeZone};
use chrono::prelude::*;
// use tokio_test;

pub fn get_quotes<'a>(ticker: &'a str, interval: &'a str, range: &'a str) -> std::result::Result<Vec<yahoo::Quote>, &'a str> {
    let provider = yahoo::YahooConnector::new();
    let response = tokio_test::block_on(provider.get_quote_range(ticker, interval, range));

    match response {
        Ok(res) => Ok(res.quotes().unwrap()),
        Err(e) => Err("Something went wrong. Are you connected to the internet? "),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fetch_lattest_works() {
        // TODO: mock response from get_quote_range
        // assert_eq!();
    }
}
