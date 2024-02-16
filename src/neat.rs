use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;

use sqlx::Statement;

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

#[derive(Default, PartialEq, Eq)]
pub enum NodeType {
    #[default]
    Input,
    Output,
    Hidden,
}

/* === Genome description === */
pub struct Genome {
    pub edges: Vec<Gene>,
    pub hidden_layer_size: usize,
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
    in_nodes: usize,
    out_nodes: usize,
    edges: Vec<Ref<Edge>>,
    nodes: Vec<Ref<Node>>,
    // ^ a big vector divided into 3 section
    // the first section is the input layer
    // the second section is the output layer
    // the third section is the hidden layers sorted topologically
    // The third section is the only one that is dynamic and needs to be sorted.
}

#[derive(Default)]
pub struct Edge {
    pub weight: f64,
    pub in_node: Ref<Node>,
    pub out_node: Ref<Node>,
    pub visited: bool,
}

#[derive(Default)]
pub struct Node {
    pub activation: f64,
    pub node_type: NodeType,
    pub incoming: Vec<Ref<Edge>>,
    pub outgoing: Vec<Ref<Edge>>,
}

impl Network {
    fn new(genome: &Genome, config: &Config) -> Self {
        let node_count =
            config.input_layer_size + config.output_layer_size + genome.hidden_layer_size;

        let mut nodes = vec![Rc::new(RefCell::new(Node::default())); node_count];

        let mut begin = 0;
        let mut end = config.input_layer_size;

        for i in begin..end {
            RefCell::borrow_mut(&nodes[i]).node_type = NodeType::Input;
        }

        begin = end;
        end += config.output_layer_size;
        for i in begin..end {
            RefCell::borrow_mut(&nodes[i]).node_type = NodeType::Output;
        }

        begin = end;
        end += genome.hidden_layer_size;
        for i in begin..end {
            RefCell::borrow_mut(&nodes[i]).node_type = NodeType::Hidden;
        }

        let mut edges = Vec::with_capacity(genome.edges.len());
        for gene in genome.edges.iter().filter(|edge| edge.enabled) {
            let edge = Rc::new(RefCell::new(Edge {
                weight: gene.weight,
                in_node: nodes[gene.in_node.0].clone(),
                out_node: nodes[gene.out_node.0].clone(),
                visited: false,
            }));
            let mut node = RefCell::borrow_mut(&nodes[gene.out_node.0]);
            node.incoming.push(edge.clone());

            let mut node = RefCell::borrow_mut(&nodes[gene.in_node.0]);
            node.outgoing.push(edge.clone());
        }

        // The hidden layer nodes need to be re-added but in topological order
        nodes.truncate(config.input_layer_size + config.output_layer_size);

        // https://en.wikipedia.org/wiki/Topological_sorting#Kahn's_algorithm
        let mut to_process = nodes[0..config.input_layer_size].to_vec();
        while let Some(node) = to_process.pop() {
            for edge in node.borrow().outgoing.iter() {
                let mut cell = RefCell::borrow_mut(edge);
                cell.visited = true;
                drop(cell);

                let edge = edge.borrow();
                let next_node = edge.out_node.borrow();
                if next_node.node_type == NodeType::Hidden
                    && next_node.incoming.iter().all(|edge| edge.borrow().visited)
                {
                    nodes.push(node.clone());
                    to_process.push(edge.out_node.clone());
                }
            }
        }

        Self {
            nodes,
            edges,
            in_nodes: config.input_layer_size,
            out_nodes: config.output_layer_size,
        }
    }

    fn input(&mut self, x: &[f64]) {
        for (i, x) in x.iter().enumerate() {
            RefCell::borrow_mut(&self.nodes[i]).activation = *x;
        }
    }

    fn propagate(&mut self) {
        for node in self.nodes[self.in_nodes..].iter() {
            let mut value = 0.0;
            for edge in node.borrow().incoming.iter() {
                value += edge.borrow().weight * edge.borrow().in_node.borrow().activation;
            }

            let mut node = RefCell::borrow_mut(&node);
            node.activation = sigmoid(value);
        }
    }

    fn output(&mut self) -> Vec<f64> {
        let begin = self.in_nodes;
        let end = self.in_nodes + self.out_nodes;
        self.nodes[begin..end]
            .iter()
            .map(|node| node.borrow().activation)
            .collect()
    }
}

fn sigmoid(num: f64) -> f64 {
    1.0 / (1.0 + (-num).exp())
}
