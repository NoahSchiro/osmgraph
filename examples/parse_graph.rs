use osm_graph::overpass_api::OverpassResponse;
use osm_graph::graph::create_graph;

use serde_json::Value;

//Benchmarking
use std::time::Instant;

fn main() {

    let parse_json_time = Instant::now();
    let json: OverpassResponse = OverpassResponse::load_blocking("./assets/selinsgrove.json")
        .expect("Was not able to load json!");
    println!("Parsed the json in {} milliseconds", parse_json_time.elapsed().as_millis());

    let elements: &Vec<Value> = json.elements().as_array().unwrap();
    println!("{} elements in request", elements.len());

    let create_graph_time = Instant::now();
    let g = create_graph(elements).unwrap();

    println!("Created graph with {} nodes and {} edges in {} milliseconds",
        g.node_count(),
        g.edge_count(),
        create_graph_time.elapsed().as_millis()
    );

    println!("Example node:\n {}", g.raw_nodes()[0].weight);
    println!("Example edge:\n {}", g.raw_edges()[0].weight);
}
