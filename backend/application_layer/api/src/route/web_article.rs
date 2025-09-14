use crate::handler::web_article::{
    select_filtered_web_articles, select_paginated_web_articles, select_paginated_web_sites, update_web_article_status,
};
use axum::{
    routing::{get, post},
    Router,
};
use registry::AppRegistry;

pub fn build_web_site_router() -> Router<AppRegistry> {
    let routers = Router::new()
        .route("/select_all_web_sites", get(select_paginated_web_sites))
        .route("/select_all_web_articles", get(select_paginated_web_articles))
        .route("/select_filtered_web_articles", get(select_filtered_web_articles))
        .route("/update_web_article_status", post(update_web_article_status));

    Router::new().nest("/web_site", routers)
}
