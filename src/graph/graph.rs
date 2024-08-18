use std::collections::HashMap;
use std::error::Error;

use serde_json::Value;
use petgraph::{graph::UnGraph, adj::NodeIndex};

use super::{
    way::{OSMWay, get_osm_ways},
    node::{OSMNode, node_dist, get_nodes_from_ways},
    edge::OSMEdge
};

/// `OSMGraph` is just a type redefinition of `UnGraph<OSMNode, OSMEdge>`
pub type OSMGraph = UnGraph<OSMNode, OSMEdge>;

/// Given a json type structure, this function tries to parse an `OSMGraph` out of that json.
pub fn create_graph(elements: &Vec<Value>) -> Result<OSMGraph, Box<dyn Error>> {

    //Parse out all of the nodes and ways
    let ways: Vec<OSMWay> = get_osm_ways(elements)?;
    let nodes: Vec<OSMNode> = get_nodes_from_ways(elements, &ways)?;

    let mut result = UnGraph::<OSMNode, OSMEdge>::with_capacity(nodes.len(), ways.len());

    //Petgraph has its own notion of an index so we want to map from the
    //OSM index to the petgraph one so we can add ways later on
    let mut node_mapping: HashMap<u64, NodeIndex> = HashMap::new();

    //Add nodes to mapping and to graph
    for node in nodes {
        node_mapping.insert(
            node.id(),
            result.add_node(node.clone()).index().try_into()?
        );
    }

    //Iterate through every way
    for way in ways {

        let nodes = way.nodes();

        //Iterate through all of the connections in way
        for i in 0..nodes.len()-1 {

            //Get OSM node ID
            let node_id_1: u64 = nodes[i];
            let node_id_2: u64 = nodes[i+1];

            //Find petgraph node ID
            let node_index_1: NodeIndex = *node_mapping
                .get(&node_id_1)
                .ok_or("Node mapping contained no node!")?;

            let node_index_2: NodeIndex = *node_mapping
                .get(&node_id_2)
                .ok_or("Node mapping contained no node!")?;

            //Get nodes out of petgraph
            let n1: &OSMNode = result.node_weight(node_index_1.into())
                .ok_or("Could not find node index!")?;
            let n2: &OSMNode = result.node_weight(node_index_2.into())
                .ok_or("Could not find node index!")?;

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
