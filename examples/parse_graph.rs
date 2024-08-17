use osm_graph::overpass_api::OverpassResponse;
use osm_graph::graph::{OSMGraph, OSMNode, OSMEdge, create_graph};

use serde_json::Value;
use petgraph::stable_graph::DefaultIx;
use petgraph::graph::{UnGraph, Edge};

//For plotting:
use plotters::prelude::*;

fn display(image_location: &str, graph: OSMGraph) -> Result<(), Box<dyn std::error::Error>> {

    let nodes: Vec<OSMNode> = graph
        .raw_nodes()
        .iter().map(|node| node.weight.clone())
        .collect();

    let min_lat: f64 = nodes.iter().map(|node| node.lat()).min_by(|a, b| a.total_cmp(b)).unwrap();
    let max_lat: f64 = nodes.iter().map(|node| node.lat()).max_by(|a, b| a.total_cmp(b)).unwrap();
    let min_lon: f64 = nodes.iter().map(|node| node.lon()).min_by(|a, b| a.total_cmp(b)).unwrap();
    let max_lon: f64 = nodes.iter().map(|node| node.lon()).max_by(|a, b| a.total_cmp(b)).unwrap();
    
    // Create a new 2000x2000 image
    let root = BitMapBackend::new(image_location, (2000, 2000)).into_drawing_area();
    root.fill(&WHITE)?;

    // Set up the chart with latitude and longitude as axes
    let mut chart = ChartBuilder::on(&root)
        .caption("Map", ("sans-serif", 50))
        .margin(20)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(min_lon..max_lon, min_lat..max_lat)?;

    chart.configure_mesh()
        .x_labels(10)
        .y_labels(10)
        .disable_mesh()
        .draw()?;

    // Plot the nodes as points
    chart.draw_series(
        nodes.iter().map(|node| {
            Circle::new((node.lon(), node.lat()), 1, ShapeStyle::from(&RED).filled())
        }),
    )?;

    let edges: &[Edge<OSMEdge, DefaultIx>] = graph.raw_edges();

    // Draw the edges as lines between points
    chart.draw_series(
        edges.iter().map(|edge| {

            let source: &OSMNode = graph
                .node_weight(edge.source())
                .ok_or("Could not find node that edge references")
                .expect("Could not find node that edge references");
            
            let target: &OSMNode = graph
                .node_weight(edge.target())
                .ok_or("Could not find node that edge references")
                .expect("Could not find node that edge references");

            PathElement::new(
                vec![(source.lon(), source.lat()), (target.lon(), target.lat())],
                &BLACK,
            )
        }),
    )?;
    
    // Write to file
    root.present()?;

    Ok(())
}

fn main() {

    let image_save_location = "./map.png";
    let graph_save_location = "./assets/test.json";

    let json: OverpassResponse = OverpassResponse::load_blocking(graph_save_location)
        .expect("Was not able to load json!");
    println!("Parsed the json!");

    let elements: &Vec<Value> = json.elements().as_array()
        .expect("Was not able to make request!");
    println!("{} elements in request", elements.len());

    let g: OSMGraph = create_graph(elements)
        .expect("Was not able to create graph from json!");

    println!("Created graph with {} nodes and {} edges",
        g.node_count(),
        g.edge_count(),
    );

    println!("Example node:\n {}", g.raw_nodes()[0].weight);
    println!("Example edge:\n {}", g.raw_edges()[0].weight);

    //Now that we have created the graph, let's show it
    println!("Displaying to {}", image_save_location);
    display(image_save_location, g).expect("Couldn't display graph!");


}
