use std::io::Error;
use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use serde_json::Value;

use tokio::{
    fs::File,
    io::{AsyncWriteExt, AsyncReadExt},
    runtime::Builder,
};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "lowercase")] 
#[serde(tag = "type")]
pub enum Element {
    Node {
        id: u64,
        lat: f64,
        lon: f64,
        tags: Option<HashMap<String, String>>
    }, 
    Way {
        id: u64,
        nodes: Vec<u64>,
        tags: Option<Value>,
    }
}

/// `OverpassResponse` is the basic structure that we expect the OSM to respond with.
/// Serde JSON helps us parse this string into the correct data structure.
///
/// Example:
/// ```rust
/// use osmgraph::api::{QueryEngine, OverpassResponse};
///
/// let engine = QueryEngine::new();
///
/// //Make the request
/// let response: String = engine.query_blocking(r#"
///     [out:json];
///     area[name="Selinsgrove"]->.searchArea;
///     (
///       way(area.searchArea);
///       node(area.searchArea);
///     );
///     out body;
///     >;
///     out skel qt;
/// "#.to_string()).expect("Was not able to request OSM!");
///
/// let json: OverpassResponse = serde_json::from_str(&response)
///     .expect("Was not able to parse json!");
/// ```
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Default)]
pub struct OverpassResponse {

    //Graph data
    elements: Vec<Element>,

    //Metadata
    generator: Value,
    osm3s: Value,
    version: Value
}

impl OverpassResponse {

    /// Return the `elements` field from the response. This field is the most important as it
    /// contains the actual graph information.
    pub fn elements(&self) -> &Vec<Element> {
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
    pub async fn save(&self, filepath: &str) -> Result<(), Error> {

        let list_as_json = serde_json::to_string(self)?;
        let mut file = File::create(filepath).await?;

        file.write_all(list_as_json.as_bytes()).await?;
        file.flush().await?;

        Ok(())
    }

    /// Behaves the same as [`OverpassResponse::save`], but will wait for the function to finish before continuing.
    pub fn save_blocking(&self, filepath: &str) -> Result<(), Error> {
        Builder::new_current_thread()
            .enable_all()
            .build()?
            .block_on(self.save(filepath))
    }

    /// Given a specified `filepath`, load the OverpassResponse from that location. The file is
    /// assumed to be a JSON and follow the structure of OverpassResponse.
    pub async fn load(filepath: &str) -> Result<Self, Error> {

        let mut file = File::open(filepath).await?;

        let mut contents = Vec::new();

        // Read the file's contents into the buffer
        file.read_to_end(&mut contents).await?;

        let contents_as_string: String = String::from_utf8_lossy(&contents).to_string();

        let json: OverpassResponse = serde_json::from_str(&contents_as_string)?;

        Ok(json)
    }

    /// Behaves the same as [`OverpassResponse::load`], but will wait for the function to finish before continuing.
    pub fn load_blocking(filepath: &str) -> Result<Self, Error> {
        Builder::new_current_thread()
            .enable_all()
            .build()?
            .block_on(Self::load(filepath))
    }
}
