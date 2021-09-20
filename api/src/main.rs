use std::collections::HashMap;

use actix::{Actor, Addr};
use actix_web::{
    web::{self, Data, HttpResponse},
    App, Error, HttpRequest, HttpServer,
};
mod actors;
mod messages;
mod state;
use actix_web_actors::ws;
use actors::{socket_session::SocketSession, stock_engine::StockEngine, user_store::UserStore};
use serde::{Deserialize, Serialize};
use state::AppState;
use stock::StockSummary;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let address = "127.0.0.1:3000";
    let app_state = state::AppState::new();

    let user_store: Addr<UserStore> = UserStore {
        users: HashMap::new(),
        stock_data_sink: app_state.stock_data.clone(),
    }
    .start();

    let stock_engine: Addr<StockEngine> = StockEngine {
        stock_data_sink: app_state.stock_data.clone(),
        user_store: user_store.clone(),
    }
    .start();

    // Create Http server with websocket support
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .data(stock_engine.clone())
            .data(user_store.clone())
            .route("/summary", web::get().to(get_summary))
            .service(web::resource("/ws/").to(handle_subscribe))
    })
    .bind(address)?
    .run()
    .await
}

async fn get_summary(state: Data<AppState>, query: web::Query<StockQuery>) -> HttpResponse {
    let stock_data = state.stock_data.read().unwrap();
    let mut result = vec![];

    let summaries = stock_data.get_summaries();

    for stock in query.stocks.split(",") {
        if let Some(summary) = summaries.get(stock) {
            if summary.is_some() {
                result.push(SummaryResponse {
                    stock: stock.into(),
                    summary: summary.clone().unwrap(),
                });
            }
        }
    }

    HttpResponse::Ok().json(result)
}

/// Entry point for our websocket route
async fn handle_subscribe(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<UserStore>>,
) -> Result<HttpResponse, Error> {
    let socket_session = SocketSession {
        addr: srv.get_ref().clone(),
        user_id: rand::random(),
    };

    ws::start(socket_session, &req, stream)
}

#[derive(Deserialize, Serialize, Debug)]
pub struct StockQuery {
    stocks: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct SummaryResponse {
    stock: String,
    summary: StockSummary,
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::dev::{Service, ServiceResponse};
    use actix_web::{http, test, web, App};
    use std::sync::{Arc, RwLock};
    use stock::{StockData, StockTrend};

    #[actix_rt::test]
    async fn test_get_summary() {
        let mut stock_data = StockData::initialize();
        let mut thread_rng = rand::thread_rng();
        stock_data.generate_next_tick(&mut thread_rng);

        let app_state = Data::new(AppState {
            stock_data: Arc::new(RwLock::new(stock_data)),
        });

        let app = App::new()
            .app_data(app_state.clone())
            .route("/summary", web::get().to(get_summary));

        let mut app = test::init_service(app).await;
        let req = test::TestRequest::get().uri("/summary").to_request();
        let resp: ServiceResponse = app.call(req).await.unwrap();
        assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);

        let req = test::TestRequest::get()
            .uri("/summary?stocks=APPL,GOOG")
            .to_request();
        let resp: ServiceResponse = app.call(req).await.unwrap();
        assert_eq!(resp.status(), http::StatusCode::OK);

        let req = test::TestRequest::get()
            .uri("/summary?stocks=APPL,GOOG")
            .to_request();
        let sum_resp: Vec<SummaryResponse> = test::read_response_json(&mut app, req).await;

        assert_eq!(sum_resp.len(), 2);

        let apple_summary = sum_resp.first().unwrap();
        assert_eq!(apple_summary.stock, "APPL");
        assert!(apple_summary.summary.highest_price.is_some());
        assert!(apple_summary.summary.lowest_price.is_some());
        assert!(apple_summary.summary.moving_average > 0.0);
        assert_eq!(apple_summary.summary.trend, StockTrend::NotEnoughData);
    }
}
