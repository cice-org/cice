use alloc::vec;
use alloc::vec::Vec;

use node::{Edge, Node};

pub mod node;

pub(crate) type EdgeIndex = usize;
pub(crate) type NodeIndex = usize;

/// This graph data struture is designed to represent a const-graph,
/// which means it is not allowed to add or remove nodes or edges once it was initialized.
pub(crate) struct ConstGraph<E, T> {
    graph: Vec<Vec<Option<EdgeIndex>>>,
    nodes: Vec<Node<T>>, //The Nodes indexed by NodeIndex
    edges: Vec<Edge<E>>, //The Edges indexed by EdgeIndex
}

impl<E, T> ConstGraph<E, T> {
    pub(crate) fn new() -> Self {
        Self {
            graph: vec![],
            nodes: vec![],
            edges: vec![],
        }
    }
    pub(crate) fn push_node(&mut self, data: T) -> NodeIndex {
        let next_index = self.graph.len();
        self.nodes.push(Node {
            id: next_index,
            data,
        });
        for item in &mut self.graph {
            item.resize_with(self.nodes.len(), || None);
        }
        self.graph.push(vec![None; self.nodes.len()]);
        return next_index;
    }

    /// ## Safety
    ///
    /// - Make sure `source` and `target` is a valid index manually
    /// - Insert an edge repeately is an undefined behavior, which should be avoided
    pub(crate) fn insert_edge(&mut self, source: NodeIndex, target: NodeIndex, data: E) {
        debug_assert!(source < self.nodes.len());
        debug_assert!(target < self.nodes.len());
        debug_assert!(self.graph[source][target].is_none());
        let next_index = self.edges.len();
        let edge = Edge {
            id: next_index,
            source,
            target,
            data,
        };
        self.graph[source][target] = Some(next_index);
        self.edges.push(edge);
    }

    /// Make sure `index` is valid manually
    pub(crate) fn get_node(&self, index: NodeIndex) -> &Node<T> {
        self.nodes.get(index).unwrap()
    }

    /// Make sure `index` is valid manually
    pub(crate) fn get_edge(&self, index: EdgeIndex) -> &Edge<E> {
        self.edges.get(index).unwrap()
    }

    /// Make sure `index` is valid manually
    pub(crate) fn get_node_mut(&mut self, index: NodeIndex) -> &Node<T> {
        self.nodes.get_mut(index).unwrap()
    }

    /// Make sure `index` is valid manually
    pub(crate) fn get_edge_mut(&mut self, index: EdgeIndex) -> &Edge<E> {
        self.edges.get_mut(index).unwrap()
    }

    pub(crate) fn get_out_edges(&self, index: NodeIndex) -> Vec<EdgeIndex> {
        let mut ret = vec![];
        for item in self.graph.get(index).unwrap() {
            if let Some(edge) = item {
                ret.push(*edge);
            }
        }
        ret
    }

    pub(crate) fn get_subnodes(&self, index: NodeIndex) -> Vec<NodeIndex> {
        let mut ret = vec![];
        for (index, item) in self.graph.get(index).unwrap().iter().enumerate() {
            if let Some(_) = item {
                ret.push(index);
            }
        }
        ret
    }
}

#[cfg(test)]
mod test {
    use super::ConstGraph;

    #[test]
    fn test_graph() {
        let mut graph = ConstGraph::new();
        let begin = graph.push_node(1);
        let end = graph.push_node(3);
        graph.insert_edge(begin, end, 2);
        assert_eq!(*graph.get_edge(graph.get_out_edges(begin)[0]).data(), 2)
    }
}
