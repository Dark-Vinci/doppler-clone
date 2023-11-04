use axum::http::{HeaderValue, Method};
use axum::Router;
use tower_http::cors::CorsLayer;

use crate::controllers::auth::auth_controllers;
use crate::controllers::page404::page404;
use crate::controllers::secret::secret_controllers;

pub struct Controller;

impl Controller {
    pub fn new() -> Router {
        let app = Router::new()
            .nest("/api/v1/auth", auth_controllers())
            .nest("/api/v1/secret", secret_controllers())
            .nest("/api/v1/others", secret_controllers())
            .layer(
                CorsLayer::new()
                    .allow_origin("http://localhost:3001".parse::<HeaderValue>())
                    .allow_methods([Method::GET, Method::PATCH, Method::PUT, Method::POST, Method::DELETE])
            )
            .fallback(page404());

        return app;
    }
}
