use crate::Deserialize;
use crate::Serialize;
// the input to our `create_user` handler
#[derive(Deserialize)]
pub struct CreateUser {
    pub username: String,
}

#[derive(Deserialize)]
pub struct Waypoints {
    pub route: Vec<String>,
}

#[derive(Serialize)]
pub struct User {
    pub id: u64,
    pub username: String,
}

#[derive(Deserialize, Debug)]
pub struct LegDistance {
    pub text: String,
    //Unit used is meter
    pub value: usize 
}

#[derive(Deserialize)]
pub struct LegDuration {
    pub text: String,
    //Unit used is Seconds
    pub value: usize
}

#[derive(Deserialize)]
pub struct Location {
    pub lat: usize,
    pub lng: usize
}

#[derive(Deserialize)]
pub struct LegStep {
    pub distance: LegDistance,
    pub duration: LegDuration,
    //end_location: Location 
}

#[derive(Deserialize)]
pub struct Legs {
    pub distance: LegDistance,
    pub duration: LegDuration,
    pub steps: Vec<LegStep>
}

#[derive(Deserialize)]
pub struct RouteOption {
    pub legs: Vec<Legs>
}

#[derive(Deserialize)]
pub struct GoogleResponse {
    pub routes: Vec<RouteOption>
}

#[derive(Serialize)]
pub struct DataResponse {
    pub data: Vec<DataDisplay>,
    pub embed_url: String
}

impl DataResponse {
    pub fn new() -> DataResponse {
        DataResponse { data: Vec::new() , embed_url: String::from("")}
    }
}

#[derive(Serialize)]
pub struct DataDisplay {
    pub id: String,
    pub value: String
}

#[derive(Deserialize)]
pub struct UserData {
    pub username: String,
    pub password: String
}
