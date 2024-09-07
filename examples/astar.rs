use std::collections::HashSet;
use std::error::Error;

use osmgraph::api::{QueryEngine, OverpassResponse};
use osmgraph::graph::{OSMGraph, OSMNode, OSMEdge, create_graph};

use serde_json::Value;
use petgraph::graph::{Edge, NodeIndex};
use petgraph::algo::astar;

//For plotting:
use plotters::prelude::*;
use rand::seq::IteratorRandom;

// This takes a graph and displays it as an image using the plotters library
fn display(image_location: &str, graph: OSMGraph, path: Vec<NodeIndex>) -> Result<(), Box<dyn std::error::Error>> {

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

    //Determine the minimum and maximum nodes so we know how big to make the image
    let min_lat: f64 = nodes.iter().map(|node| node.lat()).min_by(|a, b| a.total_cmp(b)).unwrap();
    let mut max_lat: f64 = nodes.iter().map(|node| node.lat()).max_by(|a, b| a.total_cmp(b)).unwrap();
    let d_lat: f64 = max_lat - min_lat;
    let min_lon: f64 = nodes.iter().map(|node| node.lon()).min_by(|a, b| a.total_cmp(b)).unwrap();
    let mut max_lon: f64 = nodes.iter().map(|node| node.lon()).max_by(|a, b| a.total_cmp(b)).unwrap();
    let d_lon: f64 = max_lon - min_lon;

    //We want to maintain the same ratio so that the map is not streched. This will make sure that
    //one pixel on the x-axis is the same length as a pixel on the y-axis.
    match d_lat > d_lon {
        true  => max_lon = min_lon + d_lat,
        false => max_lat = min_lat + d_lon
    }

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

    //No grid on the image
    chart.configure_mesh()
        .x_labels(10)
        .y_labels(10)
        .disable_mesh()
        .draw()?;

    // Plot the nodes as blue points
    chart.draw_series(
        nodes.iter().map(|node| {
            Circle::new((node.lon(), node.lat()), 1, ShapeStyle::from(&BLUE))
        }),
    )?;

    let edges: &[Edge<OSMEdge>] = graph.raw_edges();

    //Draw the edges as red lines between points if they are on
    //the path, otherwise, as black lines
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
                    true => ShapeStyle::from(&RED).stroke_width(5),
                    false => ShapeStyle::from(&BLACK),
                },
            )
        }),
    )?;
    
    // Write to file
    root.present()?;

    Ok(())
}

fn query_and_save(filepath: &str) -> Result<OverpassResponse, Box<dyn Error>> {

    println!("Could not find file... querying...");

    //Create a query string in the format of the Overpass Query Language
    //This is the more manual way (but with greater levels of control!
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
    let json: OverpassResponse = serde_json::from_str(&response)?;
    let _ = json.save_blocking(filepath)?;
 
    Ok(json)
}

fn main() {

    //Vars to change
    let image_save_location = "./map.png";
    let graph_save_location = "./assets/manhattan_test.json";

    //Get json structure from disk
    let json: OverpassResponse = OverpassResponse::load_blocking(graph_save_location)
        .unwrap_or_else(|_|
            query_and_save(graph_save_location).expect("Was not able to query!")
        );
    println!("Parsed the json!");

    //Get the elements
    let elements: &Vec<Value> = json.elements().as_array()
        .expect("Was not able to get elements from json!");
    println!("{} elements in request", elements.len());

    //Get the graph from the elements
    let g = create_graph(elements)
        .expect("Was not able to create graph from json!");

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
            |finish| finish == end, //Goal condition
            |e| e.weight().dist(),  //How to compute the weight of each edge
            |_| 0.                  //Estimate cost
        );

        //See if there is a path
        match result {
            Some((pl, p)) => break (pl, p),
            None => println!(
                "Was not able to find a path between {} and {}",
                start.index(), end.index()
            )
        }
    };

    println!("Found a path of {:.2} meters", path_length);

    //Now that we have created the graph, let's show it
    println!("Displaying to {}", image_save_location);
    display(image_save_location, g, path).expect("Couldn't display graph!");
}
