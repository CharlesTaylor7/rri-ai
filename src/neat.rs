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
use std::ops::{Add, Range};
use std::rc::Rc;
use std::usize;

pub struct DomainConfig {
    pub input_layer_size: usize,
    pub output_layer_size: usize,
    pub fitness: Box<dyn Fn(Rc<Network>) -> R64>,
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
    pub mutation_rate: R64,
    // percentage allowed to recombine
    pub reproduction_rate: R64,
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
    adjust_weight: R64,
    add_node: R64,
    add_connection: R64,
}

impl MutationWeights {
    pub fn sample(&self) -> Mutation {
        let mut value: R64 = rand::thread_rng().gen::<f64>().into();
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

#[derive(Clone, Copy)]
pub enum Mutation {
    AdjustWeight,
    AddNode,
    AddGene,
}

pub struct Population {
    config: Config,
    population: Vec<Rc<Genome>>,
    node_count: usize,
    edge_count: usize,
}

impl Population {
    pub fn new(config: Config) -> Population {
        let node_count = config.domain.input_layer_size + config.domain.output_layer_size;
        Self {
            node_count,
            edge_count: 0,
            population: vec![Rc::new(Genome::default()); config.parameters.population],
            config,
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
        groups
    }

    pub fn advance_gen(&mut self) {
        let groups = self.classify_species();
        let mut total_fitness: R64 = (0.).into();
        let mut group_fitness: Vec<R64> = vec![(0.).into(); groups.len()];
        let mut individual_fitness: Vec<Vec<R64>> = Vec::with_capacity(groups.len());

        for (j, species) in groups.iter().enumerate() {
            individual_fitness.push(Vec::with_capacity(species.genomes.len()));
            for genome in species.genomes.iter() {
                let network = Network::new(genome, &self.config).expect("valid network");
                let fitness: R64 = (self.config.domain.fitness)(Rc::new(network));
                let adjusted: R64 = fitness / species.genomes.len() as f64;
                individual_fitness[j].push(adjusted);
                group_fitness[j] += adjusted;
                total_fitness += adjusted;
            }
        }

        let average_fitness: R64 = total_fitness / self.config.parameters.population as f64;

        self.population = Vec::with_capacity(self.config.parameters.population);
        for (j, species) in groups.into_iter().enumerate() {
            let mut genomes = species
                .genomes
                .into_iter()
                .enumerate()
                .map(|(i, genome)| ScoredGenome {
                    genome,
                    fitness: individual_fitness[j][i],
                })
                .collect::<Vec<ScoredGenome>>();
            genomes.sort_unstable_by_key(|g| g.fitness);
            log::info!("average_fitness: {}", average_fitness);
            let new_pop_size = (group_fitness[j] / average_fitness).ceil().into_inner() as usize;
            let group_size: R64 = (genomes.len() as f64).into();
            let parents = (group_size * self.config.parameters.reproduction_rate)
                .ceil()
                .into_inner() as usize;

            self.reproduce(&mut genomes[0..parents], new_pop_size);
        }
    }

    fn reproduce(&mut self, parents: &mut [ScoredGenome], target_size: usize) {
        let mut remaining = target_size;
        loop {
            log::info!("Reproduce");
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
            log::info!("Crossover");
            match (a.genome.genes.get(i), b.genome.genes.get(j)) {
                (Some(gene_a), Some(gene_b)) => {
                    if gene_a.id == gene_b.id {
                        genome.genes.push(
                            if a.fitness > b.fitness {
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
        let count = (self.config.parameters.mutation_rate * self.population.len() as f64)
            .ceil()
            .into_inner() as usize;

        for genome in self.population[0..count].iter_mut() {
            let genome = Rc::make_mut(genome);
            match self.config.parameters.mutation.sample() {
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

pub struct ScoredGenome {
    fitness: R64,
    genome: Rc<Genome>,
}

pub struct Species {
    genomes: Vec<Rc<Genome>>,
}

pub struct Speciation {
    c1: R64, // disjoint
    c2: R64, // excess
    c3: R64, // weight
    // compatibility threshold
    ct: R64,
}

impl Speciation {
    fn compatible(&self, a: &Genome, b: &Genome) -> bool {
        // calculate speciation distance
        // counts
        let mut disjoint: usize = 0;
        let mut excess: usize = 0;
        let mut matching: usize = 0;

        let mut weight_diff: R64 = 0.0.into();

        // genome indices
        let mut i = 0;
        let mut j = 0;
        loop {
            log::info!("Checking compatibility");
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

        let mut speciation_distance: R64 = 0.0.into();
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

#[derive(Default, Debug, PartialEq, Eq)]
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

#[derive(Clone)]
pub struct Gene {
    pub id: GeneId,
    pub in_node: NodeId,
    pub out_node: NodeId,
    pub weight: R64,
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

#[derive(Debug)]
pub struct Edge {
    pub id: GeneId,
    pub weight: R64,
    pub in_node: Ref<Node>,
    pub out_node: Ref<Node>,
    pub visited: bool,
}

#[derive(Default, Debug)]
pub struct Node {
    pub id: NodeId,
    pub weight: R64,
    pub activation: R64,
    pub node_type: NodeType,
    pub incoming: Vec<Ref<Edge>>,
    pub outgoing: Vec<Ref<Edge>>,
}

impl Network {
    pub fn process(&self, input: &[R64]) -> Rc<[R64]> {
        self.input(input);
        self.propagate();
        self.output()
    }
    pub fn new(genome: &Genome, config: &Config) -> Result<Self> {
        let node_count =
            config.domain.input_layer_size + config.domain.output_layer_size + genome.hidden_nodes;

        let mut nodes = vec![Rc::new(RefCell::new(Node::default())); node_count];

        let mut begin = 0;
        let mut end = config.domain.input_layer_size;

        for i in begin..end {
            let mut node = RefCell::borrow_mut(&nodes[i]);
            node.id = NodeId(i);
            node.node_type = NodeType::Input;
        }

        begin = end;
        end += config.domain.output_layer_size;
        for i in begin..end {
            let mut node = RefCell::borrow_mut(&nodes[i]);
            node.id = NodeId(i);
            node.node_type = NodeType::Output;
        }

        begin = end;
        end += genome.hidden_nodes;
        for i in begin..end {
            let mut node = RefCell::borrow_mut(&nodes[i]);
            node.id = NodeId(i);
            node.node_type = NodeType::Hidden;
        }

        let mut edges = Vec::with_capacity(genome.genes.len());
        let mut to_process = Vec::with_capacity(genome.genes.len());
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
            if nodes[gene.in_node.0].borrow().node_type == NodeType::Input {
                to_process.push(nodes[gene.in_node.0].clone());
            }
        }

        // The hidden layer nodes need to be re-added but in topological order
        nodes.truncate(config.domain.input_layer_size + config.domain.output_layer_size);

        // https://en.wikipedia.org/wiki/Topological_sorting#Kahn's_algorithm
        while let Some(node) = to_process.pop() {
            log::info!(
                "node: {:?}, remaining: {:?}",
                node.borrow().id,
                to_process.len()
            );

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
            in_nodes: config.domain.input_layer_size,
            out_nodes: config.domain.output_layer_size,
        })
    }

    fn input(&self, x: &[R64]) {
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
            let mut value: R64 = 0_f64.into();
            for edge in node.borrow().incoming.iter() {
                value += edge.borrow().weight * edge.borrow().in_node.borrow().activation;
            }

            let mut node = RefCell::borrow_mut(&node);
            node.activation = sigmoid(value);
        }
    }

    fn output(&self) -> Rc<[R64]> {
        let begin = self.in_nodes;
        let end = self.in_nodes + self.out_nodes;
        self.nodes[begin..end]
            .iter()
            .map(|node| node.borrow().activation)
            .collect()
    }
}

pub fn sigmoid(num: R64) -> R64 {
    let one: R64 = 1.0.into();
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
        assert_approx_eq!(sigmoid(1.0.into()), R64::from(0.7310585), 1e-7);
    }
}
