#[cfg(test)]
mod create_graph {

    use osmgraph::api::{OverpassResponse, Element};
    use osmgraph::graph::create_graph;


    #[test]
    fn test_create_graph() {

        let json: OverpassResponse = OverpassResponse::load_blocking("./assets/test.json")
            .expect("Was not able to load json!");


        let elements: &Vec<Element> = json.elements();

        let graph = create_graph(elements)
            .expect("Was unable to parse graph!");

        assert!(graph.node_count() > 0);
        assert!(graph.edge_count() > 0);

        assert!(graph.raw_nodes()[0].weight.id() != 0);
        assert!(graph.raw_edges()[0].weight.highway_type() != "");
    }
}
