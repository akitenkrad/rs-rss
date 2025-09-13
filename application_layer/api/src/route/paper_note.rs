use crate::handler::paper_note::{
    ask_to_agent, create_paper_note, delete_paper_note, select_paper_note, update_paper_note,
};
use axum::{
    routing::{delete, get, post, put},
    Router,
};
use registry::AppRegistry;

pub fn build_paper_note_router() -> Router<AppRegistry> {
    let routers = Router::new()
        .route("/select", get(select_paper_note))
        .route("/create", post(create_paper_note))
        .route("/update", put(update_paper_note))
        .route("/delete", delete(delete_paper_note))
        .route("/ask-to-agent", post(ask_to_agent));

    Router::new().nest("/paper-note", routers)
}
