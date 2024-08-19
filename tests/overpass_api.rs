#[cfg(test)]
mod query {

    use osmgraph::overpass_api::QueryEngine;

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

        assert!(response.len() > 0)
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

        assert!(response.len() > 0)
    }
}

#[cfg(test)]
mod parse {

    use osmgraph::overpass_api::{QueryEngine, OverpassResponse};
    
    use serde_json::json;

    #[tokio::test]
    async fn parse() {

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

        let json: OverpassResponse = serde_json::from_str(&response).unwrap();

        assert!(*json.elements()  != json!(null));
        assert!(*json.generator() != json!(null));
        assert!(*json.osm3s()     != json!(null));
        assert!(*json.version()   != json!(null));
    }
}

#[cfg(test)]
mod save_load {

    use osmgraph::overpass_api::{QueryEngine, OverpassResponse};

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

        let json: OverpassResponse = serde_json::from_str(&response).unwrap();

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
        
        let json: OverpassResponse = serde_json::from_str(&response).unwrap();

        json.save_blocking("./assets/test.json")
            .expect("Was not able to save json!");

        let _: OverpassResponse = OverpassResponse::load_blocking("./assets/test.json")
            .expect("Was not able to load json!");
    }
}
