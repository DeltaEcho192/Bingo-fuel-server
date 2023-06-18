use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tower_http::{trace::TraceLayer};
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use http::{Request, Response, Method, header};
#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any)
        .allow_headers(Any);
    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user))
        .route("/api/route", post(route))
        .layer(cors);

    // run our app with hyper, listening globally on port 3000
    axum::Server::bind(&"0.0.0.0:3100".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

async fn route(Json(payload): Json<Waypoints>,) -> (StatusCode, Json<Route>) {
    let first = &payload.route[0];

    println!("id: {}, location: {}",first.id, first.location);
    let route = Route {
        firstWaypoint: String::from(&first.location)
    };

    (StatusCode::CREATED, Json(route))
}
// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

#[derive(Deserialize)]
struct Waypoint {
    id: usize,
    location: String
}

#[derive(Deserialize)]
struct Waypoints {
    route: Vec<Waypoint>,
}

#[derive(Serialize)]
struct Route {
    firstWaypoint: String,
}
// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}
