use crate::{Price, StockTrend};

pub(crate) fn get_trend(prices: &Vec<Price>) -> StockTrend {
    let size = prices.len();

    if size <= 1000 {
        StockTrend::NotEnoughData
    } else {
        let start_index = size - 1000 - 1;
        let low_mid_index = start_index + 250;
        let high_mid_index = low_mid_index + 250;
        let end_index = high_mid_index + 500;

        if prices[start_index] <= prices[low_mid_index]
            && prices[high_mid_index] <= prices[end_index]
        {
            StockTrend::Uptrend
        } else if prices[start_index] >= prices[low_mid_index]
            && prices[high_mid_index] >= prices[end_index]
        {
            StockTrend::Downtrend
        } else {
            StockTrend::Sideways
        }
    }
}

pub(crate) fn moving_average(prices: &Vec<Price>) -> f64 {
    if prices.is_empty() {
        0.0
    } else {
        let sum: f64 = prices.iter().sum();
        sum / prices.len() as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_moving_average() {
        let no_prices = vec![];
        assert_eq!(moving_average(&no_prices), 0.0);

        let prices = vec![1., 2., 3., 4., 5., 6.];
        assert_eq!(moving_average(&prices), 3.5);
    }

    #[test]
    fn test_get_trend() {
        let small_data_set = vec![1., 2., 3., 4., 5., 6.];
        assert_eq!(get_trend(&small_data_set), StockTrend::NotEnoughData);

        let large_uptrend_data_set: Vec<f64> = (0..1001).into_iter().map(|v| v as f64).collect();
        assert_eq!(get_trend(&large_uptrend_data_set), StockTrend::Uptrend);

        let large_uptrend_data_set: Vec<f64> = (0..1124).into_iter().map(|v| v as f64).collect();
        assert_eq!(get_trend(&large_uptrend_data_set), StockTrend::Uptrend);

        let large_downtrend_data_set: Vec<f64> =
            (0..1001).into_iter().map(|v| v as f64).rev().collect();
        assert_eq!(get_trend(&large_downtrend_data_set), StockTrend::Downtrend);

        let large_downtrend_data_set: Vec<f64> =
            (0..1451).into_iter().map(|v| v as f64).rev().collect();
        assert_eq!(get_trend(&large_downtrend_data_set), StockTrend::Downtrend);
    }
}
