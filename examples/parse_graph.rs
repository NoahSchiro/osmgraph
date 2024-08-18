use osm_graph::overpass_api::OverpassResponse;
use osm_graph::graph::{OSMGraph, OSMNode, OSMEdge, create_graph};

use serde_json::Value;
use petgraph::stable_graph::DefaultIx;
use petgraph::graph::Edge;

//For plotting:
use plotters::prelude::*;

// This takes a graph and displays it as an image using the plotters library
fn display(image_location: &str, graph: OSMGraph) -> Result<(), Box<dyn std::error::Error>> {

    //Extract nodes from graph
    let nodes: Vec<OSMNode> = graph
        .raw_nodes()
        .iter().map(|node| node.weight.clone())
        .collect();

    //Determine the minimum and maximum nodes so we know how big to make the image
    // let min_lat: f64 = nodes.iter().map(|node| node.lat()).min_by(|a, b| a.total_cmp(b)).unwrap();
    // let mut max_lat: f64 = nodes.iter().map(|node| node.lat()).max_by(|a, b| a.total_cmp(b)).unwrap();
    // let d_lat: f64 = max_lat - min_lat;
    // let min_lon: f64 = nodes.iter().map(|node| node.lon()).min_by(|a, b| a.total_cmp(b)).unwrap();
    // let mut max_lon: f64 = nodes.iter().map(|node| node.lon()).max_by(|a, b| a.total_cmp(b)).unwrap();
    // let d_lon: f64 = max_lon - min_lon;
    
    //We want to maintain the same ratio so that the map is not streched. This will make sure that
    //one pixel on the x-axis is the same length as a pixel on the y-axis.
    // match d_lat > d_lon {
    //     true  => max_lon = min_lon + d_lat,
    //     false => max_lat = min_lat + d_lon
    // }

    //Sometimes, like for manhattan, the nodes are ALL OVER THE WORLD so it doesn't show up
    //properly on the map. Here, I will just manually set where the map needs to be.
    let (min_lat, max_lat) = (40.68,  40.68  + 0.2);
    let (min_lon, max_lon) = (-74.075, -74.075 + 0.25);

    // Create a new 3000x3000 image
    let root = BitMapBackend::new(image_location, (3000, 3000)).into_drawing_area();
    root.fill(&WHITE)?;

    // Set up the chart with latitude and longitude as axes
    let mut chart = ChartBuilder::on(&root)
        .caption("Map", ("sans-serif", 50))
        .margin(20)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(min_lon..max_lon, min_lat..max_lat)?;

    //No grid
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

    //Vars to change
    let image_save_location = "./map.png";
    let graph_save_location = "./assets/manhattan_test.json";

    //Get json structure from disk
    let json: OverpassResponse = OverpassResponse::load_blocking(graph_save_location)
        .expect("Was not able to load json!");
    println!("Parsed the json!");

    //Get the elements
    let elements: &Vec<Value> = json.elements().as_array()
        .expect("Was not able to make request!");
    println!("{} elements in request", elements.len());

    //Get the graph from the elements
    let g: OSMGraph = create_graph(elements)
        .expect("Was not able to create graph from json!");

    println!("Created graph with {} nodes and {} edges",
        g.node_count(),
        g.edge_count()
    );

    println!("Example node:\n {}", g.raw_nodes()[0].weight);
    println!("Example edge:\n {}", g.raw_edges()[0].weight);

    //Now that we have created the graph, let's show it
    println!("Displaying to {}", image_save_location);
    display(image_save_location, g).expect("Couldn't display graph!");


}
