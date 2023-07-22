use axum::{
    http::StatusCode,
    routing::{get, post},
    extract::State,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tower_http::cors::{Any, CorsLayer};
use std::time::Duration;
use http::Method;
mod datastruct;
use datastruct::*;
use sqlx::postgres::{PgPool, PgPoolOptions};
use compound_duration::format_dhms;
use uuid::Uuid;
use jwt_simple::prelude::*;

#[macro_use]
extern crate lazy_static;

lazy_static!{
        static ref HASHKEY: HS256Key = HS256Key::generate();
}

const URL:&str = "https://maps.googleapis.com/maps/api/directions/json";
const EMBED_URL:&str = "https://www.google.com/maps/embed/v1/directions";
const KEY:&str =  "AIzaSyAIf-vJKm6y4vhqsCFdMkuRYIOjb8Q8rxM";



#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let db_connection_str = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://admin:xxmaster@localhost/bingo_fuel".to_string());

    // setup connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
        .expect("can't connect to database");

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/", get(root))
        .route("/users", post(create_user))
        .route("/api/route", post(route))
        .route("/api/checkUser", post(get_user)).with_state(pool)
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

async fn route(Json(payload): Json<Waypoints>,) -> (StatusCode, Json<DataResponse>) {
    let main_url = url_generator(&payload, true);
    let embed_url = url_generator(&payload, false);
    println!("{:?}", main_url);
    println!("EMBEDD: {:?}", embed_url);
    let route_data:GoogleResponse = get_route_values(main_url).await.unwrap();
    let mut user_data:DataResponse = calc_data(&route_data);
    user_data.embed_url = embed_url;
    (StatusCode::CREATED, Json(user_data))
}

#[derive(sqlx::FromRow, Serialize)]
struct UserSQL {
    userid: Uuid, 
    username: String,
    password: String,
}

#[derive(Serialize)]
struct UserLogin {
    valid: u32,
    jwt: String,
}

async fn get_user(State(pool): State<PgPool>, Json(payload): Json<UserData>,) -> (StatusCode, Json<UserLogin>) {


    let claims = Claims::create(jwt_simple::prelude::Duration::from_hours(2));
    let token = HASHKEY.authenticate(claims).unwrap();

    let data = sqlx::query_as::<_, UserSQL>("SELECT * FROM user_tbl WHERE username = $1 LIMIT 50")
        .bind(payload.username)
        .fetch_one(&pool)
        .await
        .map_err(internal_error);

    let data_res = match data {
        Ok(user_data) => {
            let check = UserLogin {
                valid: 200,
                jwt: token.to_string()
            };
            check
        }
        Err(error) => {
            let check = UserLogin {
                valid: 500,
                jwt: token.to_string()
            };
            check
        }
    };

    (StatusCode::OK, Json(data_res))
     //tracing::debug!("{}", data.unwrap().username );

}

fn url_generator(payload: &Waypoints, selector:bool) -> String {
    let mut main_url;
    if selector == true {
        main_url = String::from(URL);
    } else {
        main_url = String::from(EMBED_URL);
    }
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
            if i != (way_len - 1) {
                main_url.push_str("%7C");
            }
        }
    }
    main_url.push_str("&key=");
    main_url.push_str(&KEY);
    main_url
}

async fn get_route_values(url: String) -> Result<GoogleResponse, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let body:GoogleResponse = client.get(url).send().await?.json::<GoogleResponse>().await?;
    Ok(body)
}

fn calc_data(data:&GoogleResponse) -> DataResponse {
    //Make modular depending on the bike and location
    //Currently Based off of the M1000r
    let fuel_cons:f64 = 0.064;
    let cost_fuel:f64 = 1.82;
    let max_range:f64 = 255.00;

    let mut response:DataResponse = DataResponse::new(); 
    let legs:&Vec<Legs> = &data.routes.get(0).unwrap().legs;
    let total_time:usize = legs.into_iter().fold(0, |acc, b| acc + b.duration.value);
    let total_distance_m:usize = legs.into_iter().fold(0, |acc, b| acc + b.distance.value);
    let total_dist_km:f64 = total_distance_m as f64 / 1000f64;
    let fuel_amt = total_dist_km * fuel_cons;
    let cost = fuel_amt * cost_fuel;
    let stop_amt = (total_dist_km / max_range).floor();

    response.data.push(DataDisplay {id: String::from("Time"), value: format_dhms(total_time)});
    response.data.push(DataDisplay {id: String::from("Distance (Km)"), value: round(total_dist_km, 2).to_string()});
    response.data.push(DataDisplay {id: String::from("Cost"), value: round(cost, 2).to_string()});
    response.data.push(DataDisplay {id: String::from("Fuel Amount"), value: round(fuel_amt, 2).to_string()});
    response.data.push(DataDisplay {id: String::from("Amt Stops"), value: stop_amt.to_string()});

    response
}

fn round(x: f64, decimals: u32) -> f64 {
    let y = 10i64.pow(decimals) as f64;
    (x * y).round() / y
}

#[test]
fn round_test() {
    let res = round(555.678f64, 2);
    assert_eq!(res, 555.68);
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
