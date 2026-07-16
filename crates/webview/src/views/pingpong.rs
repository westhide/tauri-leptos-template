use leptos::{prelude::*, task::spawn_local};
use libgrpc::protowire::Ping;
use service::traits::from_ctx::FromCtx;

use crate::shared::{client::GrpcClient, logger::error};

#[component]
pub fn PingPong() -> impl IntoView {
    let id = RwSignal::new(0);

    let on_click = move |_| {
        let ping = Ping { id: id.get() };
        #[cfg(client)]
        spawn_local(async move {
            let mut client = GrpcClient::from_ctx();
            match client.pingpong(ping).await {
                Ok(pong) => id.set(pong.into_inner().id),
                Err(err) => error!("{err}"),
            }
        })
    };

    view! {
        <div>
            <div>ID: {id}</div>
            <button on:click=on_click>Call PingPong</button>
            <button on:click=move |_| { *id.write() += 1 }>+1</button>
        </div>
    }
}
