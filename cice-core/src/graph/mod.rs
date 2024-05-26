use alloc::vec;
use alloc::vec::Vec;

use node::{Edge, Node};

pub mod node;

pub(crate) type EdgeIndex = usize;
pub(crate) type NodeIndex = usize;

pub(crate) struct Graph<E, T> {
    graph: Vec<Vec<Option<EdgeIndex>>>,
    nodes: Vec<Node<T>>, //The Nodes indexed by NodeIndex
    edges: Vec<Edge<E>>, //The Edges indexed by EdgeIndex
}

impl<E, T> Graph<E, T> {
    pub(crate) fn push_node(&mut self, data: T) -> NodeIndex {
        let cur_index = self.graph.len();
        self.nodes.push(Node {
            id: cur_index + 1,
            data,
        });
        for item in &mut self.graph {
            item.resize_with(self.nodes.len(), || None);
        }
        self.graph.push(vec![None; self.nodes.len()]);
        return cur_index + 1;
    }

    pub(crate) fn insert_edge(source: NodeIndex, target: NodeIndex, data: E) {}
}

#[cfg(test)]
mod test {
    #[test]
    fn test_graph() {}
}
