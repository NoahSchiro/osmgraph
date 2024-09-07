use osmgraph::api::{QueryEngine, OverpassResponse, Element};

fn main() {

    //Create a query string in the format of the Overpass Query Language
    let response: String = QueryEngine::new()
        .query_blocking(r#"
            [out:json];
            area[name="Selinsgrove"][admin_level=8]->.searchArea;

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
    let elements: &Vec<Element> = json.elements();

    println!("{} elements in request", elements.len());
}
