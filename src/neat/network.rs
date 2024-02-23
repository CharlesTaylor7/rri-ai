use super::genome::*;
use anyhow::{bail, Result};
use core::num;
use decorum::R64;
use num_traits::real::Real;
use num_traits::sign::Signed;
use rand::seq::SliceRandom;
use rand::Rng;
use rand_distr::StandardNormal;
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fs::{self, File};
use std::io::Write;
use std::ops::{Add, Range};
use std::path::Path;
use std::process::Command;
use std::rc::Rc;
use std::{default, usize};

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub enum Activation {
    #[default]
    Inert, // already normalized
    Activating, // receiving input from previous layer of nodes
}

#[derive(Debug, Clone, Copy)]
pub struct NodeIndex(pub usize);

#[derive(Debug)]
pub struct Edge {
    id: GeneId,
    weight: f64,
    in_node: NodeIndex,
    out_node: NodeIndex,
}

#[derive(Debug)]
pub struct Network {
    node_counts: NodeCounts,
    node_values: Vec<f64>,
    node_activations: Vec<Activation>,
    sorted_edges: Vec<Edge>,
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
        for node_index in self.node_counts.input_range() {
            write!(&mut file, "{}; ", node_index)?;
        }
        write!(&mut file, "\n{indent: <2}}}\n")?;

        write!(&mut file, "{indent: <2}subgraph {{\n")?;
        write!(&mut file, "{indent: <4}rank=max;\n{indent: <4}")?;
        for node_index in self.node_counts.output_range() {
            write!(&mut file, "{}; ", node_index)?;
        }
        write!(&mut file, "\n{indent: <2}}}\n")?;

        for edge in self.sorted_edges.iter() {
            write!(
                &mut file,
                "{indent: <2}{} -> {} [label=\"{}@{:.2}\"]\n",
                edge.in_node.0, edge.out_node.0, edge.id.0, edge.weight,
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
    pub fn new(genome: &Genome) -> Result<Self> {
        log::trace!("Genome::new");
        let node_counts = genome.node_counts();
        let mut sorted_edges = Vec::with_capacity(genome.genes.len());
        let mut edges_to_sort = Vec::with_capacity(genome.genes.len());
        let mut node_indices: HashMap<NodeId, NodeIndex> =
            HashMap::with_capacity(genome.hidden_nodes.len());

        for i in 0..node_counts.in_nodes + node_counts.out_nodes {
            node_indices.insert(NodeId(i), NodeIndex(i));
        }
        for (i, node_id) in genome.hidden_nodes.iter().enumerate() {
            node_indices.insert(
                *node_id,
                NodeIndex(i + node_counts.in_nodes + node_counts.out_nodes),
            );
        }

        let mut incoming = vec![Vec::new(); node_counts.total_nodes];
        let mut outgoing = vec![Vec::new(); node_counts.total_nodes];
        let mut visited: HashSet<GeneId> = HashSet::with_capacity(genome.genes.len());

        for gene in genome.genes.iter().filter(|edge| edge.enabled) {
            incoming[gene.out_node.0].push(gene.clone());
            outgoing[gene.in_node.0].push(gene.clone());

            if node_counts.input_range().contains(&gene.in_node.0) {
                edges_to_sort.push(gene.clone());
            }
        }

        if cfg!(debug_assertions) {
            let network_dead_node = node_counts
                .hidden_range()
                .find(|i| !incoming[*i].is_empty() && outgoing[*i].is_empty());

            if let Some(node_id) = network_dead_node {
                bail!("Dead node: {node_id}");
            }
        }

        // https://en.wikipedia.org/wiki/Topological_sorting#Kahn's_algorithm
        while let Some(edge) = edges_to_sort.pop() {
            let fresh = visited.insert(edge.id);
            if !fresh {
                bail!("Neural net contains a cycle")
            }
            sorted_edges.push(Edge {
                id: edge.id,
                weight: edge.weight,
                in_node: node_indices[&edge.in_node],
                out_node: node_indices[&edge.out_node],
            });
            if incoming[edge.out_node.0]
                .iter()
                .all(|edge| visited.contains(&edge.id))
            {
                edges_to_sort.extend_from_slice(outgoing[edge.out_node.0].as_slice());
            }
        }

        let mut node_values = vec![0.0; node_counts.total_nodes];
        for i in node_counts.output_range() {
            node_values[i] = 0.5;
        }

        Ok(Self {
            node_values,
            node_activations: vec![Activation::Inert; node_counts.total_nodes],
            node_counts: node_counts.clone(),
            sorted_edges,
        })
    }

    fn input(&mut self, x: &[f64]) {
        log::trace!("Genome::input");
        self.node_values[self.node_counts.input_range()].copy_from_slice(x);
    }

    fn propagate(&mut self) {
        log::trace!("Genome::propagate");
        for edge in self.sorted_edges.iter() {
            let source_index = edge.in_node.0;
            if self.node_activations[source_index] == Activation::Activating {
                self.node_values[source_index] = sigmoid(self.node_values[source_index]);
                self.node_activations[source_index] = Activation::Inert;
            }

            let target_index = edge.out_node.0;
            if self.node_activations[target_index] == Activation::Inert {
                self.node_values[target_index] = 0.0;
                self.node_activations[target_index] = Activation::Activating;
            }

            self.node_values[target_index] += edge.weight * self.node_values[source_index];
        }
        for i in self.node_counts.output_range() {
            if self.node_activations[i] == Activation::Activating {
                self.node_values[i] = sigmoid(self.node_values[i]);
                self.node_activations[i] = Activation::Inert;
            }
        }
    }

    fn output(&self) -> &[f64] {
        log::trace!("Genome::output");
        self.node_values[self.node_counts.output_range()].borrow()
    }
}

pub trait NeuralInterface {
    fn run(&mut self, input: &[f64]) -> &[f64];
}

impl NeuralInterface for Network {
    fn run(&mut self, input: &[f64]) -> &[f64] {
        self.input(input);
        self.propagate();
        self.output()
    }
}

pub fn sigmoid(num: f64) -> f64 {
    1.0 / (1.0 + -num.exp())
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
