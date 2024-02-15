use std::sync::{Arc, Weak};

pub struct Config {
    input_layer_size: usize,
    output_layer_size: usize,
    fitness: Box<dyn FnOnce(&Genome) -> f64>,
}
pub struct Population {
    genomes: Vec<Genome>,
    node_count: usize,
    edge_count: usize,
}
impl Population {
    pub fn advance_gen(&mut self, config: &Config) {
        todo!();
    }
}

pub struct NodeId(pub usize);
pub struct EdgeId(pub usize);

pub enum NodeType {
    Input,
    Hidden,
    Output,
}

/* === Genome description === */
pub struct Genome {
    pub edges: Vec<Gene>,
    pub node_count: usize,
}

pub struct Gene {
    pub id: EdgeId,
    pub in_node: NodeId,
    pub out_node: NodeId,
    pub weight: f64,
    pub enabled: bool,
}

/* === Neural Network === */
pub struct Network<'a> {
    // sorted topologically
    nodes: Vec<Node<'a>>,
    // out_nodes: Vec<Arc<Node>>
}

pub struct Node<'a> {
    pub nodeId: NodeId,
    pub activation: f64,
    pub incoming: Vec<&'a Gene>,
    pub outgoing: Vec<&'a Gene>,
}

impl<'a> Network<'a> {
    fn new(genome: &'a Genome, config: &'_ Config) -> Self {
        let mut nodes = Vec::with_capacity(genome.node_count);
        for i in 0..genome.node_count {
            nodes.push(Node {
                nodeId: NodeId(i),
                activation: 0.0,
                incoming: vec![],
                outgoing: vec![],
            });
        }

        for edge in genome.edges.iter() {
            nodes[edge.out_node.0].incoming.push(edge);
            nodes[edge.in_node.0].outgoing.push(edge);
        }

        // TODO: Sort nodes topologically
        Self { nodes }
    }
}

impl Network<'_> {
    fn feed_input(&mut self, x: &[f64]) {
        todo!();
    }

    fn sort_topologically(&mut self) {
        todo!();
    }

    fn propagate(&mut self) {
        todo!();
    }

    fn sort_by_node_id(&mut self) {
        todo!();
    }
}
