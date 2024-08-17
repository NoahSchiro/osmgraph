use osm_graph::overpass_api::OverpassResponse;
use osm_graph::graph::{OSMNode, get_osm_nodes};

use serde_json::Value;

fn main() {

    let json: OverpassResponse = OverpassResponse::load_blocking("./assets/test.json")
        .expect("Was not able to load json!");
    println!("Parsed the json!");

    let elements: &Vec<Value> = json.elements().as_array()
        .expect("Was not able to fetch elements from json!");
    println!("{} elements in request", elements.len());

    let osm_nodes: Vec<OSMNode> = get_osm_nodes(elements)
        .expect("Was not able to get nodes from json!");
    println!("{} nodes parsed!", osm_nodes.len());
}
