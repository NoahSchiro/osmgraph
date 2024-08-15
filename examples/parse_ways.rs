use osm_graph::overpass_api::OverpassResponse;
use osm_graph::graph::way::{OSMWay, get_osm_ways};

use serde_json::Value;

//Benchmarking
use std::time::Instant;

fn main() {

    let parse_json_time = Instant::now();
    let json: OverpassResponse = OverpassResponse::load_blocking("./assets/selinsgrove.json")
        .expect("Was not able to load json!");
    println!("Parsed the json in {} milliseconds", parse_json_time.elapsed().as_millis());

    let elements: &Vec<Value> = json.elements.as_array().unwrap();
    println!("{} elements in request", elements.len());

    let create_way_time = Instant::now();
    let osm_ways: Vec<OSMWay> = get_osm_ways(elements).unwrap();
    println!("{} ways parsed in {} milliseconds", osm_ways.len(), create_way_time.elapsed().as_millis());
}
