use osmgraph::api::{OverpassResponse, Element};
use osmgraph::graph::{OSMNode, get_osm_nodes};

fn main() {

    //Get json structure from disk
    let json: OverpassResponse = OverpassResponse::load_blocking("./assets/test.json")
        .expect("Was not able to load json!");
    println!("Parsed the json!");

    //Get the elements
    let elements: &Vec<Element> = json.elements();
    println!("{} elements in request", elements.len());

    //Get the OSMNodes from the elements
    let osm_nodes: Vec<OSMNode> = get_osm_nodes(elements)
        .expect("Was not able to get nodes from json!");
    println!("{} nodes parsed!", osm_nodes.len());
}
