use osmgraph::{
    api::{QueryEngine, OverpassResponse},
    graph::{
        way::{OSMWay, get_osm_ways},
        create_graph,
        get_osm_nodes,
        get_nodes_from_ways
    }
};

use criterion::{criterion_group, criterion_main, Criterion};

pub fn small_map_parsing(c: &mut Criterion) {

    //Get json structure from disk
    let json: OverpassResponse = OverpassResponse::load_blocking("./assets/test.json")
        .expect("Was not able to load json!");

    //Get the elements
    let elements = json.elements();

    c.bench_function("node_parse", |b| b.iter(|| {
        get_osm_nodes(elements).expect("Was not able to get nodes from json!")
    }));

    let mut ways: Vec<OSMWay> = vec![];

    c.bench_function("way_parse", |b| b.iter(|| {
        ways = get_osm_ways(elements).expect("Was not able to get nodes from json!")
    }));

    c.bench_function("node_from_ways_parse", |b| b.iter(|| {
        get_nodes_from_ways(elements, &ways).expect("Was not able to get nodes from json!")
    }));

    c.bench_function("graph_parse", |b| b.iter(|| {
        create_graph(elements).expect("Was not able to get nodes from json!")
    }));
}

pub fn large_map_parsing(c: &mut Criterion) {
    
    let mut group = c.benchmark_group("large_map_parsing");
    group.sample_size(10);

    let large_graph_file_path = "./assets/manhattan_test.json";

    //Get json structure from disk
    let json: OverpassResponse = OverpassResponse::load_blocking(large_graph_file_path)
        .unwrap_or_else(|_| {
            let response = QueryEngine::new()
            .query_blocking(r#"
                [out:json];
                area[name="Manhattan"][admin_level=7]->.searchArea;
                (
                  way(area.searchArea);
                  node(area.searchArea);
                );
                out body;
                >;
                out skel qt;"#.to_string()
            ).expect("Could not query OSM!");

            //Get json structure from the response string and then save for the future
            let json: OverpassResponse = serde_json::from_str(&response)
                .expect("Was not able to parse json from response!");
            let _ = json.save_blocking(large_graph_file_path)
                .expect("Was not able to save json to file!");

            json
        });

    //Get the elements
    let elements = json.elements();

    group.bench_function("node_parse", |b| b.iter(|| {
        get_osm_nodes(elements).expect("Was not able to get nodes from json!")
    }));

    let mut ways: Vec<OSMWay> = vec![];

    group.bench_function("way_parse", |b| b.iter(|| {
        ways = get_osm_ways(elements).expect("Was not able to get nodes from json!")
    }));

    group.bench_function("node_from_ways_parse", |b| b.iter(|| {
        get_nodes_from_ways(elements, &ways).expect("Was not able to get nodes from json!")
    }));

    group.bench_function("graph_parse", |b| b.iter(|| {
        create_graph(elements).expect("Was not able to get nodes from json!")
    }));

    group.finish();
}

criterion_group!(benches, small_map_parsing, large_map_parsing);
criterion_main!(benches);
