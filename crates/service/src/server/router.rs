use axum::routing::{Router, get};

use crate::shared::error::Result;

pub async fn router<S, S2>(state: S) -> Result<Router<S2>>
where
    S: Clone + Send + Sync + 'static,
{
    let router = Router::new().route("/", get("/")).with_state(state);

    Ok(router)
}
