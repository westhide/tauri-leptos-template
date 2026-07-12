use axum::{
    extract::FromRef,
    routing::{Router, get},
};
use leptos::prelude::*;
use leptos_axum::{
    ErrorHandler, LeptosRoutes, generate_route_list, site_pkg_dir_service,
    site_pkg_dir_service_route_path,
};

use crate::{routes::version::version, server::error::Result, shell, views::Main};

pub async fn router<S, S2>(state: S) -> Result<Router<S2>>
where
    S: Clone + Send + Sync + 'static,
    LeptosOptions: FromRef<S>,
{
    let options = LeptosOptions::from_ref(&state);

    let routes = generate_route_list(Main);

    let serve_dir = site_pkg_dir_service(&options);
    let pkg_dir_route = site_pkg_dir_service_route_path(&options);

    let fallback = ErrorHandler::new(shell, options.clone());

    let router = Router::new()
        .route("/api/version", get(version))
        .leptos_routes(&state, routes, move || shell(options.clone()))
        .route_service("/assets/{*path}", serve_dir.clone())
        .route_service(&pkg_dir_route, serve_dir)
        .fallback_service(fallback)
        .with_state(state);

    Ok(router)
}
