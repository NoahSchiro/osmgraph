#[cfg(test)]
mod query {

    use osmgraph::api::QueryEngine;

    #[tokio::test]
    async fn query() {

        let engine = QueryEngine::new();

        let response: String = engine.query(r#"
            [out:json];
            area[name="Selinsgrove"]->.searchArea;
            (
              way(area.searchArea);
              node(area.searchArea);
            );
            out body;
            >;
            out skel qt;
        "#.to_string()
        ).await.expect("OSM request failed!");

        assert!(response.len() > 0);

        let next_response: String = engine
            .query_place("Selinsgrove".to_string(), None)
            .await
            .expect("OSM request failed!");

        assert!(next_response.len() > 0);

        let third_response: String = engine
            .query_place("Selinsgrove".to_string(), Some(8))
            .await
            .expect("OSM request failed!");

        assert!(third_response.len() > 0);

        let fourth_response: String = engine
            .query_poly(vec![
                (32.407, -64.896),
                (32.407, -64.630),
                (32.224, -64.630),
                (32.224, -64.896),
                (32.407, -64.896),
            ])
            .await
            .expect("OSM request failed!");

        assert!(fourth_response.len() > 0);
    }


    #[test]
    fn query_blocking() {

        let engine = QueryEngine::new();

        let response: String = engine.query_blocking(r#"
            [out:json];
            area[name="Selinsgrove"]->.searchArea;
            (
              way(area.searchArea);
              node(area.searchArea);
            );
            out body;
            >;
            out skel qt;
        "#.to_string()).expect("OSM request failed!");

        assert!(response.len() > 0);

        let next_response: String = engine
            .query_place_blocking("Selinsgrove".to_string(), None)
            .expect("OSM request failed!");

        assert!(next_response.len() > 0);

        let third_response: String = engine
            .query_place_blocking("Selinsgrove".to_string(), Some(8))
            .expect("OSM request failed!");

        assert!(third_response.len() > 0);

        let fourth_response: String = engine
            .query_poly_blocking(vec![
                (32.407, -64.896),
                (32.407, -64.630),
                (32.224, -64.630),
                (32.224, -64.896),
                (32.407, -64.896),
            ])
            .expect("OSM request failed!");

        assert!(fourth_response.len() > 0);
    }
}

#[cfg(test)]
mod parse {

    use osmgraph::api::{QueryEngine, OverpassResponse};
    
    use serde_json::json;

    #[tokio::test]
    async fn parse() {

        let engine = QueryEngine::new();

        let response: String = engine.query(r#"
            [out:json];
            area[name="Selinsgrove"][admin_level=8]->.searchArea;
            (
              way(area.searchArea);
              node(area.searchArea);
            );
            out body;
            >;
            out skel qt;
        "#.to_string()).await.expect("OSM request failed!");

        let json: OverpassResponse = serde_json::from_str(&response)
            .expect("Could not parse!");

        assert!(json.elements().len() > 0);
        assert!(*json.generator() != json!(null));
        assert!(*json.osm3s()     != json!(null));
        assert!(*json.version()   != json!(null));
    }
}

#[cfg(test)]
mod save_load {

    use osmgraph::api::{QueryEngine, OverpassResponse};

    #[tokio::test]
    async fn save_load() {

        let engine = QueryEngine::new();

        let response: String = engine.query(r#"
            [out:json];
            area[name="Selinsgrove"]->.searchArea;
            (
              way(area.searchArea);
              node(area.searchArea);
            );
            out body;
            >;
            out skel qt;
        "#.to_string()).await.expect("OSM request failed!");

        let json: OverpassResponse = serde_json::from_str(&response)
            .expect("Could not parse");

        json.save("./assets/test.json")
            .await
            .expect("Was not able to save json!");

        let _: OverpassResponse = OverpassResponse::load("./assets/test.json")
            .await
            .expect("Was not able to load json!");
    }
    
    #[test]
    fn save_load_blocking() {
        
        let engine = QueryEngine::new();

        let response: String = engine.query_blocking(r#"
            [out:json];
            area[name="Selinsgrove"]->.searchArea;
            (
              way(area.searchArea);
              node(area.searchArea);
            );
            out body;
            >;
            out skel qt;
        "#.to_string()).expect("OSM request failed!");
        
        let json: OverpassResponse = serde_json::from_str(&response)
            .expect("Could not parse");

        json.save_blocking("./assets/test.json")
            .expect("Was not able to save json!");

        let _: OverpassResponse = OverpassResponse::load_blocking("./assets/test.json")
            .expect("Was not able to load json!");
    }
}
