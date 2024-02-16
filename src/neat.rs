#![allow(unused_imports)]
#![allow(unused_braces)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unreachable_code)]

use anyhow::{bail, Result};
use std::cell::RefCell;
use std::ops::Range;
use std::rc::Rc;

pub struct Config {
    input_layer_size: usize,
    output_layer_size: usize,
    fitness: Box<dyn FnOnce(&Genome) -> f64>,
    speciation: Speciation,
    new_node_chance: f64,
    new_link_chance: f64,
    population: usize,
    survivor_rate: f64,
}

pub struct Speciation {
    c1: f64, // disjoint
    c2: f64, // excess
    c3: f64, // weight
    // compatibility threshold
    ct: f64,
}

impl Speciation {
    fn compatible(&self, a: &Genome, b: &Genome) -> bool {
        // calculate speciation distance
        // counts
        let mut disjoint = 0;
        let mut excess = 0;
        let mut matching = 0;

        let mut weight_diff = 0.0;

        let N = std::cmp::max(a.genes.len(), b.genes.len());

        // genome indices
        let mut i = 0;
        let mut j = 0;
        loop {
            let gene_a = a.genes.get(i);
            let gene_b = b.genes.get(j);
            match (a.genes.get(i), b.genes.get(j)) {
                (Some(gene_a), Some(gene_b)) => {
                    if gene_a.id == gene_b.id {
                        weight_diff += (a.genes[i].weight - b.genes[j].weight).abs();
                        matching += 1;
                        i += 1;
                        j += 1;
                    } else {
                        disjoint += 1;
                        if gene_a.id < gene_b.id {
                            i += 1;
                        } else {
                            j += 1;
                        }
                    }
                }

                (Some(gene_a), None) => {
                    excess += 1;
                    i += 1;
                }
                (None, Some(gene_b)) => {
                    excess += 1;
                    j += 1;
                }

                (None, None) => {
                    break;
                }
            }
        }

        #[rustfmt::skip]
        let speciation_distance = 
            (self.c1 * (excess as f64) + self.c2 * (disjoint as f64)) / (N as f64)
            + (self.c3 * weight_diff / (matching as f64));

        speciation_distance < self.ct
    }
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

#[derive(Default, Debug)]
pub struct NodeId(pub usize);

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
pub struct GeneId(pub usize);

#[derive(Default, PartialEq, Eq)]
pub enum NodeType {
    #[default]
    Input,
    Output,
    Hidden,
}

/* === Genome description === */
pub struct Genome {
    pub genes: Vec<Gene>,
    pub hidden_layer_size: usize,
}

pub struct Gene {
    pub id: GeneId,
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

pub struct Edge {
    pub id: GeneId,
    pub weight: f64,
    pub in_node: Ref<Node>,
    pub out_node: Ref<Node>,
    pub visited: bool,
}

#[derive(Default)]
pub struct Node {
    pub id: NodeId,
    pub weight: f64,
    pub activation: f64,
    pub node_type: NodeType,
    pub incoming: Vec<Ref<Edge>>,
    pub outgoing: Vec<Ref<Edge>>,
}

impl Network {
    fn new(genome: &Genome, config: &Config) -> Result<Self> {
        let node_count =
            config.input_layer_size + config.output_layer_size + genome.hidden_layer_size;

        let mut nodes = vec![Rc::new(RefCell::new(Node::default())); node_count];

        let mut begin = 0;
        let mut end = config.input_layer_size;

        for i in begin..end {
            let mut node = RefCell::borrow_mut(&nodes[i]);
            node.id = NodeId(i);
            node.node_type = NodeType::Input;
        }

        begin = end;
        end += config.output_layer_size;
        for i in begin..end {
            let mut node = RefCell::borrow_mut(&nodes[i]);
            node.id = NodeId(i);
            node.node_type = NodeType::Output;
        }

        begin = end;
        end += genome.hidden_layer_size;
        for i in begin..end {
            let mut node = RefCell::borrow_mut(&nodes[i]);
            node.id = NodeId(i);
            node.node_type = NodeType::Hidden;
        }

        let mut edges = Vec::with_capacity(genome.genes.len());
        for gene in genome.genes.iter().filter(|edge| edge.enabled) {
            let edge = Rc::new(RefCell::new(Edge {
                id: gene.id,
                weight: gene.weight,
                in_node: nodes[gene.in_node.0].clone(),
                out_node: nodes[gene.out_node.0].clone(),
                visited: false,
            }));
            let mut node = RefCell::borrow_mut(&nodes[gene.out_node.0]);
            node.incoming.push(edge.clone());

            let mut node = RefCell::borrow_mut(&nodes[gene.in_node.0]);
            node.outgoing.push(edge.clone());

            edges.push(edge);
        }

        // The hidden layer nodes need to be re-added but in topological order
        nodes.truncate(config.input_layer_size + config.output_layer_size);

        // https://en.wikipedia.org/wiki/Topological_sorting#Kahn's_algorithm
        let mut to_process = nodes[0..config.input_layer_size].to_vec();
        while let Some(node) = to_process.pop() {
            for edge in node.borrow().outgoing.iter() {
                if edge.borrow().visited {
                    let edge = edge.borrow();
                    bail!(
                        "Neural net contains a cycle. Gene: {:?} between nodes: {:?}, {:?}",
                        edge.id,
                        edge.in_node.borrow().id,
                        edge.out_node.borrow().id,
                    );
                }
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

        Ok(Self {
            nodes,
            edges,
            in_nodes: config.input_layer_size,
            out_nodes: config.output_layer_size,
        })
    }

    fn input(&self, x: &[f64]) {
        for (i, x) in x.iter().enumerate() {
            RefCell::borrow_mut(&self.nodes[i]).activation = *x;
        }
    }

    fn propagate(&self) {
        let fixed_end = self.in_nodes + self.out_nodes;
        let hidden_range = (fixed_end)..;
        let output_range = self.in_nodes..fixed_end;
        Self::propagate_nodes(self.nodes[fixed_end..].iter());
        Self::propagate_nodes(self.nodes[self.in_nodes..fixed_end].iter());
    }

    fn propagate_nodes<'a>(iterator: impl Iterator<Item = &'a Ref<Node>>) {
        for node in iterator {
            let mut value = 0.0;
            for edge in node.borrow().incoming.iter() {
                value += edge.borrow().weight * edge.borrow().in_node.borrow().activation;
            }

            let mut node = RefCell::borrow_mut(&node);
            node.activation = sigmoid(value);
        }
    }

    fn output(&self) -> Vec<f64> {
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