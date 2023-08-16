use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

use petgraph::stable_graph::NodeIndex;

use crate::table::Table;
use crate::table_graph::InnerGraph;

pub struct TableHolder {
    inner: Rc<RefCell<Table>>,
}

impl TableHolder {
    pub fn new(name: impl Into<String>) -> TableHolder {
        Self::new_from_table(Table::new(name))
    }

    pub fn new_from_table(table: Table) -> TableHolder {
        TableHolder {
            inner: Rc::new(RefCell::new(table)),
        }
    }

    pub fn add_to_graph(self, graph: &mut InnerGraph) -> NodeIndex {
        self.inner.borrow_mut().add_to_graph(graph)
    }

    pub fn add_dependency(&mut self, other: &TableHolder) {
        self.inner.borrow_mut().add_dependency(other.inner.clone())
    }
}

impl Debug for TableHolder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.borrow().fmt(f)
    }
}
