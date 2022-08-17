use data_structure_implementation_by_rust::graph::undirected_graph::*;


#[derive(Debug)]
struct Place {
    label: String,
    relative_position: (isize, isize),
}

#[test]
fn create_undirected_graph_should_work() {
    let temp_graph = UndirectedGraph::new(GraphNode {
        data: Place {
            label: "Home".to_string(),
            relative_position: (10, 20),
        },
        neighbors: vec!()
    });

    println!("temp_graph: {temp_graph:#?}");

    assert_eq!(temp_graph.nodes_len(), 1);
    assert_eq!(temp_graph.edges_len(), 0);
}
