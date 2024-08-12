pub struct OSMEdge {

    //Node IDs
    pub nodes: [u64; 2],

    //Distance between the two nodes
    pub dist: f64,

    //Highway type as defined by OSM
    pub highway_type: String
}
