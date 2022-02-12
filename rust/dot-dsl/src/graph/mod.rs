pub mod graph_items;

use graph_items::edge::Edge;
use graph_items::node::Node;
use std::collections::HashMap;

pub struct Graph {
    pub nodes: Vec<Node>,
    pub attrs: HashMap<String, String>,
    pub edges: Vec<Edge>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: vec![],
            attrs: HashMap::new(),
            edges: vec![],
        }
    }

    pub fn with_nodes(mut self, nodes: &Vec<Node>) -> Self {
        self.nodes = nodes.to_vec();
        self
    }

    pub fn with_edges(mut self, edges: &[Edge]) -> Self {
        self.edges = edges.to_vec();
        self
    }

    pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
        self.attrs = attrs
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        self
    }

    pub fn get_node(&self, name: &str) -> Option<&Node> {
        self.nodes.iter().find(|n| n.name == name)
    }
}
