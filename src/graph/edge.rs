pub struct OSMEdge {

    //Node IDs
    nodes: [u64; 2],

    //Distance between the two nodes
    dist: f64,

    //Highway type as defined by OSM
    highway_type: String
}

impl OSMEdge {

    pub fn new(nodes: [u64; 2], dist: f64, highway_type: String) -> Self {
        OSMEdge {
            nodes,
            dist,
            highway_type
        }
    }

    pub fn nodes(&self) -> [u64; 2] {
        self.nodes
    }
    pub fn dist(&self) -> f64 {
        self.dist
    }
    pub fn highway_type(&self) -> &str {
        &self.highway_type
    }
}
