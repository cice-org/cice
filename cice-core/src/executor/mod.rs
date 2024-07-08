use core::borrow::Borrow;

use alloc::vec::Vec;
use state::State;
use task::Task;

use crate::graph::ConstGraph;

pub mod state;
pub mod task;

pub struct Excecutor {
    graph: ConstGraph<Task, State>,
    current_state: State,
}

impl Excecutor {
    pub fn new(tasks: Vec<Task>, states: Vec<State>, first_state: State) -> Self {
        let mut graph = ConstGraph::new();
        let mut push_state = |state: State| {
            let r = state.inner.borrow_mut().id.replace(graph.next_node_index());
            debug_assert!(r.is_none());
            graph.push_node(state);
        };
        push_state(first_state.clone());
        for state in states {
            push_state(state)
        }
        for task in tasks {
            let from = task.from.inner.as_ref().borrow().id.unwrap();
            let to = task.to.inner.as_ref().borrow().id.unwrap();
            graph.insert_edge(from, to, task)
        }
        Self {
            graph,
            current_state: first_state,
        }
    }

    pub fn run(&self) {
        loop {
            let task_index = self
                .graph
                .get_out_edges(self.current_state.inner.as_ref().borrow().id.unwrap());
            for index in task_index {
                let task = &self.graph.get_edge(index).data;
            }
        }
    }
}
