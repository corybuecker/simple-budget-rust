use crate::SharedState;
mod create;
use axum::{routing::get, Router};
mod delete;
mod edit;
mod index;
mod new;
mod update;

pub fn accounts_router() -> Router<SharedState> {
    Router::new()
        .route("/", get(index::page).post(create::page))
        .route(
            "/:id",
            get(edit::page).put(update::page).delete(delete::page),
        )
        .route("/new", get(new::page))
}
