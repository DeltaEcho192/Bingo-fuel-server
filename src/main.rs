use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tower_http::cors::{Any, CorsLayer};
use http::Method;
mod datastruct;
use datastruct::*;
use compound_duration::format_dhms;

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

    println!("{:?}", main_url);
    let route_data:GoogleResponse = get_route_values(main_url).await.unwrap();
    calc_data(&route_data);
    let route = Route {
        Addr: String::from("Test")
    };
    (StatusCode::CREATED, Json(route))
}

async fn get_route_values(url: String) -> Result<GoogleResponse, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let body:GoogleResponse = client.get(url).send().await?.json::<GoogleResponse>().await?;
    println!("{:?}", body.routes.get(0).unwrap().legs.get(0).unwrap().distance);
    Ok(body)
}

fn calc_data(data:&GoogleResponse) -> DataResponse {
    let mut response:DataResponse = DataResponse::new(); 
    let legs:&Vec<Legs> = &data.routes.get(0).unwrap().legs;
    let total_time:usize = legs.into_iter().fold(0, |acc, b| acc + b.duration.value);
    let total_distance_m:usize = legs.into_iter().fold(0, |acc, b| acc + b.distance.value);
    let total_dist_km:f64 = total_distance_m as f64 / 1000f64;
    println!("total Time {:?}", format_dhms(total_time));
   /* 
    for leg in &legs {
        total_time += leg.duration.value;
        total_distance += leg.distance.value;
    }*/
    response.data.push(DataDisplay {id: String::from("Time"), value: format_dhms(total_time)});
    response.data.push(DataDisplay {id: String::from("Distance"), value: total_dist_km.to_string()});

    response
}
