use std::collections::HashSet;

use osm_graph::overpass_api::OverpassResponse;
use osm_graph::graph::{OSMNode, OSMEdge, create_graph};

use serde_json::Value;
use petgraph::graph::{UnGraph, Edge, NodeIndex};
use petgraph::algo::astar;

//For plotting:
use plotters::prelude::*;
use rand::seq::IteratorRandom;

fn display(image_location: &str, graph: UnGraph<OSMNode, OSMEdge>, path: Vec<NodeIndex>) -> Result<(), Box<dyn std::error::Error>> {

    //Get nodes
    let nodes: Vec<OSMNode> = graph
        .raw_nodes()
        .iter().map(|node| node.weight.clone())
        .collect();

    //Get edges on path
    let mut path_edges: HashSet<(NodeIndex, NodeIndex)> = HashSet::new();
    for i in 0..path.len()-1 {
        path_edges.insert((path[i], path[i+1]));
        path_edges.insert((path[i+1], path[i]));
    }

    let min_lat: f64 = nodes.iter().map(|node| node.lat()).min_by(|a, b| a.total_cmp(b)).unwrap();
    let max_lat: f64 = nodes.iter().map(|node| node.lat()).max_by(|a, b| a.total_cmp(b)).unwrap();
    let min_lon: f64 = nodes.iter().map(|node| node.lon()).min_by(|a, b| a.total_cmp(b)).unwrap();
    let max_lon: f64 = nodes.iter().map(|node| node.lon()).max_by(|a, b| a.total_cmp(b)).unwrap();
    
    // Create a new 2000x2000 image
    let root = BitMapBackend::new(image_location, (3000, 3000)).into_drawing_area();
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
            Circle::new((node.lon(), node.lat()), 1, ShapeStyle::from(&BLUE))
        }),
    )?;

    let edges: &[Edge<OSMEdge>] = graph.raw_edges();

    // Draw the edges as lines between points
    chart.draw_series(
        edges.iter().map(|edge| {

            let source_idx: NodeIndex = edge.source();
            let target_idx: NodeIndex = edge.target();

            let source: &OSMNode = graph
                .node_weight(source_idx)
                .ok_or("Could not find node that edge references")
                .expect("Could not find node that edge references");
            
            let target: &OSMNode = graph
                .node_weight(target_idx)
                .ok_or("Could not find node that edge references")
                .expect("Could not find node that edge references");

            PathElement::new(
                vec![(source.lon(), source.lat()), (target.lon(), target.lat())],

                //If on path, draw a different color and make it bold
                match path_edges.contains(&(source_idx, target_idx)) {
                    true => ShapeStyle::from(&RED).stroke_width(3),
                    false => ShapeStyle::from(&BLACK),
                },
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

    let elements: &Vec<Value> = json.elements().as_array().unwrap();
    println!("{} elements in request", elements.len());

    let g = create_graph(elements).unwrap();

    println!("Created graph with {} nodes and {} edges",
        g.node_count(),
        g.edge_count(),
    );

    println!("Example node:\n {}", g.raw_nodes()[0].weight);
    println!("Example edge:\n {}", g.raw_edges()[0].weight);

    //Because there might not be a path between any two given nodes, we will keep trying until we
    //find a path
    let (path_length, path) = loop {
        
        //Select two random nodes from the graph
        let sample: Vec<NodeIndex> = g.node_indices() 
            .choose_multiple(&mut rand::thread_rng(), 2);
        let (start, end) = (sample[0], sample[1]);

        let result: Option<(f64, Vec<NodeIndex>)> = astar(
            &g, start,              //Graph and start node
            |finish| finish == end, // Goal condition
            |e| e.weight().dist(),  // How to compute the weight of each edge
            |_| 0.                  //Estimate cost
        );

        //See if there is a path
        match result {
            Some((pl, p)) => break (pl, p),
            None => println!("Was not able to find a path between {} and {}", start.index(), end.index())

        }
    };

    println!("Found a path of {} meters", path_length);

    //Now that we have created the graph, let's show it
    println!("Displaying to {}", image_save_location);
    display(image_save_location, g, path).expect("Couldn't display graph!");
}
