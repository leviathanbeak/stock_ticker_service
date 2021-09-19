use actix_web::web::Data;
use std::sync::{Arc, RwLock};
use stock::StockData;

pub(crate) type StockDataSink = Arc<RwLock<StockData>>;

#[derive(Debug)]
pub(crate) struct AppState {
    pub stock_data: StockDataSink,
}

impl AppState {
    pub fn new() -> Data<Self> {
        let stock_data = StockData::initialize();

        Data::new(Self {
            stock_data: Arc::new(RwLock::new(stock_data)),
        })
    }
}
