mod login;
mod register;
mod whoami;

/// Handle `/api/auth/*` requests
pub fn app() -> axum::Router {
    use axum::routing::{get, post};

    axum::Router::new()
        .route("/login", post(login::handle))
        .route("/register", post(register::handle))
        .route("/whoami", get(whoami::handle))
}
