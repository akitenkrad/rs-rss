use crate::handler::web_article::get_all_sites;
use axum::{routing::get, Router};
use registry::AppRegistry;

pub fn build_web_site_router() -> Router<AppRegistry> {
    let routers = Router::new().route("/all", get(get_all_sites));

    Router::new().nest("/web_site", routers)
}
