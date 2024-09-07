use osmgraph::api::{OverpassResponse, Element};
use osmgraph::graph::way::{OSMWay, get_osm_ways};


fn main() {

    //Get json structure from disk
    let json: OverpassResponse = OverpassResponse::load_blocking("./assets/test.json")
        .expect("Was not able to load json!");
    println!("Parsed the json...");

    //Get the elements
    let elements: &Vec<Element> = json.elements();
    println!("{} elements in request", elements.len());

    //Get the OSMWay from the elements
    let osm_ways: Vec<OSMWay> = get_osm_ways(elements)
        .expect("Was not able to parse ways from json!");
    println!("{} ways parsed", osm_ways.len());
}
