use super::genome::*;
use anyhow::{bail, Result};
use core::num;
use decorum::R64;
use num_traits::real::Real;
use num_traits::sign::Signed;
use rand::seq::SliceRandom;
use rand::Rng;
use rand_distr::StandardNormal;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Write;
use std::ops::{Add, Range};
use std::path::Path;
use std::process::Command;
use std::rc::Rc;
use std::{default, usize};

type Ref<T> = Rc<RefCell<T>>;

pub struct Network {
    in_nodes: usize,
    out_nodes: usize,
    edges: Vec<Ref<Edge>>,
    nodes: Vec<Ref<Node>>,
}
impl Network {
    /// https://graphviz.org/doc/info/lang.html
    pub fn dump_graphviz(&self, gen: usize) -> Result<()> {
        if !cfg!(debug_assertions) {
            return Ok(());
        }
        let mut file = fs::OpenOptions::new()
            .truncate(true)
            .write(true)
            .create(true)
            .open(format!("graphviz/champion-{}.dot", gen))?;
        let mut indent = "";
        write!(&mut file, "strict digraph {{\n")?;
        write!(&mut file, "{indent: <2}subgraph {{\n")?;
        write!(&mut file, "{indent: <4}rank=min;\n{indent: <4}")?;
        for node_index in 0..self.in_nodes {
            write!(&mut file, "{}; ", node_index)?;
        }
        write!(&mut file, "\n{indent: <2}}}\n")?;

        write!(&mut file, "{indent: <2}subgraph {{\n")?;
        write!(&mut file, "{indent: <4}rank=max;\n{indent: <4}")?;
        for node_index in self.in_nodes..self.in_nodes + self.out_nodes {
            write!(&mut file, "{}; ", node_index)?;
        }
        write!(&mut file, "\n{indent: <2}}}\n")?;

        for edge in self.edges.iter() {
            let edge = edge.borrow();
            write!(
                &mut file,
                "{indent: <2}{} -> {} [label=\"{}@{:.2}\"]\n",
                edge.in_node.borrow().id.0,
                edge.out_node.borrow().id.0,
                edge.id.0,
                edge.weight,
            )?;
        }
        write!(&mut file, "}}")?;
        Command::new("dot")
            .args([
                "-Tsvg",
                &format!("graphviz/champion-{}.dot", gen),
                "-o",
                &format!("graphviz/champion-{}.svg", gen),
            ])
            .output()?;
        Ok(())
    }
    pub fn new(genome: &Genome, node_counts: &NodeCounts) -> Result<Self> {
        log::debug!("Genome::new");
        let nodes = Self::build_nodes(node_counts);
        let edges = Self::build_edges(genome, &nodes)?;

        Ok(Self {
            nodes,
            edges,
            in_nodes: node_counts.in_nodes,
            out_nodes: node_counts.out_nodes,
        })
    }

    fn build_nodes(node_counts: &NodeCounts) -> Vec<Ref<Node>> {
        let node_count = node_counts.total_nodes;
        let mut nodes = vec![Rc::new(RefCell::new(Node::default())); node_count];

        let mut begin = 0;
        let mut end = node_counts.in_nodes;

        for i in begin..end {
            let mut node = RefCell::borrow_mut(Rc::make_mut(&mut nodes[i]));
            node.id = NodeId(i);
            node.node_type = NodeType::Input;
        }

        begin = end;
        end += node_counts.out_nodes;
        for i in begin..end {
            let mut node = RefCell::borrow_mut(Rc::make_mut(&mut nodes[i]));
            node.id = NodeId(i);
            node.node_type = NodeType::Output;
        }

        begin = end;
        end = node_counts.total_nodes;
        for i in begin..end {
            let mut node = RefCell::borrow_mut(Rc::make_mut(&mut nodes[i]));
            node.id = NodeId(i);
            node.node_type = NodeType::Hidden;
        }
        nodes
    }

