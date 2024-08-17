use std::fmt;

/// OSMNode contains all information that we might care about in an edge as stored in
/// the petgraph. Currently, it contains the two nodes it is connected to (`[u64; 2]` where u64 is
/// the node ID as defined by OSM, and the first element is the first node, the second element is
/// the second), the distance between them, and the type of edge (highway, street, sidewalk, etc.)
#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct OSMEdge {

    //Node IDs
    nodes: [u64; 2],

    //Distance between the two nodes
    dist: f64,

    //Highway type as defined by OSM
    highway_type: String
}

impl fmt::Display for OSMEdge {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "OSMEdge(n1: {}, n2: {}, dist: {}, road type: {})",
            self.nodes[0],
            self.nodes[1],
            self.dist,
            self.highway_type
        )
    }
}

impl OSMEdge {

    /// Create a new `OSMEdge` from fields.
    pub fn new(nodes: [u64; 2], dist: f64, highway_type: String) -> Self {
        OSMEdge {
            nodes,
            dist,
            highway_type
        }
    }

    /// Get the nodes (their IDs).
    pub fn nodes(&self) -> [u64; 2] {
        self.nodes
    }
    /// Get the length of the `OSMEdge` in meters.
    pub fn dist(&self) -> f64 {
        self.dist
    }
    /// Get the type of the `OSMEdge`.
    pub fn highway_type(&self) -> &str {
        &self.highway_type
    }
}
