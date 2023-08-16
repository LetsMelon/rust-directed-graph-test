use std::error::Error;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

use petgraph::dot::{Config, Dot};
use petgraph::stable_graph::NodeIndex;
use petgraph::Graph;

use crate::table::Table;
use crate::table_holder::TableHolder;

#[derive(Debug)]
pub struct TableGraph {
    inner: Graph<Table, usize>,
}

impl TableGraph {
    pub fn new() -> TableGraph {
        TableGraph {
            inner: Graph::new(),
        }
    }

    pub fn add_table(&mut self, table: TableHolder) -> NodeIndex {
        table.add_to_graph(self.inner_mut())
    }

    pub fn inner(&self) -> &Graph<Table, usize> {
        &self.inner
    }

    pub fn inner_mut(&mut self) -> &mut Graph<Table, usize> {
        &mut self.inner
    }

    pub fn toposort(&self) -> Vec<Table> {
        // TODO don't ignore if a cycle is detected
        petgraph::algo::toposort(self.inner(), None)
            .unwrap_or_default()
            .iter()
            .filter_map(|node_index| self.inner().node_weight(*node_index))
            .cloned()
            .collect::<Vec<_>>()
    }

    pub fn save_graph<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn Error>> {
        let mut dot_child = Command::new("dot")
            .arg("-Tpng")
            .args(["-o", path.as_ref().to_str().unwrap()])
            .stdin(Stdio::piped())
            .spawn()?;

        let child_stdin = dot_child.stdin.as_mut().unwrap();
        write!(
            child_stdin,
            "{:?}",
            Dot::with_config(&self.inner(), &[Config::EdgeNoLabel])
        )?;
        let _ = child_stdin;

        let output = dot_child.wait_with_output()?;

        if !output.status.success() {
            Err(format!(
                "Exited binary `dot` with an error.\nerr: {:?}",
                output.stderr
            )
            .into())
        } else {
            Ok(())
        }
    }
}
