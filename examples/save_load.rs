use osm_graph::overpass_api::OverpassResponse;

fn main() {

    let location = "./assets/test.json";

    let loaded_json: OverpassResponse = OverpassResponse::load_blocking(location)
        .expect("Was unable to load json!");
    println!("Loaded json!");

    match loaded_json.save_blocking("./assets/test.json") {
        Ok(..) => println!("Saved json to {}!", location),
        Err(err) => println!("{}", err)
    }
}
