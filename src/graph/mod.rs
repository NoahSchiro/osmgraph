//! This module gives us all of the tools needed for dealing with graphs.
//! The crate [petgraph](https://docs.rs/petgraph/latest/petgraph/index.html) is used extensively
//! for dealing with graphs but we still need a few items here to make things mesh well.
//!
//! The first component of a graph is [`crate::graph::OSMNode`]. Petgraph stores nodes simply
//! as just one index within a graph, but there is a host of other bits of information we might
//! care about (latitude and longitude for instance). OSMNode will carry this infomation for us.
//!
//! Another component is [`crate::graph::OSMEdge`]. For edges, petgraph only cares about an
//! edge having an index and references to the nodes its connected to. In the mapping context,
//! we care about things like distance between the nodes (the length of an edge) and what type
//! it is (highway, sidewalk, railroad, etc).
//!
//! With both the `OSMNode` and `OSMEdge` we can create a [`crate::graph::OSMGraph`] which is just
//! a retyping of a petgraph type (`UnGraph<OSMNode, OSMEdge>`).
//!
//! Distinct from the `OSMEdge` is the [`crate::graph::way::OSMWay`]. There is a distinction here
//! because OSM stores a way as a *polylines* of Nodes, but petgraph stores edges just as a *pair* of
//! nodes. Storing a (long) list of nodes for each edge is also inefficient. But we still have to
//! deal with the way that OSM stores this information, therefore we have a distinction.

pub mod edge;
pub use edge::*;

pub mod node;
pub use node::*;

pub mod way;

pub mod graph;
pub use graph::*;