    fn build_edges(genome: &Genome, nodes: &[Ref<Node>]) -> Result<Vec<Ref<Edge>>> {
        let mut sorted_edges = Vec::with_capacity(genome.genes.len());
        let mut edges_to_sort = Vec::with_capacity(genome.genes.len());
        for gene in genome.genes.iter().filter(|edge| edge.enabled) {
            let edge = Rc::new(RefCell::new(Edge {
                id: gene.id,
                weight: gene.weight,
                in_node: nodes[gene.in_node.0].clone(),
                out_node: nodes[gene.out_node.0].clone(),
                visited: false,
            }));
            {
                let mut node = RefCell::borrow_mut(&nodes[gene.out_node.0]);
                node.incoming.push(edge.clone());

                let mut node = RefCell::borrow_mut(&nodes[gene.in_node.0]);
                node.outgoing.push(edge.clone());
            }

            if nodes[gene.in_node.0].borrow().node_type == NodeType::Input {
                edges_to_sort.push(edge);
            }
        }

        // https://en.wikipedia.org/wiki/Topological_sorting#Kahn's_algorithm
        while let Some(edge) = edges_to_sort.pop() {
            {
                let mut ref_cell = RefCell::borrow_mut(&edge);
                if ref_cell.visited {
                    bail!("Neural net contains a cycle")
                } else {
                    ref_cell.visited = true;
                }
            }
            sorted_edges.push(edge.clone());
            let edge = edge.borrow();
            let next_node = edge.out_node.borrow();
            if next_node.incoming.iter().all(|edge| edge.borrow().visited) {
                edges_to_sort.extend_from_slice(&next_node.outgoing);
            }
        }
        Ok(sorted_edges)
    }

    fn input(&self, x: &[f64]) {
        log::debug!("Genome::input");
        for (i, x) in x.iter().enumerate() {
            let mut ref_cell = RefCell::borrow_mut(&self.nodes[i]);
            ref_cell.activation = *x;
            ref_cell.step = Propagation::Inert;
        }
    }

    fn propagate(&self) {
        log::debug!("Genome::propagate");
        for edge in self.edges.iter() {
            let edge = edge.borrow();

            let mut source = RefCell::borrow_mut(&edge.in_node);
            if source.step == Propagation::Activating {
                source.activation = sigmoid(source.activation);
                source.step = Propagation::Inert;
            }

            let mut target = RefCell::borrow_mut(&edge.out_node);
            if target.step == Propagation::Inert {
                target.activation = 0.0;
                target.step = Propagation::Activating;
            }

            target.activation += edge.weight * source.activation;
        }
    }

    fn output(&self, output: &mut [f64]) {
        log::debug!("Genome::output");
        let begin = self.in_nodes;
        let end = self.in_nodes + self.out_nodes;
        let mut index = self.in_nodes;
        for i in 0..self.out_nodes {
            index += i;
            output[i] = self.nodes[index].borrow().activation;
        }
    }
}

pub trait NeuralInterface {
    fn run(&self, input: &[f64], output: &mut [f64]);
}

impl NeuralInterface for Network {
    fn run(&self, input: &[f64], output: &mut [f64]) {
        self.input(input);
        self.propagate();
        self.output(output)
    }
}

#[derive(Debug)]
pub struct Edge {
    pub id: GeneId,
    pub weight: f64,
    pub in_node: Ref<Node>,
    pub out_node: Ref<Node>,
    pub visited: bool,
}

#[derive(Default, Debug, Clone)]
pub struct Node {
    pub id: NodeId,
    pub activation: f64,
    pub step: Propagation,
    pub node_type: NodeType,
    pub incoming: Vec<Ref<Edge>>,
    pub outgoing: Vec<Ref<Edge>>,
}

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub enum Propagation {
    #[default]
    Inert, // already normalized
    Activating, // receiving input from previous layer of nodes
}

pub fn sigmoid(num: f64) -> f64 {
    let one: f64 = 1.0.into();
    one / (one + (-num).exp())
}

#[macro_export]
macro_rules! assert_approx_eq {
    ($a:expr, $b:expr, $epsilon:expr) => {{
        let (a, b) = (&$a, &$b);
        let diff = (*a - *b).abs();
        let epsilon = $epsilon;
        assert!(
            (*a - *b).abs() < epsilon,
            "assertion failed: `(left !== right)` \
             (left: `{:?}`, right: `{:?}`, expected diff: `{:?}`, real diff: `{:?}`)",
            *a,
            *b,
            epsilon,
            diff,
        );
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sigmoid() {
        assert_approx_eq!(sigmoid(1.0), 0.7310585, 1e-7);
    }
}
