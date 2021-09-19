use rand::{self, prelude::ThreadRng, Rng};
use std::collections::HashMap;

const TICKERS: [&'static str; 6] = ["GOOG", "APPL", "TSLA", "AMZN", "MSFT", "FB"];
type Price = f64;

/// Holds our ticker data
#[derive(Debug)]
pub struct StockData {
    thread_rng: ThreadRng,
    lowest: HashMap<&'static str, Option<Price>>,
    highest: HashMap<&'static str, Option<Price>>,
    data: HashMap<&'static str, Vec<Price>>,
}

impl StockData {
    pub fn initialize() -> Self {
        let mut data = HashMap::new();
        let mut highest = HashMap::new();
        let mut lowest = HashMap::new();

        for ticker in TICKERS {
            data.insert(ticker, vec![]);
            lowest.insert(ticker, None);
            highest.insert(ticker, None);
        }

        StockData {
            thread_rng: rand::thread_rng(),
            lowest,
            highest,
            data,
        }
    }

    /// randomly generates new data for each ticker and adds it to the hash map
    pub fn generate_next_tick(&mut self) {
        let next_prices: [Price; TICKERS.len()] = self.thread_rng.gen();
        let mut iter = next_prices.iter().map(|v| v * 100f64);

        for ticker in TICKERS {
            let next_price = iter.next().unwrap();
            self.insert_next(ticker, next_price);
            self.insert_lowest(ticker, next_price);
            self.insert_highest(ticker, next_price);
        }
    }

    /// get history of all recorded prices for given ticker
    pub fn get_prices(&self, ticker: &str) -> Option<&Vec<Price>> {
        self.data.get(ticker)
    }

    /// get lowest recorded price for a given ticker
    pub fn get_lowest_price(&self, ticker: &str) -> Option<Price> {
        *self.lowest.get(ticker).unwrap_or(&None)
    }

    /// get highest recorded price for a given ticker
    pub fn get_highest_price(&self, ticker: &str) -> Option<Price> {
        *self.highest.get(ticker).unwrap_or(&None)
    }

    /// inserts new value to the end of the vector of a given tick
    fn insert_next(&mut self, ticker: &'static str, price: Price) {
        if let Some(current_prices) = self.data.get_mut(ticker) {
            current_prices.push(price);
        }
    }

    /// inserts new value for given tick if it's the lowest ever recorded
    fn insert_lowest(&mut self, ticker: &'static str, price: Price) {
        if let Some(current_price) = self.lowest.get(ticker) {
            match current_price {
                Some(v) => {
                    if price < *v {
                        self.lowest.insert(ticker, Some(price));
                    }
                }
                None => {
                    self.lowest.insert(ticker, Some(price));
                }
            };
        };
    }

    /// inserts new value for given tick if it's the highest ever recorded
    fn insert_highest(&mut self, ticker: &'static str, price: Price) {
        if let Some(current_price) = self.highest.get(ticker) {
            match current_price {
                Some(v) => {
                    if price > *v {
                        self.highest.insert(ticker, Some(price));
                    }
                }
                None => {
                    self.highest.insert(ticker, Some(price));
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

        assert!(stock_data.data.contains_key(stock));
        assert!(stock_data.get_lowest_price(stock).is_none());
        assert!(stock_data.get_highest_price(stock).is_none());

        // first tick happens
        stock_data.generate_next_tick();

        assert!(stock_data.get_lowest_price(stock).is_some());
        assert!(stock_data.get_highest_price(stock).is_some());
        assert_eq!(stock_data.data.get(stock).unwrap().len(), 1);

        // second tick happens
        stock_data.generate_next_tick();
        assert_eq!(stock_data.data.get(stock).unwrap().len(), 2);

        // 100 more ticks happen
        for _ in 0..100 {
            stock_data.generate_next_tick();
        }

        assert_eq!(stock_data.data.get(stock).unwrap().len(), 102);

        let lowest = stock_data.get_lowest_price(stock).unwrap();
        let highest = stock_data.get_highest_price(stock).unwrap();

        for price in stock_data.data.get(stock).unwrap() {
            assert!(lowest <= *price);
            assert!(highest >= *price);
        }
    }
}
