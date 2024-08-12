use osm_graph::overpass_api::{osm_request, OverpassResponse};
use osm_graph::graph::way::{OSMWay, get_osm_ways};

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
    let response: String = osm_request(query).unwrap();
    let request_time = start.elapsed().as_secs();

    println!("Request took {} seconds", request_time);

    let parse_json_time = Instant::now();
    let json: OverpassResponse = serde_json::from_str(&response).unwrap();
    let json_time = parse_json_time.elapsed().as_millis();
    
    println!("Parsed the json in {} milliseconds", json_time);

    let elements: &Vec<Value> = json.elements.as_array().unwrap();

    println!("{} elements in request", elements.len());

    let create_way_time = Instant::now();
    let osm_ways: Vec<OSMWay> = get_osm_ways(elements).unwrap();
    let way_time = create_way_time.elapsed().as_millis();

    println!("{} ways parsed in {} milliseconds", osm_ways.len(), way_time);
}
