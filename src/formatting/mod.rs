use yahoo_finance_api as yahoo;
use chrono::{DateTime, TimeZone, Utc};
use xactor::*;

use super::aggregates;
use super::aggregates::{SuperlativeActor, SuperlativeMessage, min, max};


#[xactor::main]
pub async fn main() -> Result<()> {
    // Start actor and get its address
    let mut addr = SuperlativeActor.start().await?;

    let res = addr.call(SuperlativeMessage(min(&[1.1, 1_000_990.0, -1.234]))).await?;

    match res {
        Some(least) => println!("min: {}", least),
        None => println!("none :-("),
    };
    // println!("res of min: {}", res.unwrap());
    Ok(())
}

pub fn formatQuote<'a>(ticker: &'a str, series: &'a Vec<yahoo::Quote>) -> String {
    if series.len() == 0 {
        return "".to_string();
    }

    let prices: Vec<f64> = series.into_iter().map(|elm| elm.adjclose).collect();
    let (absolute_change, relative_change) = match aggregates::price_diff(&prices) {
        Some((abs, rel)) => (abs, rel),
        None => (0.0, 0.0),
    };

    let lows: Vec<f64> = series.into_iter().map(|elm| elm.low).collect();
    let low = match aggregates::min(&lows) {
        Some(l) => l,
        None => 0.0,
    };

    let highs: Vec<f64> = series.into_iter().map(|elm| elm.high).collect();
    let high = match aggregates::max(&highs) {
        Some(h) => h,
        None => 0.0,
    };
    
    let avg: f64 = series.into_iter().fold(0.0, |acc, elm| {
        if elm.adjclose.is_nan() {
            return acc;
        }

        acc + elm.adjclose
    }) / (series.len() as f64);
    
    format!(
        "{},{},${:.2},{:.2}%,${:.2},${:.2},${:.2},",
        Utc.timestamp(series[0].timestamp as i64, 0).to_rfc3339(), ticker,
        absolute_change, relative_change, low, high, avg
    )
}

pub fn formatTitle() -> String {
    "period start,symbol,price,change %,min,max,30d avg".to_string()
}
