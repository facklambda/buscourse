use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Agency {
    pub agency_id: i32,
    pub agency_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Route {
    pub route_id: String,
    pub agency_id: i32,
    pub route_label: String,
}

#[tokio::main]
pub async fn get_agencies() -> Result<Vec<Agency>, reqwest::Error> {
    let agencies: Vec<Agency> = reqwest::get("https://svc.metrotransit.org/nextripv2/agencies")
        .await?
        .json()
        .await?;
    // println!("{:#?}", agencies);
    return Ok(agencies);
}
#[tokio::main]
pub async fn get_routes() -> Result<Vec<Route>, reqwest::Error> {
    let routes: Vec<Route> = reqwest::get("https://svc.metrotransit.org/nextripv2/routes")
        .await?
        .json()
        .await?;
    return Ok(routes);
}
