
use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub struct OverpassResponse {

    //Elements is where there is important graph information
    pub elements: Value,

    //Metadata
    pub generator: Value,
    pub osm3s: Value,
    pub version: Value
}

//A function to request data from the Overpass API given a particular query
pub fn osm_request(query: String) -> Result<String, reqwest::Error> {

    let url = "https://overpass-api.de/api/interpreter";
    
    let client = reqwest::blocking::Client::new();
    let response = client.post(url)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(format!("data={}", query))
        .send()?;

    // Parse the response as JSON
    let json_string: String = response.text()?;

    Ok(json_string)
}
