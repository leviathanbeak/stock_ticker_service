use std::collections::HashMap;

use actix::{Actor, Addr, Context, Handler};

use crate::{
    messages::{Connected, SendClientMessage, StockUpdated, UpdateUserSubscriptions},
    state::StockDataSink,
};

use super::socket_session::SocketSession;

const USER_CREDITS: u32 = 1024;

/// UserStore
/// where we store newly created Users and their info
/// the only place from where we update Users
pub(crate) struct UserStore {
    pub users: HashMap<usize, User>,
    pub stock_data_sink: StockDataSink,
}

impl Actor for UserStore {
    type Context = Context<Self>;
}

impl Handler<StockUpdated> for UserStore {
    type Result = ();

    /// on stock updates - iterate over all users and send them their subscribed prices
    /// also performs crediting the users
    fn handle(&mut self, _msg: StockUpdated, _ctx: &mut Self::Context) -> Self::Result {
        let stock_data = self.stock_data_sink.read().unwrap();

        for (_, user) in &mut self.users {
            let subs = user.subscriptions.len() as u32;

            if subs > 0 && user.credits > 0 && user.credits >= subs {
                let response = user
                    .subscriptions
                    .iter()
                    .filter(|stock| stock_data.get_last_price(stock).is_some())
                    .map(|stock| {
                        format!("{}: {}", stock, stock_data.get_last_price(stock).unwrap())
                    })
                    .collect::<Vec<String>>()
                    .join(",");

                if !response.is_empty() {
                    user.addr.do_send(SendClientMessage { message: response });
                    user.credits = user.credits - subs;
                }
            }
        }
    }
}

impl Handler<UpdateUserSubscriptions> for UserStore {
    type Result = ();

    /// handles users subscriptions that are coming via websocket
    fn handle(&mut self, msg: UpdateUserSubscriptions, _ctx: &mut Self::Context) -> Self::Result {
        let user = self.users.get_mut(&msg.user_id);
        if user.is_some() {
            let user = user.unwrap();
            for stock in msg.subscriptions {
                user.subscriptions.push(stock);
            }
        }
    }
}

impl Handler<Connected> for UserStore {
    type Result = ();

    /// creates new User struct with info from the SocketSession (user_id, Addr<SocketSession>)
    fn handle(&mut self, msg: Connected, _ctx: &mut Self::Context) -> Self::Result {
        let user = User::new(msg.user_id, msg.addr);
        self.users.insert(user.id, user);
    }
}

pub(crate) struct User {
    credits: u32,
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
