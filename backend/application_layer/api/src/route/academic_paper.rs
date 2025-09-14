use crate::{
    handler::academic_paper::{
        add_academic_paper_with_sse, select_academic_papers_by_id, select_paginated_academic_papers,
        update_academic_paper_with_sse,
    },
    route::paper_note::build_paper_note_router,
};
use axum::{routing::get, Router};
use registry::AppRegistry;

pub fn build_academic_paper_router() -> Router<AppRegistry> {
    let routers = Router::new()
        .route("/all", get(select_paginated_academic_papers))
        .route("/paper", get(select_academic_papers_by_id))
        .route("/add-sse", get(add_academic_paper_with_sse))
        .route("/update-sse", get(update_academic_paper_with_sse));
    let routers = routers.merge(build_paper_note_router());

    Router::new().nest("/academic-paper", routers)
}
