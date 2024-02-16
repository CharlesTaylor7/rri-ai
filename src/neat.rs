use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;

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
type Ref<T> = Rc<RefCell<T>>;
pub struct Network {
    edges: Vec<Ref<Edge>>,
    /// sorted topologically
    nodes: Vec<Ref<Node>>,
    in_nodes: Vec<Ref<Node>>,
    out_nodes: Vec<Ref<Node>>,
}

#[derive(Default, Clone)]
pub struct Edge {
    pub weight: f64,
    pub in_node: Ref<Node>,
    pub visited: bool,
}

#[derive(Default, Clone)]
pub struct Node {
    pub activation: f64,
    pub incoming: Vec<Ref<Edge>>,
    pub outgoing: Vec<Ref<Edge>>,
}

impl Network {
    fn new(genome: &Genome, config: &Config) -> Self {
        let mut nodes_to_sort = Vec::with_capacity(genome.node_count);
        for i in 0..config.input_layer_size {
            nodes_to_sort.push(Node {
                activation: 0.0,
                incoming: vec![],
                outgoing: vec![],
            });
        }

        let mut nodes = vec![Rc::new(RefCell::new(Node::default())); genome.node_count];

        let mut edges = Vec::with_capacity(genome.edges.len());
        for gene in genome.edges.iter() {
            let edge = Rc::new(RefCell::new(Edge {
                weight: gene.weight,
                in_node: nodes[gene.in_node.0].clone(),
                visited: false,
            }));
            let mut node = RefCell::borrow_mut(&nodes[gene.out_node.0]);
            node.incoming.push(edge.clone());

            let mut node = RefCell::borrow_mut(&nodes[gene.in_node.0]);
            node.incoming.push(edge.clone());
        }

        while let Some(node) = nodes_to_sort.pop() {
            todo!();
        }

        // TODO: Sort nodes topologically
        Self {
            nodes,
            edges,
            in_nodes: vec![],
            out_nodes: vec![],
        }
    }

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
