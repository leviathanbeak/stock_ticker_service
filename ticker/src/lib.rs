use rand::{self, prelude::ThreadRng, Rng};
use std::collections::HashMap;

const STOCKS: [&'static str; 6] = ["GOOG", "APPL", "TSLA", "AMZN", "MSFT", "FB"];
type Price = f64;

/// Holds our stock data
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

        for stock in STOCKS {
            data.insert(stock, vec![]);
            lowest.insert(stock, None);
            highest.insert(stock, None);
        }

        StockData {
            thread_rng: rand::thread_rng(),
            lowest,
            highest,
            data,
        }
    }

    /// randomly generates new price for each stock and adds it to the hash maps
    pub fn generate_next_tick(&mut self) {
        let next_prices: [Price; STOCKS.len()] = self.thread_rng.gen();
        let mut iter = next_prices.iter().map(|v| v * 100f64);

        for stock in STOCKS {
            let next_price = iter.next().unwrap();
            self.insert_next(stock, next_price);
            self.insert_lowest(stock, next_price);
            self.insert_highest(stock, next_price);
        }
    }

    /// get history of all recorded prices for given stock
    pub fn get_prices(&self, stock: &str) -> Option<&Vec<Price>> {
        self.data.get(stock)
    }

    /// get lowest recorded price for a given stock
    pub fn get_lowest_price(&self, stock: &str) -> Option<Price> {
        *self.lowest.get(stock).unwrap_or(&None)
    }

    /// get highest recorded price for a given stock
    pub fn get_highest_price(&self, stock: &str) -> Option<Price> {
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
