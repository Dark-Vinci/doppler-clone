use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Router;

pub fn auth_controllers() -> Router {
    let authController = AuthControllers::new();

    let router = Router::new()
        .route("/login", authController.login())
        .route("/logout", authController.logout());

    return router;
}

pub struct AuthControllers(String);

impl AuthControllers {
    pub fn new() -> Self {
        Self("".to_owned())
    }

    pub async fn login(&self) -> impl IntoResponse {
        // extract all needed info, pass to application[service]
        return (StatusCode::OK, "you\'re now logged in");
    }

    pub async fn logout(&self) -> impl IntoResponse {
        // extract all needed info, pass to application[service]
        return (StatusCode::OK, "you\'re now logged out");
    }
}

//      anyhow error response(into_response)
//      |> body extractor middleware for validation of request inputs(body, params, query, header)
//      |> custom error extractor
//      |> custom path extractor
//      |> parse query params with empty string

// each layer gets error[enum of errors]
// each layer error is also an enum
// custom request body extractor
// custom query param extractor
// custom param extractor
// custom anyhow error handler [AppError struct<Implements into response>]
// pass stuffs from [with state]
// Standard error object [ApiError]
// global into response for [ApiError]