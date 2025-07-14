use crate::handler::web_article::{select_paginated_web_articles, select_paginated_web_sites};
use axum::{routing::get, Router};
use registry::AppRegistry;

pub fn build_web_site_router() -> Router<AppRegistry> {
    let routers = Router::new()
        .route("/all_web_sites", get(select_paginated_web_sites))
        .route("/all_web_articles", get(select_paginated_web_articles));

    Router::new().nest("/web_site", routers)
}
