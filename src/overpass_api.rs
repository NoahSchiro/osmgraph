use std::fs::{File, read_to_string};
use std::io::prelude::*;

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

impl OverpassResponse {

    //Save from a valid json
    pub fn save(&self, filepath: &str) -> Result<(), &'static str> {

        let list_as_json = serde_json::to_string(self).unwrap();

        let mut file = File::create(filepath)
            .expect("Could not create file!");

        file.write_all(list_as_json.as_bytes())
            .expect("Cannot write to the file!");

        Ok(())
    }

    //Load from a valid json
    pub fn load(filepath: &str) -> Result<Self, &'static str> {

        let file_contents = read_to_string(filepath)
            .expect("Was not able to read from filepath");

        let json: OverpassResponse = serde_json::from_str(&file_contents)
            .expect("JSON was invalid");

        Ok(json)
    }

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
