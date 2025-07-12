use crate::route::{build_academic_paper_router, build_health_check_router, build_web_site_router};
use axum::Router;
use registry::AppRegistry;

pub fn routes() -> Router<AppRegistry> {
    let routers = Router::new()
        .merge(build_academic_paper_router())
        .merge(build_health_check_router())
        .merge(build_web_site_router());
    Router::new().nest("/api/v1", routers)
}
