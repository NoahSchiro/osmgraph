use std::collections::HashMap;
use std::error::Error;

use serde_json::Value;
use petgraph::{graph::UnGraph, adj::NodeIndex};

use super::{
    way::{OSMWay, get_osm_ways},
    node::{OSMNode, node_dist, get_osm_nodes},
    edge::OSMEdge
};

/// `OSMGraph` is just a type redefinition of `UnGraph<OSMNode, OSMEdge>`
pub type OSMGraph = UnGraph<OSMNode, OSMEdge>;

/// Given a json type structure, this function tries to parse an `OSMGraph` out of that json.
pub fn create_graph(elements: &Vec<Value>) -> Result<OSMGraph, Box<dyn Error>> {

    //Parse out all of the nodes and ways
    let ways: Vec<OSMWay> = get_osm_ways(elements)?;
    let nodes: Vec<OSMNode> = get_osm_nodes(elements)?;

    let mut result = UnGraph::<OSMNode, OSMEdge>::with_capacity(nodes.len(), ways.len());

    //Petgraph has its own notion of an index so we want to map from the
    //OSM index to the petgraph one so we can add ways later on
    let mut node_mapping: HashMap<u64, NodeIndex> = HashMap::with_capacity(nodes.len());

    //Add nodes to mapping and to graph
    for node in nodes {
        let petgraph_index = result.add_node(node).index() as u32;
        node_mapping.insert(
            node.id(),
            petgraph_index
        );
    }

    //Iterate through every way
    for way in ways {
        let nodes = way.nodes();

        //Iterate through all pairs of nodes in way
        for window in nodes.windows(2) {

            //Get OSM node ID
            let node_id_1: u64 = window[0];
            let node_id_2: u64 = window[1];

            //Find petgraph node ID
            let node_index_1: NodeIndex = node_mapping[&node_id_1];
            let node_index_2: NodeIndex = node_mapping[&node_id_2];

            //Get nodes out of petgraph
            let n1: &OSMNode = result.node_weight(node_index_1.into()).unwrap();
            let n2: &OSMNode = result.node_weight(node_index_2.into()).unwrap();

            //Insert edge into graph
            result.add_edge(
                node_index_1.into(), // Start node
                node_index_2.into(), // End node
                
                //Weight information
                OSMEdge::new([n1.id(), n2.id()], node_dist(n1,n2), way.highway_type().to_string())
            );
        }
    }

    //Return
    Ok(result)
}
