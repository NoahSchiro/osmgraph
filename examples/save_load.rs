use osm_graph::overpass_api::OverpassResponse;

fn main() {

    //Vars to change
    let location = "./assets/manhattan_test.json";

    //Get json structure from disk
    let loaded_json: OverpassResponse = OverpassResponse::load_blocking(location)
        .expect("Was unable to load json!");
    println!("Loaded json!");

    //Save the json to disk
    match loaded_json.save_blocking("./assets/manhattan_test.json") {
        Ok(..) => println!("Saved json to {}!", location),
        Err(err) => println!("{}", err)
    }
}
