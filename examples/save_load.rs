use osmgraph::overpass_api::OverpassResponse;

fn main() {

    //Vars to change
    let location = "./assets/test.json";

    //Get json structure from disk
    let loaded_json: OverpassResponse = OverpassResponse::load_blocking(location)
        .expect("Was unable to load json!");
    println!("Loaded json!");

    //Save the json to disk
    match loaded_json.save_blocking("./assets/test.json") {
        Ok(..) => println!("Saved json to {}!", location),
        Err(err) => println!("{}", err)
    }
}
