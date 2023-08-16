use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

use petgraph::stable_graph::NodeIndex;

use crate::table_graph::InnerGraph;

#[derive(Clone)]
pub struct Table {
    name: String,
    id: Option<NodeIndex>,

    dependent: Vec<Rc<RefCell<Table>>>,
}

impl Table {
    pub fn new(name: impl Into<String>) -> Table {
        Table {
            name: name.into(),
            id: None,

            dependent: Vec::new(),
        }
    }

    pub fn add_to_graph(&mut self, graph: &mut InnerGraph) -> NodeIndex {
        const DEFAULT_WEIGHT: usize = 1;

        if self.id.is_none() {
            let id = graph.add_node(self.clone());
            self.id = Some(id);

            for dependency in &mut self.dependent {
                let mut other_id = dependency.borrow().id;

                if other_id.is_none() {
                    other_id = Some(dependency.borrow_mut().add_to_graph(graph));
                }
                assert!(other_id.is_some());

                graph.add_edge(other_id.unwrap(), id, DEFAULT_WEIGHT);
            }
        }

        self.id.unwrap()
    }

    pub fn add_dependency(&mut self, other: Rc<RefCell<Table>>) {
        self.dependent.push(other.clone())
    }
}

impl Debug for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Table")
            .field("name", &self.name)
            .field(
                "dependent",
                &self
                    .dependent
                    .iter()
                    .map(|item| item.borrow().name.clone())
                    .collect::<Vec<_>>(),
            )
            .finish()
    }
}
