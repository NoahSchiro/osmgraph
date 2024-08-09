use osm_graph::overpass_api::{osm_request, OverpassResponse};
use osm_graph::graph::{get_osm_nodes, OSMNode};

use serde_json::Value;

//Plotting, maybe temporary
use plotters::prelude::*;

//Benchmarking, maybe temporary
use std::time::Instant;

//Given a set of OSMNodes, plot them
fn plot(nodes: Vec<OSMNode>) -> Result<(), Box<dyn std::error::Error>> {

    let node_data: Vec<(f64, f64)> = nodes
        .iter()
        .map(|node| {
            (node.lat, node.lon)
        })
        .collect();

    let lats: Vec<f64> = node_data.iter().map(|x| x.0).collect();
    let lons: Vec<f64> = node_data.iter().map(|x| x.1).collect();

    let min_lat: f64 = *lats.iter().min_by(|a, b| a.total_cmp(b)).unwrap();
    let max_lat: f64 = *lats.iter().max_by(|a, b| a.total_cmp(b)).unwrap();
    let min_lon: f64 = *lons.iter().min_by(|a, b| a.total_cmp(b)).unwrap();
    let max_lon: f64 = *lons.iter().max_by(|a, b| a.total_cmp(b)).unwrap();

    // Create a drawing area
    let root = BitMapBackend::new("map.png", (1000, 1000)).into_drawing_area();
    root.fill(&WHITE)?;

    // Create the chart
    let mut chart = ChartBuilder::on(&root)
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(min_lat..max_lat, min_lon..max_lon)?;

    // Configure the mesh
    chart.configure_mesh().draw()?;

    // Draw the scatter plot
    chart.draw_series(node_data.iter().map(|point| {
        Circle::new(*point, 1, &RED.mix(0.5))
    }))?;

    Ok(())
}

fn main() {

    let query = String::from(r#"
        [out:json];
        area[name="Selinsgrove"]->.searchArea;
        (
          way(area.searchArea);
          node(area.searchArea);
        );
        out body;
        >;
        out skel qt;
    "#);

    let start = Instant::now();
    let response: String = osm_request(query).unwrap();
    let request_time = start.elapsed().as_secs();

    println!("Request took {} seconds", request_time);

    let parse_json_time = Instant::now();
    let json: OverpassResponse = serde_json::from_str(&response).unwrap();
    let json_time = parse_json_time.elapsed().as_millis();
    
    println!("Parsed the json in {} milliseconds", json_time);

    let elements: &Vec<Value> = json.elements.as_array().unwrap();

    println!("{} elements in request", elements.len());

    let create_node_time = Instant::now();
    let osm_nodes: Vec<OSMNode> = get_osm_nodes(elements).unwrap();
    let node_time = create_node_time.elapsed().as_millis();

    println!("{} nodes parsed in {} milliseconds", osm_nodes.len(), node_time);

    let _ = plot(osm_nodes);
}
