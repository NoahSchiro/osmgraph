use osm_graph::overpass_api::{osm_request_blocking, OverpassResponse};

use serde_json::Value;

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

    let response: String = osm_request_blocking(query).unwrap();

    println!("Request complete!");

    let json: OverpassResponse = serde_json::from_str(&response).unwrap();
    
    println!("Parsed the json!");

    let elements: &Vec<Value> = json.elements().as_array().unwrap();

    println!("{} elements in request", elements.len());
}
