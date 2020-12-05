// Aggregate the closing (adjclose) prices and find their minimum (fn min(series: &[f64]) -> Option<f64>) and maximum (fn max(series: &[f64]) -> Option<f64>) across the period. What data structures and types from the standard library can you use?

pub fn min(series: &[f64]) -> Option<f64> {
    if series.len() == 0 {
        None()
    } else {
        series.iter().min()
    }
}

pub fn max(series: &[f64]) -> Option<f64> {
    if series.len() == 0 {
        None()
    } else {
        series.iter().max()
    }
}

// A simple moving average (SMA) calculates the average of a selected
// range of prices, usually closing prices, by the number of periods
// in that range.

pub fn n_window_sma(n: usize, series: &[f64]) -> Option<Vec<f64>> {
    if series.len() == 0 || n == 0 {
        return None();
    }

    let duration = if n > series.len() { series.len() } else { n };
    let mut sma_beginning = 0;
    let mut sma_ending = duration;
    let mut sma: Vec<f64> = vec![];

    while sma_ending <= series.len() {
        sma.push(series[sma_beginning..sma_ending].iter().sum() / duration);
        sma_beginning += duration;
        sma_ending += duration;
    }

    Some(sma)
}

pub fn price_diff(series: &[f64]) -> Option<(f64, f64)> {
    if series.len() < 2 {
        return None();
    }

    // todo determine what relative price is
    Some((series[0] - series[series.len() - 1], series[0] - series[series.len() - 1]))
}

#[cfg(test)]
mod tests {
    #[test]
    fn min_works() {
        assert_eq!(vec![4.1, 5.1, 6.1, 7.1, 8.1, 9.1, -2.1], Some(-2.1));
    }

    #[test]
    fn max_works() {
        assert_eq!(vec![-4.1, -5.1, 6.1, 7.1, -8.1, -9.1, 2.1], Some(6.1));
    }
}
