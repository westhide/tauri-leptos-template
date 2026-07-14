use leptos::{prelude::*, task::spawn_local};
use libgrpc::protowire::Ping;
use service::routes::pingpong;

use crate::shared::logger::{debug, error};

#[component]
pub fn PingPong() -> impl IntoView {
    let id = RwSignal::new(0);

    let on_click = move |_| {
        let ping = Ping { id: id.get() };
        spawn_local(async move {
            debug!("pingpong");
            // TODO: extension
            let mut client = pingpong::client("http://127.0.0.1:3001".parse().unwrap());
            match client.pingpong(ping).await.map(|r| r.into_inner()) {
                Ok(pong) => id.set(pong.id),
                Err(err) => error!("{err}"),
            }
        });
    };

    view! {
        <div>
            <div>ID: {id}</div>
            <button on:click=on_click>Call PingPong</button>
        </div>
    }
}
