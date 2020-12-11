mod aggregates;
mod api_interface;
mod formatting;

#[macro_use]
extern crate clap;

use clap::{Arg, App, SubCommand};

fn main() {
    let yaml = load_yaml!("../cli.yml");
    let matches = App::new("ffl")
        .version("0.1.0")
        .author("Justin Hale <jinhale@gmail.com>")
        .about("Report market figures")
        .arg(Arg::with_name("ticker")
             .short("t")
             .long("ticker")
             .value_name("TICKER_CODE")
             .help("Sets a custom config file")
             .takes_value(true))
        .arg(Arg::with_name("interval")
             .short("i")
             .long("interval")
             .value_name("INTERVAL_CODE")
             .help("The smallest unit of time to measure: 1m, 1h, 1d, 1m")
             .takes_value(true))
        .arg(Arg::with_name("duration")
             .short("d")
             .long("duration")
             .value_name("DURATION_CODE")
             .help("The range of time to consider")
             .takes_value(true)).get_matches();

    let ticker = matches.value_of("ticker").unwrap_or("MSFT");
    let interval = matches.value_of("interval").unwrap_or("30d");
    let duration = matches.value_of("duration").unwrap_or("1d");

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
        Err(e) => println!("error: {}", e),
    }
}
