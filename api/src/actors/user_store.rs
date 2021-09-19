use std::collections::HashMap;

use actix::{Actor, Addr, Context, Handler};

use crate::{
    messages::{Connected, SendClientMessage, StockUpdated},
    state::StockDataSink,
};

use super::socket_session::SocketSession;

const USER_CREDITS: u8 = 45;

pub(crate) struct UserStore {
    pub users: HashMap<usize, User>,
    pub stock_data_sink: StockDataSink,
}

impl Actor for UserStore {
    type Context = Context<Self>;
}

impl Handler<StockUpdated> for UserStore {
    type Result = ();

    fn handle(&mut self, _msg: StockUpdated, _ctx: &mut Self::Context) -> Self::Result {
        let data = self.stock_data_sink.read().unwrap();

        for (key, user) in &mut self.users {
            if user.credits > 0 {
                // user.addr.do_send(SendMessage {
                //     message: format!("{:?}", data.get_summary("APPL").unwrap()),
                // })
            }
        }
    }
}

impl Handler<Connected> for UserStore {
    type Result = ();

    fn handle(&mut self, msg: Connected, _ctx: &mut Self::Context) -> Self::Result {
        let user = User::new(msg.user_id, msg.addr);
        self.users.insert(user.id, user);
    }
}

pub(crate) struct User {
    credits: u8,
    addr: Addr<SocketSession>,
    id: usize,
    subscriptions: Vec<String>,
}

impl User {
    fn new(id: usize, addr: Addr<SocketSession>) -> Self {
        Self {
            credits: USER_CREDITS,
            addr,
            id,
            subscriptions: vec![],
        }
    }
}
