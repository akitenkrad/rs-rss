use crate::handler::paper_note::{create_paper_note, delete_paper_note, select_paper_note, update_paper_note};
use axum::{routing::get, Router};
use registry::AppRegistry;

pub fn build_paper_note_router() -> Router<AppRegistry> {
    let routers = Router::new()
        .route("/select", get(select_paper_note))
        .route("/create", get(create_paper_note))
        .route("/update", get(update_paper_note))
        .route("/delete", get(delete_paper_note));

    Router::new().nest("/paper-note", routers)
}
