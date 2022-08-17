use std::fmt::Debug;

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
pub struct GraphNode<T: Debug> {
    // Generic data that binds with the current node/vertex
    pub data: T,

    // All nodes that connects with `edges`
    pub neighbors: Vec<GraphNodeEdge>,
}

///
///
///
pub trait Graph<T: Debug> {
    fn with_first_node(first_node: GraphNode<T>) -> Self;
    fn with_all_nodes(nodes: Vec<GraphNode<T>>) -> Self;
    // fn add_node(&mut self, node: GraphNode<T>);
    fn nodes_len(&self) -> usize;
    fn edges_len(&self) -> usize;
    // fn get_neighbors_by_node(&self, node_index: usize) -> [GraphNode<T>];
}

///
///
///
pub struct UndirectedGraph<T: Debug> {
    nodes: Vec<GraphNode<T>>,
}

impl<T: Debug> UndirectedGraph<T> {
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

impl<T: Debug> std::fmt::Debug for UndirectedGraph<T> {
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

impl<T: Debug> Graph<T> for UndirectedGraph<T> {
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
