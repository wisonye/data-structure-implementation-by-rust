use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::fmt::Debug;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

///
///
///
#[derive(Debug)]
pub struct GraphNodeEdge {
    // The node index in the graph node array
    pub node_index: usize,

    // The weight of the edge
    pub weight: usize,
}

///
///
///
#[derive(Debug)]
pub struct GraphNode<T: Debug + DeserializeOwned> {
    // Generic data that binds with the current node/vertex
    pub data: Option<T>,

    // All nodes that connects with `edges`
    pub neighbors: Vec<GraphNodeEdge>,
}

///
///
///
pub trait Graph<T: Debug + DeserializeOwned> {
    fn with_first_node(first_node: GraphNode<T>) -> Self;
    fn with_all_nodes(nodes: Vec<GraphNode<T>>) -> Self;
    fn load_from_file(graph_filename: &str) -> Result<Self, String>
    where
        Self: Sized;
    // fn add_node(&mut self, node: GraphNode<T>);
    fn nodes_len(&self) -> usize;
    fn edges_len(&self) -> usize;
    // fn get_neighbors_by_node(&self, node_index: usize) -> [GraphNode<T>];
}

///
///
///
pub struct UndirectedGraph<T: Debug + DeserializeOwned> {
    nodes: Vec<GraphNode<T>>,
}

impl<T: Debug + DeserializeOwned> UndirectedGraph<T> {
    ///
    ///
    ///
    fn get_node_edge_in_ajacency_list_format(&self, node_index: usize) -> String {
        if node_index >= self.nodes.len() {
            return "[  ]".to_string();
        }

        let node = &self.nodes[node_index];
        let mut temp_vec = Vec::<String>::new();
        for neighbor in node.neighbors.iter() {
            temp_vec.push(format!(
                "-> {:?}({})",
                self.nodes[neighbor.node_index].data, neighbor.weight
            ));
            // temp_vec.push(format!("-> {}({})", neighbor.node_index, neighbor.weight));
        }
        temp_vec.join(", ")
    }
}

impl<T: Debug + DeserializeOwned> std::fmt::Debug for UndirectedGraph<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut debug_info = f.debug_struct("[ UndirectedGraph ]");
        debug_info.field("nodes_len", &self.nodes_len());
        debug_info.field("edges_len", &self.edges_len());
        debug_info.field("nodes and edges", &self.edges_len());

        // debug_info.field("{ Adjacency list }", &"");

        //
        // Output like a adjacency list
        //
        for (index, node) in self.nodes.iter().enumerate() {
            let label = if index == 0 {
                format!(
                    "\n>>> Nodes in adjacency list format>>>\n\n[{}] {:?}",
                    index, &node.data
                )
            } else {
                format!("[{}] {:?}", index, &node.data)
            };

            debug_info.field(&label, &self.get_node_edge_in_ajacency_list_format(index));
        }

        debug_info.finish()
    }
}

impl<T: Debug + DeserializeOwned> Graph<T> for UndirectedGraph<T> {
    ///
    ///
    ///
    fn with_first_node(first_node: GraphNode<T>) -> Self {
        Self {
            nodes: vec![first_node],
        }
    }

    ///
    ///
    ///
    fn with_all_nodes(nodes: Vec<GraphNode<T>>) -> Self {
        Self { nodes }
    }

    ///
    ///
    ///
    fn load_from_file(graph_filename: &str) -> Result<Self, String>
    where
        Self: Sized,
    {
        // The output is wrapped in a Result to allow matching on errors
        // Returns an Iterator to the Reader of the lines of the file.
        fn read_lines<P>(filename: P) -> Result<io::Lines<io::BufReader<File>>, std::io::Error>
        where
            P: AsRef<Path>,
        {
            let file = File::open(filename)?;
            Ok(io::BufReader::new(file).lines())
        }

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct Planet {
            name: String,
            draw_color: String,
            draw_sprite: String,
            is_reachable: bool,
        }

        let lines = read_lines(graph_filename);
        if lines.is_err() {
            return Err(lines.unwrap_err().to_string());
        }

        let mut loaded_graph = Self { nodes: vec![] };

        for line in lines.unwrap() {
            if let Ok(node_str) = line {
                // println!("lines: {node_str}");

                let mut graph_node = GraphNode::<T> {
                    data: None,
                    neighbors: vec![],
                };

                //
                // Each line has the following format:
                //
                // `{...JSON data here} | edges_separated_by_comma`
                //
                // Each edge has the following format:
                //
                // `-> Connected_node_index(edge_weight)`
                //
                // Here is the example:
                //
                // {"name": "Alien Home"} |  -> 1(5),
                // {"name": "Earth"} | -> 0(5), -> 2(8),
                // {"name": "Mars"} | -> 1(8),
                //
                for (index, temp_str) in node_str.split("|").into_iter().enumerate() {
                    // GraphNode data json
                    if index == 0 {
                        graph_node.data = Some(from_str::<T>(temp_str).unwrap());
                    }
                    // GraphNodeEdge
                    else if index == 1 {
                        let edges = temp_str.split(",");
                        for edge_str in edges.into_iter() {
                            if edge_str.trim() == "" {
                                continue;
                            }
                            // println!("edge_str: {edge_str}");

                            let invalid_edge_format = format!("Edge string '{edge_str}' should the following format: '-> Connected_node_index(edge_weight)'");
                            let after_arrow_pos = edge_str.find("-> ");
                            let left_parentheses_pos = edge_str.find('(');
                            let right_parentheses_pos = edge_str.find(')');

                            if after_arrow_pos.is_none()
                                || left_parentheses_pos.is_none()
                                || right_parentheses_pos.is_none()
                            {
                                return Err(invalid_edge_format);
                            }

                            let connected_node_index = &edge_str
                                [after_arrow_pos.unwrap() + 3..left_parentheses_pos.unwrap()]
                                .parse::<usize>();
                            // println!("connected_node_index {connected_node_index:?}");

                            if connected_node_index.is_err() {
                                return Err(invalid_edge_format);
                            }

                            let edge_weight = &edge_str
                                [left_parentheses_pos.unwrap() + 1..right_parentheses_pos.unwrap()]
                                .parse::<usize>();
                            // println!("edge_weight: {edge_weight:?}");

                            if edge_weight.is_err() {
                                return Err(invalid_edge_format);
                            }

                            graph_node.neighbors.push(GraphNodeEdge {
                                node_index: *connected_node_index.as_ref().unwrap(),
                                weight: *edge_weight.as_ref().unwrap(),
                            });
                        }
                    }
                }

                loaded_graph.nodes.push(graph_node);
            }
        }

        Ok(loaded_graph)
    }

    // ///
    // ///
    // fn add_node(&mut self, node: GraphNode<T>) {
    //    self. nodes.push(node);
    // }

    ///
    ///
    ///
    fn nodes_len(&self) -> usize {
        self.nodes.len()
    }

    ///
    ///
    ///
    fn edges_len(&self) -> usize {
        self.nodes
            .iter()
            .fold(0, |acc, ele| acc + ele.neighbors.len())
    }

    // ///
    // ///
    // ///
    // fn get_neighbors_by_node(&self, node_index: usize) -> [GraphNode<T>] {
    //     let edges = self.nodes[node_index];
    // }
}
