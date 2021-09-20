use crate::{messages::StockUpdated, state::StockDataSink};
use actix::{
    clock::{interval_at, Instant},
    Actor, Addr, Context,
};
use futures::StreamExt;
use std::time::Duration;

use super::user_store::UserStore;

const TICK_INTERVAL: u64 = 1;

/// Stock Engine
/// engine that generates ticks and informs UserStore of Stock Updates
/// this engine is the only place from where we are updating the AppState's stock data
pub(crate) struct StockEngine {
    pub stock_data_sink: StockDataSink,
    pub user_store: Addr<UserStore>,
}

impl Actor for StockEngine {
    type Context = Context<Self>;

    /// once started, perform ticking and update of stock data, and inform UserStore
    fn started(&mut self, _ctx: &mut Self::Context) {
        let stock_data = self.stock_data_sink.clone();
        let user_store = self.user_store.clone();
        let mut thread_rng = rand::thread_rng();

        actix_web::rt::spawn(async move {
            let mut task = interval_at(Instant::now(), Duration::from_secs(TICK_INTERVAL));

            while task.next().await.is_some() {
                stock_data
                    .write()
                    .unwrap()
                    .generate_next_tick(&mut thread_rng);
                user_store.do_send(StockUpdated {});
            }
        });
    }
}
