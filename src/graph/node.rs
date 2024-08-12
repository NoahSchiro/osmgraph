use core::fmt;
use std::collections::HashSet;

use serde_json::Value;

use crate::graph::way::OSMWay;

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
pub(super) fn node_dist(n1: &OSMNode, n2: &OSMNode) -> f64 {
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

pub fn get_nodes_from_ways(elements: &Vec<Value>, ways: &Vec<OSMWay>)
    -> Result<Vec<OSMNode>, &'static str> { 

    //Create set of node ids
    let mut node_ids: HashSet<u64> = HashSet::new();
    for way in ways {
        for id in way.nodes.clone() {
            node_ids.insert(id);
        }
    }

    //Only get OSM elements that are nodes
    let node_elements: Vec<Value> = elements.clone().into_iter()

        //Filter to only the node elements
        .filter(|e| {
            match e.get("type") {
                Some(t) => t == "node",
                None => false
            }
        })

        //Filter to node elements referenced in way element
        .filter(|e| {
            match e.get("id") {
                Some(id) => node_ids.contains(
                    &id.as_u64().expect("Could not parse id into u64!")
                ),
                None => false
            }
        })
        .collect();

    //Result to collect into
    let mut result: Vec<OSMNode> = vec![];

    for elem in node_elements {

        let id: u64 = if let Some(x) = elem.get("id").cloned() {
            x.as_u64().expect("Could not parse to node id into u64!")
        } else {
            continue; //Node is invalid if it has no ID
        };

        let lat: f64 = if let Some(x) = elem.get("lat").cloned() {
            x.as_f64().expect("Could not parse node latitude into f64!")
        } else {
            continue; //Node is invalid if it has no lat 
        };

        let lon: f64 = if let Some(x) = elem.get("lon").cloned() {
            x.as_f64().expect("Could not parse node longitude into f64!")
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
