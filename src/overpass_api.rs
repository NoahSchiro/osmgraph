use serde::{Serialize, Deserialize};
use serde_json::Value;

use tokio::{
    fs::File,
    io::{AsyncWriteExt, AsyncReadExt},
    runtime::Runtime
};

#[derive(Serialize, Deserialize, Clone, PartialEq, Hash, Debug, Default)]
pub struct OverpassResponse {

    //Elements is where there is important graph information
    elements: Value,

    //Metadata
    generator: Value,
    osm3s: Value,
    version: Value
}

impl OverpassResponse {

    pub fn elements(&self) -> &Value {
        &self.elements
    }
    pub fn generator(&self) -> &Value {
        &self.generator
    }
    pub fn osm3s(&self) -> &Value {
        &self.osm3s
    }
    pub fn version(&self) -> &Value {
        &self.version
    }

    //Save to a valid json
    pub async fn save(&self, filepath: &str) -> Result<(), &'static str> {

        let list_as_json = serde_json::to_string(self).unwrap();

        let mut file = File::create(filepath)
            .await
            .expect("Could not create file!");

        file.write_all(list_as_json.as_bytes())
            .await
            .expect("Could not write to file!");

        file.flush()
            .await
            .expect("Could not flush file!");

        Ok(())
    }

    //Save to a valid json and wait
    pub fn save_blocking(&self, filepath: &str) -> Result<(), &'static str> {
        Runtime::new()
            .expect("Could not create runtime!")
            .block_on(self.save(filepath))
    }

    //Load from a valid json
    pub async fn load(filepath: &str) -> Result<Self, &'static str> {

        let mut file = File::open(filepath)
            .await
            .expect("Could not open filepath!");

        let mut contents = Vec::new();

        // Read the file's contents into the buffer
        file.read_to_end(&mut contents)
            .await
            .expect("Could not read file!");

        let contents_as_string: String = String::from_utf8_lossy(&contents).to_string();

        let json: OverpassResponse = serde_json::from_str(&contents_as_string)
            .expect("JSON was invalid");

        Ok(json)
    }

    //Load from a valid json and wait for response
    pub fn load_blocking(filepath: &str) -> Result<Self, &'static str> {
        Runtime::new()
            .expect("Could not create runtime!")
            .block_on(Self::load(filepath))
    }
}

//A function to request data from the Overpass API given a particular query
pub async fn osm_request(query: String) -> Result<String, reqwest::Error> {

    let url = "https://overpass-api.de/api/interpreter";
    
    let client = reqwest::Client::new();
    let response = client.post(url)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(format!("data={}", query))
        .send()
        .await?;

    // Parse the response as JSON
    let json_string: String = response.text().await?;

    Ok(json_string)
}

pub fn osm_request_blocking(query: String) -> Result<String, reqwest::Error> {
    Runtime::new()
        .expect("Could not create runtime!")
        .block_on(osm_request(query))
}
