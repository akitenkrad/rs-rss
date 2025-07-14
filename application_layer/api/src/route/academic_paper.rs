use crate::handler::academic_paper::select_paginated_academic_papers;
use axum::{routing::get, Router};
use registry::AppRegistry;

pub fn build_academic_paper_router() -> Router<AppRegistry> {
    let routers = Router::new()
        .route("/all", get(select_paginated_academic_papers))
        .route(
            "/paper",
            get(crate::handler::academic_paper::select_academic_papers_by_id),
        );

    Router::new().nest("/academic_paper", routers)
}
