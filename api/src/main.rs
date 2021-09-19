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

    for stock in query.stocks.split(",") {
        let summary = stock_data.get_summary(stock);
        if summary.is_some() {
            result.push(SummaryResponse {
                stock: stock.into(),
                summary: summary.unwrap(),
            });
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

#[derive(Serialize, Deserialize)]
struct SummaryResponse {
    stock: String,
    summary: StockSummary,
}
