use core::fmt;
use std::collections::HashSet;

use serde_json::Value;
use petgraph::graph::Graph;

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

#[allow(dead_code)]
pub struct OSMWay {
    pub id: u64,
    pub nodes: Vec<u64>,
    pub highway_type: String
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
        result.push(OSMWay { id, nodes, highway_type });

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

pub fn create_graph(elements: &Vec<Value>) -> Result<Graph<OSMNode, OSMWay>, &'static str> {

    let result = Graph::<OSMNode, OSMWay>::new();

    let nodes: Vec<OSMNode> = get_osm_nodes(elements)?;
    let ways: Vec<OSMWay> = get_osm_ways(elements)?;
    let nodes = filter_unconnected_nodes(&ways, nodes);




    Ok(result)
}
