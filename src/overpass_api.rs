use std::io::{Error, ErrorKind};

use serde::{Serialize, Deserialize};
use serde_json::Value;

use tokio::{
    fs::File,
    io::{AsyncWriteExt, AsyncReadExt},
    runtime::Runtime
};

/// QueryEngine is a structure that helps create queries to the Overpass API.
/// It allows us to make lower level API calls (with the Overpass QL) as well as some higher level
/// API calls such as just fetching a place of interest or a polygon of interest.
#[derive(Clone, Debug, Default)]
pub struct QueryEngine {
    client: reqwest::Client,
    base_url: String,
    way_filters: Vec<String>,
}

impl QueryEngine {

    /// Creates a new instance of the query engine with the base url set to:
    ///
    /// <https://overpass-api.de/api/interpreter>
    ///
    /// QueryEngine also has a default set of filters for ways. The filter is currently set to only
    /// fetch roads that can be driven on with a car. However, you might be interested in
    /// footpaths, railroads, etc. If you would like to change the filter, take a look at
    /// <https://wiki.openstreetmap.org/wiki/Key:highway> for more information on the options
    /// available.
    ///
    /// Note that these filters are only applied when using higher level api calls such as
    /// [`Self::query_place`]. The lowest level api call, [`Self::query`] directly sends your query to the api
    /// without any modification.
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: "https://overpass-api.de/api/interpreter".to_string(),
            way_filters: vec![
                String::from("motorway"),
                String::from("trunk"),
                String::from("primary"),
                String::from("secondary"),
                String::from("tertiary"),
                String::from("unclassified"),
                String::from("residential"),
                String::from("service")
            ].into_iter().collect()
        }
    }

    /// Getter for the base URL that the QueryEngine uses.
    pub fn url(&self) -> &str {
        &self.base_url
    }

    /// Set a new url to query. Meant to be used in a functional style
    ///
    /// ```rust
    /// use osmgraph::overpass_api::QueryEngine;
    ///
    /// let engine = QueryEngine::new()
    ///     .with_url("www.url_example.com".to_string());
    /// ```
    pub fn with_url(&self, new_url: String) -> Self {
        Self {
            base_url: new_url,
            ..self.clone()
        }
    }

    /// Getter for the default way filters used in queries.
    pub fn filters(&self) -> &Vec<String> {
        &self.way_filters
    }

    /// Set new way filters in queries. Meant to be used in a functional style
    ///
    /// ```rust
    /// use osmgraph::overpass_api::QueryEngine;
    ///
    /// let engine = QueryEngine::new()
    ///     .with_filters(vec![String::from("motorway")]);
    /// ```
    pub fn with_filters(&self, new_filters: Vec<String>) -> Self {
        Self {
            way_filters: new_filters,
            ..self.clone()
        }
    }

    /// Given an area name, like "Manhattan" or "Germany", and an admin level, return the nodes and
    /// ways for that specific area.
    ///
    /// Admin level is a number that represents the scope of area you are interested in. In
    /// general:
    /// admin_level=2: Usually represents countries
    /// admin_level=4: Often represents states, provinces, or regions
    /// admin_level=6: May represent counties or districts
    /// admin_level=8: Often represents municipalities, cities, or towns
    /// admin_level=10: May represent neighborhoods or suburbs
    ///
    /// If you don't want to worry about admin levels, it is not required but will generally
    /// improve the results of your query.
    ///
    /// Example: 
    ///
    /// ```rust
    /// use osmgraph::overpass_api::{QueryEngine, OverpassResponse};
    ///
    /// let response: String = QueryEngine::new()
    ///     .query_place_blocking("Selinsgrove".to_string(), Some(7))
    ///     .expect("Could not query the server!");
    /// ```
    pub async fn query_place(&self, area_name: String, admin_level: Option<usize>) -> Result<String, Error> {

        let this_admin_level: String = match admin_level {
            Some(num) => format!("[admin_level={num}]"),
            None => "".to_string(),
        };

        let way_filter: String = match &self.way_filters.len() {
            0 => "(area.searchArea)".to_string(),
            _ => {
                format!("[\"highway\"~\"{}\"](area.searchArea)",
                    &self.way_filters.join("|"))
            }
        };

        //Return a query with the specified city name
        self.query(format!(r#"
            [out:json];
            area[name="{area_name}"]{this_admin_level}->.searchArea;

            //Find all ways according to filter
            (
              way{way_filter};
            );

            //Get nodes associated with ways defined before
            (._; >;);

            out body; >;

            out skel qt;"#
        )).await
    }

    /// This function does the same thing as [`Self::query_place`] but waits for the request to complete
    pub fn query_place_blocking(&self, area_name: String, admin_level: Option<usize>) -> Result<String, Error> {
        Runtime::new()?
            .block_on(self.query_place(area_name, admin_level))
    }

    /// Given a closed polygon, return all of the nodes and ways within that polygon.
    ///
    /// **Note**: the first and last element of the vector must be the same!
    ///
    /// Example: 
    ///
    /// ```rust
    /// use osmgraph::overpass_api::{QueryEngine, OverpassResponse};
    /// 
    /// //A big box
    /// let poly = vec![
    ///     (40.0, -76.0),
    ///     (41.0, -76.0),
    ///     (41.0, -75.0),
    ///     (40.0, -75.0),
    ///     (40.0, -76.0),
    /// ];
    ///
    /// let response: String = QueryEngine::new()
    ///     .query_poly_blocking(vec![
    ///         (32.407, -64.896),
    ///         (32.407, -64.630),
    ///         (32.224, -64.630),
    ///         (32.224, -64.896),
    ///         (32.407, -64.896),
    ///     ])
    ///     .expect("Could not query the server!");
    /// ```
    pub async fn query_poly(&self, polygon: Vec<(f64, f64)>) -> Result<String, Error> {

        assert!(polygon[0] == polygon[polygon.len()-1], "Beginning and end of polygon must be the same point!");

        let polyline_string: String = polygon
            .iter()
            .map(|(lat, lon)| format!("{lat} {lon}"))
            .collect::<Vec<String>>()
            .join(" ");

        let way_filter: String = match &self.way_filters.len() {
            0 => "".to_string(),
            _ => {
                format!("[\"highway\"~\"{}\"]",
                    &self.way_filters.join("|"))
            }
        };

        //Return a query with the specified city name
        self.query(format!(r#"
            [out:json];

            //Get the ways from the polygon
            way{way_filter}(poly:"{polyline_string}");

            //Get the nodes and anything else on the way
            (._; >;);

            out body; >;

            out skel qt;"#
        )).await
    }

    /// This function does the same thing as [`Self::query_poly`] but waits for the request to complete
    pub fn query_poly_blocking(&self, polygon: Vec<(f64, f64)>) -> Result<String, Error> {
        Runtime::new()?
            .block_on(self.query_poly(polygon))
    }

    /// Requests data from the Overpass API given a particular query. The query must conform to the
    /// Overpass Query Language.
    pub async fn query(&self, query: String) -> Result<String, Error> {

        let response = self.client
            .post(&self.base_url)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(format!("data={}", query))
            .send()
            .await
            .map_err(|e| Error::new(ErrorKind::Other, e))?;

        // Parse the response as JSON
        let json_string: String = response.text()
            .await
            .map_err(|e| Error::new(ErrorKind::Other, e))?;

        Ok(json_string)
    }

    /// Behaves the same as [`Self::query`], but will wait for the function to finish before continuing.
    pub fn query_blocking(&self, query: String) -> Result<String, Error> {
        Runtime::new()?
            .block_on(self.query(query))
    }
}

/// `OverpassResponse` is the basic structure that we expect the OSM to respond with.
/// Serde JSON helps us parse this string into the correct data structure.
///
/// Example:
/// ```rust
/// use osmgraph::overpass_api::{QueryEngine, OverpassResponse};
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
    pub async fn save(&self, filepath: &str) -> Result<(), Error> {

        let list_as_json = serde_json::to_string(self)?;
        let mut file = File::create(filepath).await?;

        file.write_all(list_as_json.as_bytes()).await?;
        file.flush().await?;

        Ok(())
    }

    /// Behaves the same as [`OverpassResponse::save`], but will wait for the function to finish before continuing.
    pub fn save_blocking(&self, filepath: &str) -> Result<(), Error> {
        Runtime::new()?
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
        Runtime::new()?
            .block_on(Self::load(filepath))
    }
}
