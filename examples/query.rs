use osmgraph::api::{QueryEngine, OverpassResponse};

use serde_json::Value;

fn main() {

    //Create a query string in the format of the Overpass Query Language
    let response = QueryEngine::new()
        .query_blocking(r#"
            [out:json];
            area[name="Manhattan"][admin_level=7]->.searchArea;

            (
              way["highway"~"motorway|trunk|primary|secondary|tertiary|unclassified|service|residential"](area.searchArea);
            );

            //Get nodes associated with ways defined before
            (._; >;);

            out body; >;
            out skel qt;"#.to_string()
        ).expect("Could not query OSM!");

    println!("Request complete!");

    //Get json structure from the response string
    let json: OverpassResponse = serde_json::from_str(&response)
        .expect("Was not able to parse json!");
    
    println!("Parsed the json!");

    //Get the elements
    let elements: &Vec<Value> = json.elements().as_array()
        .expect("Was not able to fetch elements from json!");

    println!("{} elements in request", elements.len());
}
