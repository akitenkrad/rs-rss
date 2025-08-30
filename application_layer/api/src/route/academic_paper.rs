use crate::handler::academic_paper::{
    add_academic_paper_with_sse, select_academic_papers_by_id, select_paginated_academic_papers,
};
use axum::{routing::get, Router};
use registry::AppRegistry;

pub fn build_academic_paper_router() -> Router<AppRegistry> {
    let routers = Router::new()
        .route("/all", get(select_paginated_academic_papers))
        .route("/paper", get(select_academic_papers_by_id))
        .route("/add-sse", get(add_academic_paper_with_sse));

    Router::new().nest("/academic-paper", routers)
}
