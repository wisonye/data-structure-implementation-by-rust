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
pub struct GraphNode<T> {
    // Generic data that binds with the current node/vertex
    pub data: T,

    // All nodes that connects with `edges`
    pub neighbors: Vec<GraphNodeEdge>,
}

///
///
///
pub trait Graph<T> {
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
#[derive(Debug)]
pub struct UndirectedGraph<T> {
    nodes: Vec<GraphNode<T>>,
}

impl<T> Graph<T> for UndirectedGraph<T> {

    ///
    ///
    ///
    fn with_first_node(first_node: GraphNode<T>) -> Self {
        Self {
            nodes: vec!(first_node)
        }
    }

    ///
    ///
    ///
    fn with_all_nodes(nodes: Vec<GraphNode<T>>) -> Self {
        Self {
            nodes
        }
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
