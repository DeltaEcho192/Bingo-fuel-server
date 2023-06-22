use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tower_http::cors::{Any, CorsLayer};
use http::Method;
use urlencoding::encode;

const URL:&str = "https://maps.googleapis.com/maps/api/directions/json";
const KEY:&str =  "AIzaSyAIf-vJKm6y4vhqsCFdMkuRYIOjb8Q8rxM";

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/", get(root))
        .route("/users", post(create_user))
        .route("/api/route", post(route))
        .layer(cors);

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
    let mut main_url = String::from(URL);
    main_url.push_str("?destination=place_id:");
    main_url.push_str(&payload.route.last().unwrap());
    main_url.push_str("&origin=place_id:");
    main_url.push_str(&payload.route.get(0).unwrap());
    
    let way_len = payload.route.len() - 1;
    if way_len > 1 {
        main_url.push_str("&waypoints=");
        for i in 1..way_len {
            main_url.push_str("place_id:");
            main_url.push_str(&payload.route.get(i).unwrap());
            if i != way_len {
                main_url.push_str("%7C");
            }
        }
    }
    main_url.push_str("&key=");
    main_url.push_str(&KEY);
    let route = Route {
        Addr: main_url
    };
    (StatusCode::CREATED, Json(route))
}
// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}


#[derive(Deserialize)]
struct Waypoints {
    route: Vec<String>,
}

#[derive(Serialize)]
struct Route {
   Addr : String,
}

#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}
