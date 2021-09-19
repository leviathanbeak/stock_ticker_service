use rand::{self, prelude::ThreadRng, Rng};
use std::collections::HashMap;
use utils::{get_trend, moving_average};
mod utils;
use serde::{Deserialize, Serialize};

const STOCKS: [&'static str; 6] = ["GOOG", "APPL", "TSLA", "AMZN", "MSFT", "FB"];
pub(crate) type Price = f64;

#[derive(Debug, PartialEq, Serialize, Deserialize, Copy, Clone)]
pub enum StockTrend {
    Uptrend,
    Downtrend,
    Sideways,
    NotEnoughData,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct StockSummary {
    pub trend: StockTrend,
    pub lowest_price: Option<Price>,
    pub highest_price: Option<Price>,
    pub moving_average: Price,
}

/// Holds our stock data
#[derive(Debug)]
pub struct StockData {
    lowest: HashMap<&'static str, Option<Price>>,
    highest: HashMap<&'static str, Option<Price>>,
    data: HashMap<&'static str, Vec<Price>>,
    summaries: HashMap<&'static str, Option<StockSummary>>,
}

impl StockData {
    pub fn initialize() -> Self {
        let mut data = HashMap::new();
        let mut highest = HashMap::new();
        let mut lowest = HashMap::new();
        let mut summaries = HashMap::new();

        for stock in STOCKS {
            data.insert(stock, vec![]);
            lowest.insert(stock, None);
            highest.insert(stock, None);
            summaries.insert(stock, None);
        }

        StockData {
            lowest,
            highest,
            data,
            summaries,
        }
    }

    /// randomly generates new price for each stock and adds it to the hash maps
    pub fn generate_next_tick(&mut self, thread_rng: &mut ThreadRng) {
        let next_prices: [Price; STOCKS.len()] = thread_rng.gen();
        let mut iter = next_prices.iter().map(|v| v * 100f64);

        for stock in STOCKS {
            let next_price = iter.next().unwrap();
            self.insert_next(stock, next_price);
            self.insert_lowest(stock, next_price);
            self.insert_highest(stock, next_price);

            let stock_summary = self.get_summary(stock);
            self.summaries.insert(stock, stock_summary);
        }
    }

    /// get all sumarries
    pub fn get_summaries(&self) -> &HashMap<&'static str, Option<StockSummary>> {
        &self.summaries
    }

    /// get the Summary for a given stock    
    fn get_summary(&self, stock: &str) -> Option<StockSummary> {
        if let Some(current_prices) = self.get_prices(stock) {
            let moving_avg = moving_average(current_prices);
            let trend = get_trend(current_prices);
            Some(StockSummary {
                trend,
                lowest_price: self.get_lowest_price(stock),
                highest_price: self.get_highest_price(stock),
                moving_average: moving_avg,
            })
        } else {
            None
        }
    }

    /// get last recorded price for a stock
    pub fn get_last_price(&self, stock: &str) -> Option<Price> {
        if let Some(prices) = self.get_prices(stock) {
            match prices.last() {
                Some(price) => Some(*price),
                None => None,
            }
        } else {
            None
        }
    }

    /// get history of all recorded prices for given stock
    fn get_prices(&self, stock: &str) -> Option<&Vec<Price>> {
        self.data.get(stock)
    }

    /// get lowest recorded price for a given stock
    fn get_lowest_price(&self, stock: &str) -> Option<Price> {
        *self.lowest.get(stock).unwrap_or(&None)
    }

    /// get highest recorded price for a given stock
    fn get_highest_price(&self, stock: &str) -> Option<Price> {
        *self.highest.get(stock).unwrap_or(&None)
    }

    /// inserts new value to the end of the vector of a given stock
    fn insert_next(&mut self, stock: &'static str, price: Price) {
        if let Some(current_prices) = self.data.get_mut(stock) {
            current_prices.push(price);
        }
    }

    /// inserts new value for given stock if it's the lowest ever recorded
    fn insert_lowest(&mut self, stock: &'static str, price: Price) {
        if let Some(current_price) = self.lowest.get(stock) {
            match current_price {
                Some(v) => {
                    if price < *v {
                        self.lowest.insert(stock, Some(price));
                    }
                }
                None => {
                    self.lowest.insert(stock, Some(price));
                }
            };
        };
    }

    /// inserts new value for given stock if it's the highest ever recorded
    fn insert_highest(&mut self, stock: &'static str, price: Price) {
        if let Some(current_price) = self.highest.get(stock) {
            match current_price {
                Some(v) => {
                    if price > *v {
                        self.highest.insert(stock, Some(price));
                    }
                }
                None => {
                    self.highest.insert(stock, Some(price));
                }
            };
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stock_data() {
        let mut stock_data = StockData::initialize();
        let stock = "APPL";

        let mut thread_rng = rand::thread_rng();

        assert!(stock_data.data.contains_key(stock));
        assert!(stock_data.get_lowest_price(stock).is_none());
        assert!(stock_data.get_highest_price(stock).is_none());

        // first tick happens
        stock_data.generate_next_tick(&mut thread_rng);

        assert!(stock_data.get_lowest_price(stock).is_some());
        assert!(stock_data.get_highest_price(stock).is_some());
        assert_eq!(stock_data.data.get(stock).unwrap().len(), 1);

        // second tick happens
        stock_data.generate_next_tick(&mut thread_rng);
        assert_eq!(stock_data.data.get(stock).unwrap().len(), 2);

        // 100 more ticks happen
        for _ in 0..100 {
            stock_data.generate_next_tick(&mut thread_rng);
        }

        assert_eq!(stock_data.data.get(stock).unwrap().len(), 102);

        let lowest = stock_data.get_lowest_price(stock).unwrap();
        let highest = stock_data.get_highest_price(stock).unwrap();

        for price in stock_data.data.get(stock).unwrap() {
            assert!(lowest <= *price);
            assert!(highest >= *price);
        }

        let summary = stock_data.get_summary(stock);
        assert!(summary.is_some());
        let summary = summary.unwrap();

        assert!(summary.highest_price.is_some());
        assert!(summary.lowest_price.is_some());
        assert_eq!(summary.highest_price.unwrap(), highest);
        assert_eq!(summary.lowest_price.unwrap(), lowest);
        assert!(summary.moving_average > 0.0);
    }
}
