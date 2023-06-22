use crate::Deserialize;
use crate::Serialize;
// the input to our `create_user` handler
#[derive(Deserialize)]
pub struct CreateUser {
    pub username: String,
}


pub struct Route_data {
    pub test: usize
}

#[derive(Deserialize)]
pub struct Waypoints {
    pub route: Vec<String>,
}

#[derive(Serialize)]
pub struct Route {
   pub Addr : String,
}

#[derive(Serialize)]
pub struct User {
    pub id: u64,
    pub username: String,
}

#[derive(Deserialize, Debug)]
pub struct leg_distance {
    pub text: String,
    pub value: usize 
}

#[derive(Deserialize)]
pub struct leg_duration {
    pub text: String,
    pub value: usize
}

#[derive(Deserialize)]
pub struct location {
    pub lat: usize,
    pub lng: usize
}

#[derive(Deserialize)]
pub struct leg_step {
    pub distance: leg_distance,
    pub duration: leg_duration,
    //end_location: location 
}

#[derive(Deserialize)]
pub struct Legs {
    pub distance: leg_distance,
    pub duration: leg_duration,
    pub steps: Vec<leg_step>
}

#[derive(Deserialize)]
pub struct RouteOption {
    pub legs: Vec<Legs>
}

#[derive(Deserialize)]
pub struct Google_Response {
    pub routes: Vec<RouteOption>
}

