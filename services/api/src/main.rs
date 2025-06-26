use axum::{
    routing::get,
    Router,
};
use tower::ServiceBuilder;

#[tokio::main]
async fn main() {

    let unprotected_routes = Router::new()
        .route("/neurium/api/login", get(AuthManager::get_key()))

    let protected_routes = Router::new()
        .route("/neurium/api/test", get(handler))
        .layer(
            ServiceBuilder::new()
                .layer(AuthManager::auth())
        )

}
