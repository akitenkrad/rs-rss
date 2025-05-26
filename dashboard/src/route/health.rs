use crate::handler::health::{health_check, health_check_db};
use axum::{routing::get, Router};
use registry::AppRegistry;

pub fn build_health_check_router() -> Router<AppRegistry> {
    let routers = Router::new()
        .route("/", get(health_check))
        .route("/db", get(health_check_db));

    Router::new().nest("/health", routers)
}
