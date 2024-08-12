use std::collections::HashMap;

use serde_json::Value;
use petgraph::{graph::UnGraph, adj::NodeIndex};

use crate::graph::{
    way::{OSMWay, get_osm_ways, filter_unconnected_nodes},
    node::{OSMNode, get_osm_nodes, node_dist}
};

pub fn create_graph(elements: &Vec<Value>) -> Result<UnGraph<OSMNode, OSMWay>, &'static str> {

    //Parse out all of the nodes and ways
    let nodes: Vec<OSMNode> = get_osm_nodes(elements)?;
    let ways: Vec<OSMWay> = get_osm_ways(elements)?;

    //Filter nodes down to the ones that are on ways.
    //TODO: In the future, we may want to use more than just that
    let nodes: Vec<OSMNode> = filter_unconnected_nodes(&ways, nodes);

    //TODO: In the future, this should support one way streets (directed graph).
    let mut result = UnGraph::<OSMNode, OSMWay>::with_capacity(nodes.len(), ways.len());

    //Petgraph has its own notion of an index so we want to map from the
    //OSM index to the petgraph one so we can add ways later on
    let mut node_mapping: HashMap<u64, NodeIndex> = HashMap::new();

    for node in nodes {

        node_mapping.insert(
            node.id,
            result.add_node(node.clone()).index().try_into().unwrap()
        );
    }

    //Iterate through every way
    for mut way in ways {

        //Iterate through all of the connections in way
        for i in 0..way.nodes.len()-1 {

            let node_id_1: u64 = way.nodes[i];
            let node_id_2: u64 = way.nodes[i+1];

            let node_index_1: NodeIndex = *node_mapping
                .get(&node_id_1)
                .ok_or("Node mapping contained no node!")
                .unwrap();

            let node_index_2: NodeIndex = *node_mapping
                .get(&node_id_2)
                .ok_or("Node mapping contained no node!")
                .unwrap();

            let n1: &OSMNode = result.node_weight(node_index_1.into())
                .ok_or("Could not find node index!")?;
            let n2: &OSMNode = result.node_weight(node_index_2.into())
                .ok_or("Could not find node index!")?;

            //Add the distance between the nodes
            way.dists.push(
                node_dist(n1, n2)
            );

            //TODO: Instead, create an edge type that doesn't copy the whole way object. Copying
            //the way object is expensive!

            result.add_edge(
                node_index_1.into(),
                node_index_2.into(),
                way.clone()
            );
        }
    }

    Ok(result)
}
