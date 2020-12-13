mod aggregates;
mod api_interface;
mod formatting;

use std::time::{Duration, Instant};
use std::thread::sleep;
use futures::executor::block_on;

#[macro_use]
extern crate clap;
use clap::App;

fn main() {
    let yaml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let ticker = matches.value_of("ticker").unwrap_or("MSFT");
    let interval = matches.value_of("interval").unwrap_or("1d");
    let duration = matches.value_of("duration").unwrap_or("30d");
    let system = actix::System::new("test");

    system.run();

    loop {
        let maybe_quotes = api_interface::get_quotes(ticker, interval, duration);

        match block_on(maybe_quotes) {
            Ok(quotes) => {
                if quotes.len() > 2 {
                    let future_quotes = formatting::format_quote(&ticker, &quotes);
                    println!("{}", formatting::format_title());
                    println!("{}", block_on(future_quotes));
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
