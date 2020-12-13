
use futures::executor::block_on;

// Aggregate the closing (adjclose) prices and find their minimum (fn min(series: &[f64]) -> Option<f64>) and maximum (fn max(series: &[f64]) -> Option<f64>) across the period. What data structures and types from the standard library can you use?

pub async fn async_min(series: &[f64]) -> Option<f64> {
    min(series)
}

pub fn min(series: &[f64]) -> Option<f64> {
    if series.len() == 0 {
        return None;
    }

    let mut least = series.into_iter().nth(0).unwrap();

    for f in series {
        if f.is_nan() {
            continue;
        } else if f < least {
            least = f;
        }
    }

    match least {
        x if !x.is_nan() => Some(*x),
        _ => None,
    }
}

pub async fn async_max(series: &[f64]) -> Option<f64> {
    max(series)
}

pub fn max(series: &[f64]) -> Option<f64> {
    if series.len() == 0 {
        return None;
    }

    let mut most = series.into_iter().nth(0).unwrap();

    for f in series {
        if f.is_nan() {
            continue;
        } else if f > most {
            most = f;
        }
    }

    match most {
        x if !x.is_nan() => Some(*x),
        _ => None,
    }
}

// A simple moving average (SMA) calculates the average of a selected
// range of prices, usually closing prices, by the number of periods
// in that range.
pub async fn async_n_window_sma(n: usize, series: &[f64]) -> Option<Vec<f64>> {
    n_window_sma(n, series)
}

pub fn n_window_sma(n: usize, series: &[f64]) -> Option<Vec<f64>> {
    if series.len() == 0 || n == 0 || n > series.len() {
        return None;
    }

    let sma_windows = series.windows(n);
    let sma = sma_windows.map(|overton| overton.iter().sum::<f64>() / (n as f64)).collect();

    Some(sma)
}

pub async fn async_price_diff(series: &[f64]) -> Option<(f64, f64)> {
    price_diff(series)
}

pub fn price_diff(series: &[f64]) -> Option<(f64, f64)> {
    if series.len() < 2 {
        return None;
    }

    // implement relative diff as % change
    let diff = series[series.len() - 1] - series[0];
    Some((diff, diff / series[0]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_works() {
        let _vec = vec![4.1, 5.1, 6.1, 7.1, 8.1, 9.1, -2.1];
        assert_eq!(min(&_vec), Some(-2.1));
    }

    #[test]
    fn async_min_works() {
        let _vec = vec![4.1, 5.1, 6.1, 7.1, 8.1, 9.1, -22.1];
        assert_eq!(block_on(async_min(&_vec)), Some(-22.1));
    }

    #[test]
    fn min_fails_well() {
        let _vec = vec![];
        assert_eq!(min(&_vec), None);
    }

    #[test]
    fn max_works() {
        let _vec = vec![-4.1, -5.1, 6.1, 7.1, -8.1, -9.1, 2.1];
        assert_eq!(max(&_vec), Some(7.1));
    }

    #[test]
    fn async_max_works() {
        let _vec = vec![-4.1, -5.1, 6.1, 7.1, -8.1, -9.1, 2.1];
        assert_eq!(block_on(async_max(&_vec)), Some(7.1));
    }

    #[test]
    fn max_fails_well() {
        let _vec = vec![];
        assert_eq!(max(&_vec), None);
    }

    #[test]
    fn n_window_sma_works() {
        let test_vec: Vec<f64> = vec![1.3, 2.3, 3.3, 4.3, 5.3, 6.3, 7.3, 8.3, 9.3, 10.3];
        assert_eq!(n_window_sma(5, &test_vec), Some(vec![3.3, 4.3, 5.3, 6.3, 7.3, 8.3]));
    }

    #[test]
    fn n_window_sma_bad_overlap_fails_well() {
        let _vec: Vec<f64> = vec![1.3, 2.3, ];
        assert_eq!(n_window_sma(5, &_vec), None);
    }

    #[test]
    fn n_window_sma_no_fails_well() {
        let _vec: Vec<f64> = vec![1.3, 2.3, ];
        assert_eq!(n_window_sma(0, &_vec), None);
    }

    #[test]
    fn n_window_sma_no_vec_fails_well() {
        let _vec: Vec<f64> = vec![];
        assert_eq!(n_window_sma(1, &_vec), None);
    }

    #[test]
    fn async_n_window_sma_works() {
        let test_vec: Vec<f64> = vec![1.3, 2.3, 3.3, 4.3, 5.3, 6.3, 7.3, 8.3, 9.3, 10.3];
        assert_eq!(block_on(async_n_window_sma(5, &test_vec)), Some(vec![3.3, 4.3, 5.3, 6.3, 7.3, 8.3]));
    }

    #[test]
    fn price_diff_works() {
        let test_vec: Vec<f64> = vec![1.3, 2.3, 3.3, 4.3, 5.3, 6.3, 7.3, 8.3, 9.3, 10.3];
        assert_eq!(price_diff(&test_vec), Some((9.0, 9.0 / 1.3)));
    }

    #[test]
    fn async_diff_works() {
        let test_vec: Vec<f64> = vec![1.3, 2.3, 3.3, 4.3, 5.3, 6.3, 7.3, 8.3, 9.3, 10.3];
        assert_eq!(block_on(async_price_diff(&test_vec)), Some((9.0, 9.0 / 1.3)));
    }
}
