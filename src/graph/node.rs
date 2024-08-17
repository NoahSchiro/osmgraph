use core::fmt;
use std::collections::HashSet;

use serde_json::Value;

use crate::graph::way::OSMWay;

/// OSMNode contains all information that we might care about in a node. Currently, it contains a
/// node ID (as defined in Overpass API) a latitude and a longitude.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct OSMNode {
    id: u64,
    lat: f64,
    lon: f64
}

impl fmt::Display for OSMNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "OSMNode(id: {}, lat: {}, lon: {})", self.id, self.lat, self.lon)
    }
}

//Just getters and setters for now
impl OSMNode {

    /// Create a new OSMNode from fields.
    pub fn new(id: u64, lat: f64, lon: f64) -> Self {
        OSMNode { id, lat, lon }
    }

    /// Get the node ID.
    pub fn id(&self) -> u64 {
        self.id
    }
    /// Get the node latitude.
    pub fn lat(&self) -> f64 {
        self.lat
    }
    /// Get the node longitude. 
    pub fn lon(&self) -> f64 {
        self.lon
    }
}

/// Compute the [haversine distance](https://en.wikipedia.org/wiki/Haversine_formula)
/// (in meters) between two sets of coordinates, assuming those coordinates are in radians.
fn haversine_dist(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let dlat = lat2 - lat1;
    let dlon = lon2 - lon1;

    let a = (dlat / 2.).sin().powi(2) + lat1.cos() * lat2.cos() * (dlon / 2.).sin().powi(2);
    let c = 2. * a.sqrt().asin();
    let earth_radius = 6371009.;

    earth_radius * c
}

/// Get the distance between two nodes using the haversine distance.
pub(super) fn node_dist(n1: &OSMNode, n2: &OSMNode) -> f64 {
    haversine_dist(
        n1.lat.to_radians(), n1.lon.to_radians(),
        n2.lat.to_radians(), n2.lon.to_radians()
    )
}

/// Given a json type structure, this function tries to parse all `OSMNodes` out of that json.
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
            x.as_u64().unwrap_or_else(||
                panic!("Could not parse id {} as u64", x)
            )
        } else {
            continue; //Node is invalid if it has no ID
        };

        let lat: f64 = if let Some(x) = elem.get("lat").cloned() {
            x.as_f64().unwrap_or_else(||
                panic!("Could not parse latitude {} as f64", x)
            )
        } else {
            continue; //Node is invalid if it has no lat 
        };

        let lon: f64 = if let Some(x) = elem.get("lon").cloned() {
            x.as_f64().unwrap_or_else(||
                panic!("Could not parse longitude {} as f64", x)
            )
        } else {
            continue; //Node is invalid if it has no lon
        };

        result.push(
            OSMNode {
                id, lat, lon
            }
        );
    }

    Ok(result)
}

/// Given a set of nodes and ways, this function tries to parse all `OSMNodes` that lie
/// on one of the ways provided.
pub fn filter_unconnected_nodes(ways: &Vec<OSMWay>, nodes: Vec<OSMNode>) -> Vec<OSMNode> {

    //Create set of node ids
    let mut node_ids: HashSet<u64> = HashSet::new();
    for way in ways {
        for id in way.nodes().clone() {
            node_ids.insert(id);
        }
    }

    //Filter anything not in the hashset
    nodes
        .into_iter()
        .filter(|node| node_ids.contains(&node.id))
        .collect()
}

/// Given a json type structure and a `Vec<OSMWay>`, this function tries to
/// parse all `OSMNodes` out of that json if and only if the node lies on one of the ways provided.
pub fn get_nodes_from_ways(elements: &Vec<Value>, ways: &Vec<OSMWay>)
    -> Result<Vec<OSMNode>, &'static str> { 

    //Create set of node ids
    let mut node_ids: HashSet<u64> = HashSet::new();
    for way in ways {
        for id in way.nodes().clone() {
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
                    &id.as_u64().unwrap_or_else(||
                        panic!("Could not parse id {} into u64!", id)
                    )
                ),
                None => false
            }
        })
        .collect();

    //Result to collect into
    let mut result: Vec<OSMNode> = vec![];

    for elem in node_elements {

        let id: u64 = if let Some(x) = elem.get("id").cloned() {
            x.as_u64().unwrap_or_else(||
                panic!("Could not parse to id {} into u64!", x)
            )
        } else {
            continue; //Node is invalid if it has no ID
        };

        let lat: f64 = if let Some(x) = elem.get("lat").cloned() {
            x.as_f64().unwrap_or_else(||
                panic!("Could not parse latitude {} as f64", x)
            )
        } else {
            continue; //Node is invalid if it has no lat 
        };

        let lon: f64 = if let Some(x) = elem.get("lon").cloned() {
            x.as_f64().unwrap_or_else(||
                panic!("Could not parse latitude {} as f64", x)
            )
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


//This test is needed in this file since the havesine function is private to this module
#[cfg(test)]
mod haversine_tests {
    use super::*;
    
    // Floating point tolerance
    const EPSILON: f64 = 0.1;

    fn approx_equal(a: f64, b: f64, epsilon: f64) -> bool {
        (a - b).abs() < epsilon
    }

    #[test]
    fn test_zero_distance() {
        let (lat, lon): (f64, f64) = (52.5200, 13.4050);

        let dist = haversine_dist(
            lat.to_radians(),
            lon.to_radians(),
            lat.to_radians(),
            lon.to_radians()
        );
        assert!(approx_equal(dist, 0.0, EPSILON));
    }

    #[test]
    fn test_berlin_paris() {
        let (lat1, lon1): (f64, f64) = (52.5200, 13.4050);
        let (lat2, lon2): (f64, f64) = (48.8566, 2.3522);

        let dist = haversine_dist(
            lat1.to_radians(),
            lon1.to_radians(),
            lat2.to_radians(),
            lon2.to_radians()
        );
        assert!(approx_equal(dist, 877464.57, EPSILON));
    }

    #[test]
    fn test_new_york_los_angeles() {
        let (lat1, lon1): (f64, f64) = (40.7128, -74.0060);
        let (lat2, lon2): (f64, f64) = (34.0522, -118.2437);
        
        let dist = haversine_dist(
            lat1.to_radians(),
            lon1.to_radians(),
            lat2.to_radians(),
            lon2.to_radians()
        );
        assert!(approx_equal(dist, 3935751.81, EPSILON));
    }

    #[test]
    fn test_poles_distance() {
        let (lat1, lon1): (f64, f64) = (90.0, 0.0);
        let (lat2, lon2): (f64, f64) = (-90.0, 0.0);
        
        let dist = haversine_dist(
            lat1.to_radians(),
            lon1.to_radians(),
            lat2.to_radians(),
            lon2.to_radians()
        );
        assert!(approx_equal(dist, 20015115.07, EPSILON));
    }

    #[test]
    fn test_equator_distance() {
        let (lat1, lon1): (f64, f64) = (0., 0.);
        let (lat2, lon2): (f64, f64) = (0., 90.);

        let dist = haversine_dist(
            lat1.to_radians(),
            lon1.to_radians(),
            lat2.to_radians(),
            lon2.to_radians()
        );
        // Should be quarter of the Earth's circumference
        assert!(approx_equal(dist, 10007557.53, EPSILON));
    }
}
