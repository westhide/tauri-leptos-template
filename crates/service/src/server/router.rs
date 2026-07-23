use axum::{
    Extension,
    extract::FromRef,
    http::StatusCode,
    routing::{Router, get},
};
use leptos::prelude::*;
use leptos_axum::{
    ErrorHandler, LeptosRoutes,
    generate_route_list_with_exclusions_and_ssg_and_context as generate_route_list,
    site_pkg_dir_service, site_pkg_dir_service_route_path,
};
use tokio_util::{sync::CancellationToken, task::TaskTracker};

use crate::{
    config::Config,
    impl_from_ctx,
    routes::{database::schemas, version::version},
    server::{
        context::Context,
        extensions::{database::Client, platform::SaasPlatform},
        middleware::credential::Auth,
    },
    shared::error::Result,
};

pub async fn router<F, IV, S, S2>(ctx: Context<S>, shell: F) -> Result<Router<S2>>
where
    S: Clone + Send + Sync + 'static,
    F: Fn(LeptosOptions) -> IV + 'static + Clone + Sync + Send,
    IV: IntoView + 'static,
    Config: FromRef<S>,
    LeptosOptions: FromRef<Context<S>>,
{
    let config = Config::from_ref(&ctx.state);
    let database = Client::new(&config.server.database).await?;
    let platform = SaasPlatform::new(config.server.platform.clone());
    let credential = Auth::layer(database.clone(), config.server.credential.clone());

    let ctx_hook = {
        let Context { state, task_tracker, cancellation } = ctx.clone();
        let database = database.clone();
        let platform = platform.clone();

        move || {
            provide_context(state.clone());
            provide_context(config.clone());
            provide_context(database.clone());
            provide_context(platform.clone());
            provide_context(task_tracker.clone());
            provide_context(cancellation.clone());
        }
    };

    let options = LeptosOptions::from_ref(&ctx);

    let render_fn = {
        let shell = shell.clone();
        let options = options.clone();
        move || shell(options.clone())
    };

    let routes = { generate_route_list(render_fn.clone(), None, ctx_hook.clone()).0 };

    let serve_dir = site_pkg_dir_service(&options);
    let pkg_dir_route = site_pkg_dir_service_route_path(&options);

    let fallback = ErrorHandler::new_with_context(ctx_hook.clone(), shell, options);

    let router = Router::new()
        // API
        .route("/api/health", get(StatusCode::OK))
        .route("/api/version", get(version))
        .route("/api/database/schemas", get(schemas))
        // Leptos SSR
        .leptos_routes_with_context(&ctx, routes, ctx_hook, render_fn)
        // Assets & pkg
        .route_service("/assets/{*path}", serve_dir.clone())
        .route_service(&pkg_dir_route, serve_dir)
        // Auth layer
        .layer(credential)
        // Fallback
        .fallback_service(fallback)
        // Extensions
        .layer(Extension(database))
        .layer(Extension(platform))
        // Context state
        .with_state(ctx);

    Ok(router)
}

// Safety: Nonce provided
impl_from_ctx!(Nonce);

// Unsafe: must call provide_context() hook
impl_from_ctx!(Client);
impl_from_ctx!(SaasPlatform);
impl_from_ctx!(TaskTracker);
impl_from_ctx!(CancellationToken);
