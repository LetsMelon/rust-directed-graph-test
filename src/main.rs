mod table;
mod table_graph;
mod table_holder;

use crate::table_graph::TableGraph;
use crate::table_holder::TableHolder;

fn main() {
    let table_d = TableHolder::new("d");

    let mut table_a = TableHolder::new("a");
    table_a.add_dependency(&table_d);

    let mut table_b = TableHolder::new("b");
    table_b.add_dependency(&table_a);
    table_b.add_dependency(&table_d);

    let mut table_c = TableHolder::new("c");
    table_c.add_dependency(&table_a);
    table_c.add_dependency(&table_b);

    let mut deps = TableGraph::new();

    deps.add_table(table_d);
    deps.add_table(table_a);
    deps.add_table(table_b);
    deps.add_table(table_c);

    let toposort = deps.toposort();
    for (i, node) in toposort.iter().enumerate() {
        println!("{}: {:?}", i, node);
    }

    deps.inner_mut().reverse();
    deps.save_graph("output.png").unwrap();
}
