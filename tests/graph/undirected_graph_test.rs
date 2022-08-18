use data_structure_implementation_by_rust::graph::undirected_graph::*;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Planet {
    label: Option<String>,
    draw_color: Option<String>,
    draw_sprite: Option<String>,
    is_reachable: Option<bool>,
    relative_position: Option<(isize, isize)>,
}

impl std::fmt::Debug for Planet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let label_str = if self.label.is_some() {
            self.label.as_ref().unwrap()
        } else {
            "No lablel"
        };
        write!(f, "{}", label_str)
    }
}

fn create_graph() -> UndirectedGraph<Planet> {
    let home_earth_edge = GraphNodeEdge {
        node_index: 1, // Earth node index in graph nodes
        weight: 5,
    };

    let home_node = GraphNode {
        data: Some(Planet {
            label: Some("Alien Home".to_string()),
            draw_color: None,
            draw_sprite: None,
            relative_position: Some((10, 20)),
            is_reachable: Some(true),
        }),
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
        data: Some(Planet {
            label: Some("Earth".to_string()),
            draw_color: None,
            draw_sprite: None,
            relative_position: Some((20, 30)),
            is_reachable: Some(true),
        }),
        neighbors: vec![earth_home_edge, earth_mars_edge],
    };

    let mars_earth_edge = GraphNodeEdge {
        node_index: 1, // Earth node index in graph nodes
        weight: 8,
    };

    let mars_node = GraphNode {
        data: Some(Planet {
            label: Some("Mars".to_string()),
            draw_color: None,
            draw_sprite: None,
            relative_position: Some((40, 50)),
            is_reachable: Some(true),
        }),
        neighbors: vec![mars_earth_edge],
    };

    UndirectedGraph::with_all_nodes(vec![home_node, earth_node, mars_node])
}

#[test]
fn create_undirected_graph_should_work() {
    let temp_graph = create_graph();
    println!("temp_graph: {temp_graph:#?}");

    assert_eq!(temp_graph.nodes_len(), 3);
    assert_eq!(temp_graph.edges_len(), 4);
}

#[test]
fn load_graph_from_file_should_work() {
    // if let Ok(graph) = UndirectedGraph::<Planet>::load_from_file("alien_map_1.txt") {

    // }

    match UndirectedGraph::<Planet>::load_from_file("alien_map_1.txt") {
        Ok(graph) => {
            println!("loaded graph: {graph:#?}");

            // assert_eq!(graph.nodes_len(), 3);
            // assert_eq!(graph.edges_len(), 4);
        }
        Err(error) => {
            println!(">>> 'load_graph_from_file_should_work' failed by: {error:#?}");
        }
    }
}
