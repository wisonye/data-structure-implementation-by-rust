use data_structure_implementation_by_rust::graph::undirected_graph::*;

#[derive(Debug)]
struct Place {
    label: String,
    relative_position: (isize, isize),
}

fn create_graph() -> UndirectedGraph<Place> {
    let home_earth_edge = GraphNodeEdge {
        node_index: 1, // Earth node index in graph nodes
        weight: 5,
    };

    let home_node = GraphNode {
        data: Place {
            label: "Home".to_string(),
            relative_position: (10, 20),
        },
        neighbors: vec![home_earth_edge],
    };

    let earth_home_edge = GraphNodeEdge {
        node_index: 0, // Home node index in graph nodes
        weight: 5,
    };

    let earth_mars_edge = GraphNodeEdge {
        node_index: 2, // Mars node index in graph nodes
        weight: 8,
    };

    let earth_node = GraphNode {
        data: Place {
            label: "Earth".to_string(),
            relative_position: (20, 30),
        },
        neighbors: vec![earth_home_edge, earth_mars_edge],
    };

    UndirectedGraph::with_all_nodes(vec![home_node, earth_node])
}

#[test]
fn create_undirected_graph_should_work() {
    let temp_graph = create_graph();
    println!("temp_graph: {temp_graph:#?}");

    assert_eq!(temp_graph.nodes_len(), 2);
    assert_eq!(temp_graph.edges_len(), 3);
}

