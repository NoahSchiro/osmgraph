use serde::{Serialize, Deserialize};
use serde_json::Value;

use tokio::{
    fs::File,
    io::{AsyncWriteExt, AsyncReadExt},
    runtime::Runtime
};

/// `OverpassResponse` is the basic structure that we expect the `osm_request` string to return.
/// Serde JSON helps us parse this string into the correct data structure.
///
/// Example:
/// ```rust
/// let query = String::from(r#"
///     [out:json];
///     area[name="Selinsgrove"]->.searchArea;
///     (
///       way(area.searchArea);
///       node(area.searchArea);
///     );
///     out body;
///     >;
///     out skel qt;
/// "#);
/// let response: String = osm_request_blocking(query)
///     .expect("Was not able to request OSM!");
/// let json: OverpassResponse = serde_json::from_str(&response)
///     .expect("Was not able to parse json!");
/// ```
#[derive(Serialize, Deserialize, Clone, PartialEq, Hash, Debug, Default)]
pub struct OverpassResponse {

    //Graph data
    elements: Value,

    //Metadata
    generator: Value,
    osm3s: Value,
    version: Value
}

impl OverpassResponse {

    /// Return the `elements` field from the response. This field is the most important as it
    /// contains the actual graph information.
    pub fn elements(&self) -> &Value {
        &self.elements
    }
    /// Return the `generator` field from the response.
    pub fn generator(&self) -> &Value {
        &self.generator
    }
    /// Return the `osm3s` field from the response.
    pub fn osm3s(&self) -> &Value {
        &self.osm3s
    }
    /// Return the `version` field from the response.
    pub fn version(&self) -> &Value {
        &self.version
    }

    /// Given a specified `filepath`, save the OverpassResponse to that location.
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

    /// Behaves the same as [`OverpassResponse::save`], but will wait for the function to finish before continuing.
    pub fn save_blocking(&self, filepath: &str) -> Result<(), &'static str> {
        Runtime::new()
            .expect("Could not create runtime!")
            .block_on(self.save(filepath))
    }

    /// Given a specified `filepath`, load the OverpassResponse from that location. The file is
    /// assumed to be a JSON and follow the structure of OverpassResponse.
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

    /// Behaves the same as [`OverpassResponse::load`], but will wait for the function to finish before continuing.
    pub fn load_blocking(filepath: &str) -> Result<Self, &'static str> {
        Runtime::new()
            .expect("Could not create runtime!")
            .block_on(Self::load(filepath))
    }
}

/// Requests data from the Overpass API given a particular query. The query must conform to the
/// Overpass Query Language.
pub async fn osm_request(query: String) -> Result<String, &'static str> {

    let url = "https://overpass-api.de/api/interpreter";
    
    let client = reqwest::Client::new();
    let response = client.post(url)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(format!("data={}", query))
        .send()
        .await
        .expect("Could not contact OverpassAPI server!");

    // Parse the response as JSON
    let json_string: String = response.text()
        .await
        .expect("Could not retrieve text from json!");

    Ok(json_string)
}

/// Behaves the same as [`osm_request`], but will wait for the function to finish before continuing.
pub fn osm_request_blocking(query: String) -> Result<String, &'static str> {
    Runtime::new()
        .expect("Could not create runtime!")
        .block_on(osm_request(query))
}
