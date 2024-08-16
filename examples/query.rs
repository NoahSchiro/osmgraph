use osm_graph::overpass_api::{osm_request_blocking, OverpassResponse};

use serde_json::Value;

//Benchmarking, maybe temporary
use std::time::Instant;

fn main() {

    let query = String::from(r#"
        [out:json];
        area[name="Selinsgrove"]->.searchArea;
        (
          way(area.searchArea);
          node(area.searchArea);
        );
        out body;
        >;
        out skel qt;
    "#);

    let start = Instant::now();
    let response: String = osm_request_blocking(query).unwrap();
    let request_time = start.elapsed().as_secs();

    println!("Request took {} seconds", request_time);

    let parse_json_time = Instant::now();
    let json: OverpassResponse = serde_json::from_str(&response).unwrap();
    let json_time = parse_json_time.elapsed().as_millis();
    
    println!("Parsed the json in {} milliseconds", json_time);

    let elements: &Vec<Value> = json.elements().as_array().unwrap();

    println!("{} elements in request", elements.len());
}
