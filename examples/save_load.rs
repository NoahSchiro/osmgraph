use std::time::Instant;

use osm_graph::overpass_api::{OverpassResponse, osm_request_blocking};

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
    let response: String = osm_request_blocking(query).unwrap();
    let request_time = start.elapsed().as_secs();

    println!("Request took {} seconds", request_time);

    let parse_json_time = Instant::now();
    let json: OverpassResponse = serde_json::from_str(&response).unwrap();
    let json_time = parse_json_time.elapsed().as_millis();

    println!("Parsed json in {} milliseconds", json_time);

    let save_json_time = Instant::now();
    match json.save_blocking("./assets/selinsgrove.json") {
        Ok(..) => println!("Saved in {} milliseconds", save_json_time.elapsed().as_millis()),
        Err(err) => println!("{}", err)
    }

    let load_json_time = Instant::now();
    let _: OverpassResponse = OverpassResponse::load_blocking("./assets/selinsgrove.json")
        .expect("Was unable to load json!");
    println!("Loaded json in {} milliseconds", load_json_time.elapsed().as_millis());
}
