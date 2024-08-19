use std::fmt;
use std::error::Error;

use serde_json::Value;

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
pub fn get_osm_ways(elements: &Vec<Value>) -> Result<Vec<OSMWay>, Box<dyn Error>> {

    //Only get OSM elements that are nodes
    let way_elements: Vec<Value> = elements.clone().into_iter()
        .filter(|e| {
            match e.get("type") {
                Some(t) => t == "way",
                None => false
            }
        })
        .collect();

    let mut result: Vec<OSMWay> = Vec::with_capacity(way_elements.len());

    for e in way_elements {

        //If not tags, skip
        let tags: &Value = if let Some(tags) = e.get("tags") {
            tags
        } else {
            continue;
        };

        //If not a highway, then skip
        let highway_type: String = if let Some(highway_type) = tags.get("highway") {
            highway_type.to_string()
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
            .expect("Could not parse nodes into Vec!")
            .iter()
            .map(|x| 
                x.as_u64().unwrap_or_else(||
                    panic!("Could not parse id {} into u64!", x)
                )
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
