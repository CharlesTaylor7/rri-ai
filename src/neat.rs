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
use std::fs::{self, File};
use std::io::Write;
use std::ops::{Add, Range};
use std::path::Path;
use std::rc::Rc;
use std::{default, usize};

pub struct DomainConfig {
    pub input_layer_size: usize,
    pub output_layer_size: usize,
    pub fitness: Box<dyn Fn(Rc<Network>) -> f64>,
}

pub struct Config {
    pub domain: DomainConfig,
    pub parameters: Parameters,
}

pub struct Parameters {
    pub speciation: Speciation,
    pub mutation: MutationWeights,
    pub population: usize,
    // percentage changed in new generation
    pub mutation_rate: f64,
    // percentage allowed to recombine
    pub reproduction_rate: f64,
}
impl Parameters {
    pub fn default() -> Self {
        Self {
            reproduction_rate: 0.5.into(),
            mutation_rate: 0.3.into(),
            population: 200,
            mutation: MutationWeights {
                adjust_weight: 0.7.into(),
                add_node: 0.2.into(),
                add_connection: 0.1.into(),
            },
            speciation: Speciation {
                c1: 1.0.into(),
                c2: 1.0.into(),
                c3: 3.0.into(),
                ct: 4.0.into(),
            },
        }
    }
}

// rates of mutation
pub struct MutationWeights {
    pub adjust_weight: f64,
    pub add_node: f64,
    pub add_connection: f64,
}

