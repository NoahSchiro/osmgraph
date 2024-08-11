use core::fmt;
use std::collections::{HashSet, HashMap};

use serde_json::Value;
use petgraph::{graph::UnGraph, adj::NodeIndex};

#[derive(Clone)]
pub struct OSMNode {
    pub id: u64,
    pub lat: f64,
    pub lon: f64
}

impl fmt::Display for OSMNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "OSMNode(id: {}, lat: {}, lon: {})", self.id, self.lat, self.lon)
    }
}

#[derive(Clone)]
pub struct OSMWay {
    pub id: u64,
    pub nodes: Vec<u64>,
    pub dists: Vec<f64>,
    pub highway_type: String
}

//Haversine dist in meters given lat/lons in radians
fn haversine_dist(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let dlat = lat2 - lat1;
    let dlon = lon2 - lon1;

    let a = (dlat / 2.).sin().powi(2) + lat1.cos() * lat2.cos() * (dlon / 2.).sin().powi(2);
    let c = 2. * a.sqrt().atan2((1. - a).sqrt());
    let earth_radius = 6371. * 1000.;

    earth_radius * c
}

//Get the distance between two nodes
fn node_dist(n1: &OSMNode, n2: &OSMNode) -> f64 {
    haversine_dist(
        n1.lat.to_radians(), n1.lon.to_radians(),
        n2.lat.to_radians(), n2.lon.to_radians()
    )
}

pub fn get_osm_nodes(elements: &Vec<Value>) -> Result<Vec<OSMNode>, &'static str> {

    //Only get OSM elements that are nodes
    let node_elements: Vec<Value> = elements.clone().into_iter()
        .filter(|e| {
            match e.get("type") {
                Some(t) => t == "node",
                None => false
            }
        })
        .collect();

    //Result to collect into
    let mut result: Vec<OSMNode> = vec![];

    for elem in node_elements {

        let id: u64 = if let Some(x) = elem.get("id").cloned() {
            x.as_u64().expect("Could not parse to u64!")
        } else {
            continue; //Node is invalid if it has no ID
        };

        let lat: f64 = if let Some(x) = elem.get("lat").cloned() {
            x.as_f64().expect("Could not parse to f64!")
        } else {
            continue; //Node is invalid if it has no lat 
        };

        let lon: f64 = if let Some(x) = elem.get("lon").cloned() {
            x.as_f64().expect("Could not parse to f64!")
        } else {
            continue; //Node is invalid if it has no lon
        };

        result.push(
            OSMNode {
                id, lat, lon
            }
        );
    }

    //Return
    Ok(result)
}

pub fn get_osm_ways(elements: &Vec<Value>) -> Result<Vec<OSMWay>, &'static str> {

    //Only get OSM elements that are nodes
    let way_elements: Vec<Value> = elements.clone().into_iter()
        .filter(|e| {
            match e.get("type") {
                Some(t) => t == "way",
                None => false
            }
        })
        .collect();

    let mut result: Vec<OSMWay> = Vec::new();

    for e in way_elements {

        //If not tags, skip
        let tags: &Value = if let Some(tags) = e.get("tags") {
            tags
        } else {
            continue;
        };

        //If not a highway, then skip
        let highway_type: String = if let Some(highway_type) = tags.get("highway") {
            highway_type
                .as_str()
                .expect("Could not parse highway type into str!")
                .to_string()
        } else {
            continue;
        };

        //Get id
        let id: u64 = e.get("id")
            .ok_or("Way did not contain id!")?
            .as_u64()
            .expect("Could not parse into u64");

        let nodes: Vec<u64> = e.get("nodes")
            .ok_or("Way did not contain nodes!")?
            .as_array()
            .expect("Could not parse nodes into vec!")
            .iter()
            .map(|x| 
                x.as_u64().expect("Could not parse node id into u64!")
            ).collect();

        //Add to list
        result.push(OSMWay {
            id,
            nodes,
            // We can only compute distance if we have access to the nodes as well
            // Leave this blank at the moment
            dists: vec![], 
            highway_type
        });

    }

    Ok(result)
}

//Given a set of ways, only collect nodes that lie on a way
pub fn filter_unconnected_nodes(ways: &Vec<OSMWay>, nodes: Vec<OSMNode>) -> Vec<OSMNode> {

    //Create set of node ids
    let mut node_ids: HashSet<u64> = HashSet::new();
    for way in ways {
        for id in way.nodes.clone() {
            node_ids.insert(id);
        }
    }

    //Filter anything not in the hashset
    nodes
        .into_iter()
        .filter(|node| node_ids.contains(&node.id))
        .collect()
}

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
