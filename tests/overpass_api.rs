#[cfg(test)]
mod query {

    use osmgraph::api::QueryEngine;

    #[tokio::test]
    async fn query() {

        let engine = QueryEngine::new();

        match engine.query(r#"
            [out:json][timeout:25];
            area[name="Selinsgrove"]->.searchArea;
            (
              way(area.searchArea);
              node(area.searchArea);
            );
            out body;
            >;
            out skel qt;
        "#.to_string()).await {
            // Since we are dealing with a third party service, sometimes it doesn't work
            Ok(x) => assert!(x.len() > 0),
            Err(_) => {
                eprintln!("Overpass was not responsive... Skipping");
            }
        }

        match engine.query_place(
            "Selinsgrove".to_string(), None
        ).await {
            // Since we are dealing with a third party service, sometimes it doesn't work
            Ok(x) => assert!(x.len() > 0),
            Err(_) => {
                eprintln!("Overpass was not responsive... Skipping");
            }
        }
            
        match engine.query_place(
            "Selinsgrove".to_string(), Some(8)
        ).await {
            // Since we are dealing with a third party service, sometimes it doesn't work
            Ok(x) => assert!(x.len() > 0),
            Err(_) => {
                eprintln!("Overpass was not responsive... Skipping");
            }
        }

        match engine.query_poly(vec![
            (32.407, -64.896),
            (32.407, -64.630),
            (32.224, -64.630),
            (32.224, -64.896),
            (32.407, -64.896),
        ]).await {
            // Since we are dealing with a third party service, sometimes it doesn't work
            Ok(x) => assert!(x.len() > 0),
            Err(_) => {
                eprintln!("Overpass was not responsive... Skipping");
            }
        }
    }


    #[test]
    fn query_blocking() {

        let engine = QueryEngine::new();

        match engine.query_blocking(r#"
            [out:json][timeout:25];
            area[name="Selinsgrove"]->.searchArea;
            (
              way(area.searchArea);
              node(area.searchArea);
            );
            out body;
            >;
            out skel qt;
        "#.to_string()) {
            // Since we are dealing with a third party service, sometimes it doesn't work
            Ok(x) => assert!(x.len() > 0),
            Err(_) => {
                eprintln!("Overpass was not responsive... Skipping");
            }
        }

        match engine.query_place_blocking("Selinsgrove".to_string(), None) {
            Ok(x) => assert!(x.len() > 0),
            Err(_) => {
                eprintln!("Overpass was not responsive... Skipping");
            }
        }

        match engine.query_place_blocking(
            "Selinsgrove".to_string(), Some(8)
        ) {
            Ok(x) => assert!(x.len() > 0),
            Err(_) => {
                eprintln!("Overpass was not responsive... Skipping");
            }
        }

        match engine.query_poly_blocking(vec![
            (32.407, -64.896),
            (32.407, -64.630),
            (32.224, -64.630),
            (32.224, -64.896),
            (32.407, -64.896),
        ]) {
            Ok(x) => assert!(x.len() > 0),
            Err(_) => {
                eprintln!("Overpass was not responsive... Skipping");
            }
        }
    }
}

#[cfg(test)]
mod parse {

    use osmgraph::api::{OverpassResponse};
    
    use serde_json::json;

    #[tokio::test]
    async fn parse() {

        let json: OverpassResponse = OverpassResponse::load("./assets/test.json")
            .await
            .expect("Was not able to load json!");

        assert!(json.elements().len() > 0);
        assert!(*json.generator() != json!(null));
        assert!(*json.osm3s()     != json!(null));
        assert!(*json.version()   != json!(null));
    }
}

#[cfg(test)]
mod save_load {

    use osmgraph::api::{OverpassResponse};

    #[tokio::test]
    async fn save_load() {

        let json: OverpassResponse = OverpassResponse::load("./assets/test.json")
            .await
            .expect("Was not able to load json!");

        json.save("./assets/test.json")
            .await
            .expect("Was not able to save json!");

    }
    
    #[test]
    fn save_load_blocking() {

        let json: OverpassResponse = OverpassResponse::load_blocking("./assets/test.json")
            .expect("Was not able to load json!");

        json.save_blocking("./assets/test.json")
            .expect("Was not able to save json!");
    }
}
