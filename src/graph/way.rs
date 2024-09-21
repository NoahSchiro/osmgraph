use std::fmt;
use std::error::Error;

use crate::api::Element;

/// OSMWay contains all information that we might care about in a way. Currently, it contains a
/// way ID (as defined in Overpass API) the nodes indicies on the path, the distances between them,
/// and the type of way (highway, street, sidewalk, etc).
#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct OSMWay {
    id: u64,
    nodes: Vec<u64>,
    dists: Vec<f64>,
    highway_type: String
}

impl fmt::Display for OSMWay {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "OSMWay(\n")?;
        write!(f, "  id: {}\n", self.id)?;
        write!(f, "  type: {}\n", self.highway_type)?;

        write!(f, "  nodes: [\n")?;
        for node in &self.nodes {
            write!(f, "    {}\n", node)?;
        }
        write!(f, "  ]\n")?;
        
        write!(f, "  dists: [\n")?;
        for dist in &self.dists {
            write!(f, "    {}\n", dist)?;
        }
        write!(f, "  ]\n")?;

        write!(f, ")")?;

        Ok(())
    }
}

//Getters and setters
impl OSMWay {

    /// Create a new OSMWay from fields.
    pub fn new(id: u64, nodes: Vec<u64>, dists: Vec<f64>, highway_type: String) -> Self {
        OSMWay { id, nodes, dists, highway_type }
    }

    /// Get the way ID.
    pub fn id(&self) -> u64 {
        self.id
    }
    /// Get the nodes on this way.
    pub fn nodes(&self) -> &Vec<u64> {
        &self.nodes
    }
    /// Get the distances between nodes.
    pub fn dists(&self) -> &Vec<f64> {
        &self.dists
    }
    /// Get the type of way.
    pub fn highway_type(&self) -> &str {
        &self.highway_type
    }
}

/// Given a json type structure, this function tries to parse all `OSMWay` out of that json.
pub fn get_osm_ways(elements: &Vec<Element>) -> Result<Vec<OSMWay>, Box<dyn Error>> {

    //Only get OSM elements that are ways and the ways must have tags
    let way_elements: Vec<OSMWay> = elements.into_iter()
        .filter_map(|elem| {
            if let Element::Way { id, nodes, tags } = elem {

                if *tags == None {
                    return None
                }
                if let Some(highway_type) = tags.clone()?.get("highway") {
                    Some(OSMWay {
                        id: *id,
                        nodes: nodes.to_vec(),
                        // We can only compute distance if we have access to the nodes as well
                        // Leave this blank at the moment
                        dists: vec![], 
                        highway_type: highway_type.to_string()
                    })
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    Ok(way_elements)
}
