use crate::html::{actions, index, update_goal, update_number, validate_numbers};
use crate::state::initial_state;

use axum::{
    routing::{get, post},
    Router,
};
use tower_http::services::ServeDir;

pub async fn run() {
    let state = initial_state();

    let app = Router::new()
        .route("/", get(index))
        .route("/update_goal", post(update_goal))
        .route("/update_number/:index", post(update_number))
        .route("/validate_numbers", post(validate_numbers))
        // .route("/actions", get(actions))
        // .route("/get_solution", post(get_solution))
        // .route("/reveal_goal", post(reveal_goal))
        // .route("/solve_action", post(solve_action))
        .nest_service("/assets", ServeDir::new("assets"))
        .with_state(state);

    let address = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    println!("Server Started: http://{address}");

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
