use osm_graph::overpass_api::OverpassResponse;
use osm_graph::graph::{OSMNode, get_osm_nodes};

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

    let create_node_time = Instant::now();
    let osm_nodes: Vec<OSMNode> = get_osm_nodes(elements).unwrap();
    println!("{} nodes parsed in {} milliseconds", osm_nodes.len(), create_node_time.elapsed().as_millis());
}
