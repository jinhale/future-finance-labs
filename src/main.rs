mod aggregates;
mod api_interface;
mod formatting;

use std::time::{Duration, Instant};
use std::thread::sleep;

#[macro_use]
extern crate clap;
use clap::App;

fn main() {
    let yaml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let ticker = matches.value_of("ticker").unwrap_or("MSFT");
    let interval = matches.value_of("interval").unwrap_or("1d");
    let duration = matches.value_of("duration").unwrap_or("30d");

    loop {
        let maybe_quotes = api_interface::get_quotes(ticker, interval, duration);

        match maybe_quotes {
            Ok(quotes) => {
                if quotes.len() > 2 {
                    println!("{}", formatting::formatTitle());
                    println!("{}", formatting::formatQuote(&ticker, &quotes));
                } else {
                    println!("{:?}", quotes[0]);
                    println!("quotes.len(): {}", quotes.len());
                }
            },
            Err(e) => {
                eprintln!("Error: {}", e);

                std::process::exit(1);
            }
        };

        // sleep for 30 seconds
        sleep(Duration::new(30, 0));
    }
}
