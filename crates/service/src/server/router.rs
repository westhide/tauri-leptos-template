use axum::{
    extract::FromRef,
    http::StatusCode,
    routing::{Router, get},
};
use leptos::prelude::*;
use leptos_axum::{
    ErrorHandler, LeptosRoutes, generate_route_list, site_pkg_dir_service,
    site_pkg_dir_service_route_path,
};

use crate::{routes::version::version, shared::error::Result};

pub async fn router<F, IV, S, S2>(shell: F, state: S) -> Result<Router<S2>>
where
    F: Fn(LeptosOptions) -> IV + 'static + Clone + Sync + Send,
    IV: IntoView + 'static,
    S: Clone + Send + Sync + 'static,
    LeptosOptions: FromRef<S>,
{
    let options = LeptosOptions::from_ref(&state);

    let routes = {
        let shell = shell.clone();
        let options = options.clone();
        generate_route_list(move || shell(options.clone()))
    };

    let serve_dir = site_pkg_dir_service(&options);
    let pkg_dir_route = site_pkg_dir_service_route_path(&options);

    let fallback = ErrorHandler::new(shell.clone(), options.clone());

    let router = Router::new()
        .route("/api/health", get(StatusCode::OK))
        .route("/api/version", get(version))
        .leptos_routes(&state, routes, move || shell(options.clone()))
        .route_service("/assets/{*path}", serve_dir.clone())
        .route_service(&pkg_dir_route, serve_dir)
        .fallback_service(fallback)
        .with_state(state);

    Ok(router)
}