impl MutationWeights {
    pub fn sample(&self) -> Mutation {
        let mut value: f64 = rand::thread_rng().gen::<f64>().into();
        if value < self.adjust_weight {
            return Mutation::AdjustWeight;
        }
        value -= self.adjust_weight;

        if value < self.add_node {
            return Mutation::AddNode;
        }
        value -= self.add_node;

        return Mutation::AddGene;
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Mutation {
    AdjustWeight,
    AddNode,
    AddGene,
}

pub struct Population {
    pub config: Config,
    pub population: Vec<Rc<Genome>>,
    pub node_count: usize,
    pub edge_count: usize,
    pub champion: ScoredGenome,
}

#[derive(Default, Clone)]
pub struct Fitness {
    pub actual: f64,
    pub adjusted: f64,
}

impl Population {
    pub fn new(config: Config) -> Population {
        let node_count = config.domain.input_layer_size + config.domain.output_layer_size;
        let population = vec![Rc::new(Genome::default()); config.parameters.population];
        let champion = population[0].clone();
        Self {
            node_count,
            edge_count: 0,
            population,
            config,
            champion: ScoredGenome {
                fitness: Fitness::default(),
                genome: champion,
            },
        }
    }

    fn classify_species(&self) -> Vec<Species> {
        let mut groups: Vec<Species> = vec![];
        'outer: for genome in self.population.iter() {
            for species in groups.iter_mut() {
                let rep = species
                    .genomes
                    .choose(&mut rand::thread_rng())
                    .expect("Species group should not be empty");
                if self.config.parameters.speciation.compatible(genome, rep) {
                    species.genomes.push(genome.clone());
                    continue 'outer;
                }
            }
            groups.push(Species {
                genomes: vec![genome.clone()],
            });
        }
        log::info!("Classified into {} species", groups.len());
        groups
    }

    pub fn advance_gen(&mut self) {
        let groups = self.classify_species();
        let mut total_fitness: f64 = (0.).into();
        let mut group_fitness: Vec<f64> = vec![(0.).into(); groups.len()];
        let mut individual_fitness: Vec<Vec<ScoredGenome>> = Vec::with_capacity(groups.len());

        for (j, species) in groups.iter().enumerate() {
            individual_fitness.push(Vec::with_capacity(species.genomes.len()));
            for genome in species.genomes.iter() {
                let network = Network::new(genome, &self.config).expect("valid network");
                let actual = (self.config.domain.fitness)(Rc::new(network));
                let adjusted = actual / species.genomes.len() as f64;
                let scored = ScoredGenome {
                    fitness: Fitness { actual, adjusted },
                    genome: genome.clone(),
                };
                if scored.fitness.actual > self.champion.fitness.actual {
                    self.champion = scored.clone();
                }

                // log::info!("actual: {actual}, adjusted: {adjusted}");
                total_fitness += adjusted;
                group_fitness[j] += adjusted;
                individual_fitness[j].push(scored);
            }
        }

        let average_fitness: f64 = total_fitness / self.config.parameters.population as f64;

        log::info!("average_fitness: {average_fitness}");
        log::info!("total_fitness: {total_fitness}");
        self.population = Vec::with_capacity(self.config.parameters.population);
        for (j, species) in individual_fitness.into_iter().enumerate() {
            let mut genomes = species;

            genomes.sort_unstable_by_key(|g| R64::from(g.fitness.actual));
            let new_pop_size = (group_fitness[j] / average_fitness).ceil() as usize;
            let group_size: f64 = (genomes.len() as f64).into();
            let parents = (group_size * self.config.parameters.reproduction_rate).ceil() as usize;

            log::info!(
                "species: {j}, fitness: {}, pop: {} -> {}",
                group_fitness[j] * genomes.len() as f64,
                genomes.len(),
                new_pop_size
            );
            self.reproduce(&mut genomes[0..parents], new_pop_size);
        }

        self.mutate_population();
    }

    fn reproduce(&mut self, parents: &mut [ScoredGenome], target_size: usize) {
        let mut remaining = target_size;
        loop {
            parents.shuffle(&mut rand::thread_rng());
            for chunk in parents.chunks(2) {
                if chunk.get(1).is_none() {
                    // copy directly
                    self.population.push(chunk[0].genome.clone());
                } else {
                    // crossover 2 genomes
                    self.population
                        .push(Rc::new(self.crossover(&chunk[0], &chunk[1])));
                }
                remaining -= 1;
                if remaining == 0 {
                    return;
                }
            }
        }
    }

    fn crossover(&self, a: &ScoredGenome, b: &ScoredGenome) -> Genome {
        let mut i = 0;
        let mut j = 0;
        let mut genome = Genome {
            genes: vec![],
            hidden_nodes: std::cmp::max(a.genome.hidden_nodes, b.genome.hidden_nodes),
        };
        loop {
            match (a.genome.genes.get(i), b.genome.genes.get(j)) {
                (Some(gene_a), Some(gene_b)) => {
                    if gene_a.id == gene_b.id {
                        genome.genes.push(
                            if a.fitness.actual > b.fitness.actual {
                                gene_a
                            } else {
                                gene_b
                            }
                            .clone(),
                        );
                        i += 1;
                        j += 1;
                    } else {
                        if gene_a.id < gene_b.id {
                            genome.genes.push(gene_a.clone());
                            i += 1;
                        } else {
                            genome.genes.push(gene_b.clone());
                            j += 1;
                        }
                    }
                }
                (Some(gene_a), None) => {
                    genome.genes.push(gene_a.clone());
                    i += 1;
                }
                (None, Some(gene_b)) => {
                    genome.genes.push(gene_b.clone());
                    j += 1;
                }
                (None, None) => {
                    break;
                }
            }
        }

        genome
    }

    // mutate the whole population.
    fn mutate_population(&mut self) {
        self.population.shuffle(&mut rand::thread_rng());
        let count =
            (self.config.parameters.mutation_rate * self.population.len() as f64).ceil() as usize;

        for genome in self.population[0..count].iter_mut() {
            let genome = Rc::make_mut(genome);
            match dbg!(self.config.parameters.mutation.sample()) {
                Mutation::AdjustWeight => {
                    if let Some(gene) = genome.genes.choose_mut(&mut rand::thread_rng()) {
                        let gene = Rc::make_mut(gene);
                        gene.weight += rand::thread_rng().sample::<f64, _>(StandardNormal);
                    }
                }

                Mutation::AddNode => {
                    if let Some(gene) = genome.genes.choose_mut(&mut rand::thread_rng()) {
                        let gene = Rc::make_mut(gene);
                        gene.enabled = false;
                        let in_node = gene.in_node;
                        let out_node = gene.out_node;
                        let new_node = NodeId(self.node_count);
                        let weight = gene.weight;
                        genome.genes.push(Rc::new(Gene {
                            weight,
                            enabled: true,
                            id: GeneId(self.edge_count),
                            in_node,
                            out_node: new_node,
                        }));

                        genome.genes.push(Rc::new(Gene {
                            weight,
                            enabled: true,
                            id: GeneId(self.edge_count + 1),
                            in_node: new_node,
                            out_node,
                        }));
                        genome.hidden_nodes += 1;
                        self.node_count += 1;
                        self.edge_count += 2;
                    }
                }

                Mutation::AddGene => {
                    self.edge_count += 1;

                    let mut nodes = (0..self.node_count).collect::<Vec<_>>();
                    nodes.shuffle(&mut rand::thread_rng());
                    let last = genome.genes.len();
                    genome.genes.push(Rc::new(Gene {
                        weight: 0.5.into(),
                        enabled: true,
                        id: GeneId(self.edge_count),
                        in_node: NodeId(0),
                        out_node: NodeId(1),
                    }));

                    'outer: for i in 0..nodes.len() {
                        Rc::make_mut(&mut genome.genes[last]).in_node = NodeId(i);
                        for j in (i + 1)..nodes.len() {
                            let gene = Rc::make_mut(&mut genome.genes[last]);
                            Rc::make_mut(&mut genome.genes[last]).out_node = NodeId(j);
                            if Network::new(genome, &self.config).is_ok() {
                                break 'outer;
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct ScoredGenome {
    pub fitness: Fitness,
    pub genome: Rc<Genome>,
}

pub struct Species {
    pub genomes: Vec<Rc<Genome>>,
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
        let mut disjoint: usize = 0;
        let mut excess: usize = 0;
        let mut matching: usize = 0;

        let mut weight_diff: f64 = 0.0.into();

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

        let mut speciation_distance: f64 = 0.0.into();
        let num_genes = std::cmp::max(a.genes.len(), b.genes.len());
        if num_genes > 0 {
            (self.c1 * (excess as f64) + self.c2 * (disjoint as f64)) / (num_genes as f64);
        }

        if matching > 0 {
            speciation_distance += (self.c3 * weight_diff / (matching as f64));
        }

        speciation_distance < self.ct
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct NodeId(pub usize);

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
pub struct GeneId(pub usize);

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub enum NodeType {
    #[default]
    Input,
    Output,
    Hidden,
}

/* === Genome description === */
#[derive(Clone, Default)]
pub struct Genome {
    pub genes: Vec<Rc<Gene>>,
    pub hidden_nodes: usize,
}

#[derive(Clone, Debug)]
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
    // the third section is the hidden layers
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

impl Network {
    /// https://graphviz.org/doc/info/lang.html
    pub fn dump_graphviz<P: AsRef<Path>>(&self, p: P) -> Result<()> {
        let mut file = fs::OpenOptions::new().write(true).create(true).open(p)?;
        write!(&mut file, "digraph {{");
        for edge in self.edges.iter() {
            let edge = edge.borrow();
            write!(
                &mut file,
                "{:?} -> {:?} [label={:?}]",
                edge.in_node.borrow().id,
                edge.out_node.borrow().id,
                edge.id
            );
        }
        write!(&mut file, "}}");
        Ok(())
    }
    pub fn new(genome: &Genome, config: &Config) -> Result<Self> {
        let node_count =
            config.domain.input_layer_size + config.domain.output_layer_size + genome.hidden_nodes;

        let mut nodes = vec![Rc::new(RefCell::new(Node::default())); node_count];

        let mut begin = 0;
        let mut end = config.domain.input_layer_size;

        for i in begin..end {
            let mut node = RefCell::borrow_mut(Rc::make_mut(&mut nodes[i]));
            node.id = NodeId(i);
            node.node_type = NodeType::Input;
        }

        begin = end;
        end += config.domain.output_layer_size;
        for i in begin..end {
            let mut node = RefCell::borrow_mut(Rc::make_mut(&mut nodes[i]));
            node.id = NodeId(i);
            node.node_type = NodeType::Output;
        }

        begin = end;
        end += genome.hidden_nodes;
        for i in begin..end {
            let mut node = RefCell::borrow_mut(Rc::make_mut(&mut nodes[i]));
            node.id = NodeId(i);
            node.node_type = NodeType::Hidden;
        }

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
            log::info!("in_node: {:?}", nodes[gene.in_node.0]);
            log::info!("out_node: {:?}", nodes[gene.out_node.0]);
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
            let mut ref_cell = RefCell::borrow_mut(&edge);
            if ref_cell.visited {
                bail!("Neural net contains a cycle")
            } else {
                ref_cell.visited = true;
            }
            sorted_edges.push(edge.clone());
            let edge = edge.borrow();
            let next_node = edge.out_node.borrow();
            if next_node.incoming.iter().all(|edge| edge.borrow().visited) {
                edges_to_sort.extend_from_slice(&next_node.outgoing);
            }
        }

        Ok(Self {
            nodes,
            edges: sorted_edges,
            in_nodes: config.domain.input_layer_size,
            out_nodes: config.domain.output_layer_size,
        })
    }

    fn input(&self, x: &[f64]) {
        for (i, x) in x.iter().enumerate() {
            let mut ref_cell = RefCell::borrow_mut(&self.nodes[i]);
            ref_cell.activation = *x;
            ref_cell.step = Propagation::Inert;
        }
    }

    fn propagate(&self) {
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
        let begin = self.in_nodes;
        let end = self.in_nodes + self.out_nodes;
        let mut index = self.in_nodes;
        for i in 0..self.out_nodes {
            index += i;
            output[i] = self.nodes[index].borrow().activation;
        }
    }
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
