use std::fs::File;
use std::io::{Write, BufReader, BufWriter, BufRead, Error};

use serde::{Serialize, Deserialize};

use yahoo_finance_api as yahoo;
use chrono::{DateTime, TimeZone, Utc};

use super::aggregates;

#[derive(Serialize, Deserialize)]
pub struct QuoteRecord<'a> {
    initialTime: &'a str,
    symbol: &'a str,
    absoluteChange: f64,
    relativeChange: f64,
    lowestPrice: f64,
    highestPrice: f64,
    averagePrice: f64,
}

impl QuoteRecord<'_> {
    fn to_csv(&self, byte_wrapper: &mut Vec<u8>) {
        let mut csv = self.initialTime.to_string();
        csv.push_str(self.symbol);
        csv.push_str(&self.absoluteChange.to_string());
        csv.push_str(&self.relativeChange.to_string());
        csv.push_str(&self.lowestPrice.to_string());
        csv.push_str(&self.highestPrice.to_string());
        csv.push_str(&self.averagePrice.to_string());

        csv.into_bytes().copy_from_slice(byte_wrapper);
    }
}

// pub async fn new_csv<'a>(ticker: &'a str, series: &'a Vec<yahoo::Quote>) -> 'a + QuoteRecord {
// Why do I need to use `+` and why is QuoteRecord 'not a trait'?
//     let prices: Vec<f64> = series.into_iter().map(|elm| elm.adjclose).collect();
//     let (absolute_change, relative_change) = match aggregates::async_price_diff(&prices).await {
//         Some((abs, rel)) => (abs, rel),
//         None => (0.0, 0.0),
//     };

//     let lows: Vec<f64> = series.into_iter().map(|elm| elm.low).collect();
//     let low = match aggregates::async_min(&lows).await {
//         Some(l) => l,
//         None => 0.0,
//     };

//     let highs: Vec<f64> = series.into_iter().map(|elm| elm.high).collect();
//     let high = match aggregates::async_max(&highs).await {
//         Some(h) => h,
//         None => 0.0,
//     };
    
//     let avg: f64 = series.into_iter().fold(0.0, |acc, elm| {
//         if elm.adjclose.is_nan() {
//             return acc;
//         }

//         acc + elm.adjclose
//     }) / (series.len() as f64);

//     QuoteRecord {
//         initialTime: Utc.timestamp(series[0].timestamp as i64, 0).to_rfc3339(),
//         symbol: ticker,
//         absoluteChange: absolute_change,
//         relativeChange: relative_change,
//         lowestPrice: low,
//         highestPrice: high,
//         averagePrice: avg,
//     }
// }

pub async fn format_quote<'a>(ticker: &'a str, series: &'a Vec<yahoo::Quote>) -> String {
    if series.len() == 0 {
        return "".to_string();
    }

    let prices: Vec<f64> = series.into_iter().map(|elm| elm.adjclose).collect();
    let (absolute_change, relative_change) = match aggregates::async_price_diff(&prices).await {
        Some((abs, rel)) => (abs, rel),
        None => (0.0, 0.0),
    };

    let lows: Vec<f64> = series.into_iter().map(|elm| elm.low).collect();
    let low = match aggregates::async_min(&lows).await {
        Some(l) => l,
        None => 0.0,
    };

    let highs: Vec<f64> = series.into_iter().map(|elm| elm.high).collect();
    let high = match aggregates::async_max(&highs).await {
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

pub fn format_title() -> String {
    "period start,symbol,price,change %,min,max,30d avg".to_string()
}

pub fn write_csv_quotes(record: QuoteRecord) -> Result<(), Error> {
    let path = "quote_record.csv";
    let mut output = File::create(path)?;
    let mut stream = BufWriter::new(output);
    let mut bytes: Vec<u8> = Vec::new();
    record.to_csv(&mut bytes);
    stream.write(&bytes)?;

    Ok(())
}
