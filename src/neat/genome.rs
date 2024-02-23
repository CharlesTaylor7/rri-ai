use super::network::*;
use anyhow::Result;
use decorum::R64;
use rand::seq::SliceRandom;
use rand::Rng;
use rand_distr::Normal;
use std::collections::HashMap;
use std::fs::{self};
use std::io::Write;
use std::ops::Range;
use std::process::Command;
use std::rc::Rc;
use std::usize;

pub struct DomainConfig {
    pub input_layer_size: usize,
    pub output_layer_size: usize,
    pub fitness: Box<dyn Fn(&mut Network) -> f64>,
}

#[derive(Clone, Debug)]
pub struct NodeCounts {
    pub in_nodes: usize,
    pub out_nodes: usize,
    pub total_nodes: usize,
}

impl NodeCounts {
    pub fn input_range(&self) -> Range<usize> {
        0..self.in_nodes
    }

    pub fn output_range(&self) -> Range<usize> {
        self.in_nodes..self.in_nodes + self.out_nodes
    }

    pub fn hidden_range(&self) -> Range<usize> {
        self.in_nodes + self.out_nodes..self.total_nodes
    }

    pub fn hidden_nodes(&self) -> usize {
        self.total_nodes - self.in_nodes - self.out_nodes
    }
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
            reproduction_rate: 0.5,
            mutation_rate: 0.3,
            population: 100,
            mutation: MutationWeights::new([
                (Mutation::AdjustWeight, 18),
                (Mutation::AddGene, 1),
                (Mutation::AddNode, 1),
            ]),
            speciation: Speciation {
                c1: 1.0,
                c2: 1.0,
                c3: 3.0,
                ct: 4.0,
            },
        }
    }
}

// rates of mutation
pub struct MutationWeights {
    storage: Vec<Mutation>,
}

impl MutationWeights {
    pub fn new<const SIZE: usize>(arr: [(Mutation, usize); SIZE]) -> Self {
        let size = arr.iter().map(|(_, w)| w).sum();

        let mut storage = Vec::with_capacity(size);
        for (mutation, weight) in arr {
            for _ in 0..weight {
                storage.push(mutation);
            }
        }
        Self { storage }
    }

    pub fn sample(&self) -> Mutation {
        *self
            .storage
            .choose(&mut rand::thread_rng())
            .expect("distribution should not be empty")
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Mutation {
    AdjustWeight,
    AddNode,
    AddGene,
}

#[derive(Default, Clone, Debug)]
pub struct Fitness {
    pub actual: f64,
    pub adjusted: f64,
}
pub struct Genes {
    map: HashMap<(NodeId, NodeId), GeneId>,
}

impl Genes {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
    pub fn iter(&self) -> impl Iterator<Item = (&(NodeId, NodeId), &GeneId)> {
        self.map.iter()
    }

    pub fn insert(&mut self, in_node: NodeId, out_node: NodeId) {
        let gene_id = GeneId(self.map.len());
        self.map.insert((in_node, out_node), gene_id);
    }

    pub fn contains(&self, a: NodeId, b: NodeId) -> bool {
        self.map.contains_key(&(a, b)) || self.map.contains_key(&(b, a))
    }

    pub fn count(&self) -> usize {
        self.map.len()
    }
}

pub struct Population {
    pub generation: usize,
    pub config: Config,
    pub champion: ScoredGenome,
    pub genomes: Vec<Rc<Genome>>,
    pub genes: Genes,
    pub node_count: usize,
}

impl Population {
    pub fn dump_graphviz(&self) -> Result<()> {
        if !cfg!(debug_assertions) {
            return Ok(());
        }
        let mut file = fs::OpenOptions::new()
            .truncate(true)
            .write(true)
            .create(true)
            .open("graphviz/pop.dot")?;
        let indent = "";
        write!(&mut file, "strict digraph {{\n")?;
        write!(&mut file, "{indent: <2}subgraph {{\n")?;
        write!(&mut file, "{indent: <4}rank=min;\n{indent: <4}")?;
        let node_counts = self.node_counts();
        for node_index in 0..node_counts.in_nodes {
            write!(&mut file, "{}; ", node_index)?;
        }
        write!(&mut file, "\n{indent: <2}}}\n")?;

        write!(&mut file, "{indent: <2}subgraph {{\n")?;
        write!(&mut file, "{indent: <4}rank=max;\n{indent: <4}")?;
        for node_index in node_counts.in_nodes..node_counts.in_nodes + node_counts.out_nodes {
            write!(&mut file, "{}; ", node_index)?;
        }
        write!(&mut file, "\n{indent: <2}}}\n")?;

        for ((node1, node2), gene) in self.genes.iter() {
            write!(
                &mut file,
                "{indent: <2}{} -> {} [label=\"{}\"]\n",
                node1.0, node2.0, gene.0,
            )?;
        }
        write!(&mut file, "}}")?;
        drop(file);
        Command::new("dot")
            .args(["-Tsvg", "graphviz/pop.dot", "-o", "graphviz/pop.svg"])
            .output()?;
        Ok(())
    }

    pub fn gene_count(&self) -> usize {
        self.genes.count()
    }

    pub fn new(config: Config) -> Population {
        let node_count = config.domain.input_layer_size + config.domain.output_layer_size;
        let initial_genome = Rc::new(Genome::new(&config.domain));
        let population = vec![initial_genome; config.parameters.population];
        let champion = population[0].clone();
        Self {
            generation: 0,
            node_count,
            genomes: population,
            config,
            genes: Genes::new(),
            champion: ScoredGenome {
                fitness: Fitness::default(),
                genome: champion,
            },
        }
    }

    pub fn node_counts(&self) -> NodeCounts {
        NodeCounts {
            in_nodes: self.config.domain.input_layer_size,
            out_nodes: self.config.domain.output_layer_size,
            total_nodes: self.node_count,
        }
    }

    fn classify_species(&self) -> Vec<Species> {
        let mut groups: Vec<Species> = vec![];
        'outer: for genome in self.genomes.iter() {
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
        log::debug!("Classified into {} species", groups.len());
        groups
    }

    pub fn advance_gen(&mut self) {
        let groups = self.classify_species();
        let mut total_fitness: f64 = 0.0;
        let mut group_fitness: Vec<f64> = vec![0.0; groups.len()];
        let mut individual_fitness: Vec<Vec<ScoredGenome>> = Vec::with_capacity(groups.len());

        for (j, species) in groups.iter().enumerate() {
            individual_fitness.push(Vec::with_capacity(species.genomes.len()));
            for genome in species.genomes.iter() {
                let mut network = Network::new(genome).expect("valid network");
                let actual = (self.config.domain.fitness)(&mut network);
                let adjusted = actual / species.genomes.len() as f64;
                let scored = ScoredGenome {
                    fitness: Fitness { actual, adjusted },
                    genome: genome.clone(),
                };

                if scored.fitness.actual > self.champion.fitness.actual {
                    log::info!(
                        "New champion (Gen {}): actual: {actual}, adjusted: {adjusted}",
                        self.generation
                    );
                    self.champion = scored.clone();
                }

                total_fitness += adjusted;
                group_fitness[j] += adjusted;
                individual_fitness[j].push(scored);
            }
        }

        let average_fitness: f64 = total_fitness / self.config.parameters.population as f64;

        self.genomes = Vec::with_capacity(self.config.parameters.population);
        for (j, species) in individual_fitness.into_iter().enumerate() {
            let mut genomes = species;

            genomes.sort_unstable_by_key(|g| R64::from(g.fitness.actual));
            let new_pop_size = (group_fitness[j] / average_fitness).ceil() as usize;
            let group_size: f64 = genomes.len() as f64;
            let parents = (group_size * self.config.parameters.reproduction_rate).ceil() as usize;

            log::debug!("species: {j}, pop: {} -> {}", genomes.len(), new_pop_size);
            self.reproduce(&mut genomes[0..parents], new_pop_size);
        }

        self.mutate_population();
        self.generation += 1;
    }

    fn reproduce(&mut self, parents: &mut [ScoredGenome], target_size: usize) {
        let mut remaining = target_size;
        loop {
            log::trace!("loop: reproduce");
            parents.shuffle(&mut rand::thread_rng());
            for chunk in parents.chunks(2) {
                if chunk.get(1).is_none() {
                    // copy directly
                    self.genomes.push(chunk[0].genome.clone());
                } else {
                    // crossover 2 genomes
                    self.genomes
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
        let genome = Genome {
            genes: self.merge_genes(a, b),
            hidden_nodes: self.merge_nodes(a, b),
            in_nodes: a.genome.in_nodes,
            out_nodes: a.genome.out_nodes,
        };

        if cfg!(debug_assertions) {
            if let Err(error) = Network::new(&genome) {
                log::error!("{}", error);
                log::info!("a: {:?}\nb: {:?}", a.genome, b.genome);
                panic!();
            }
        }
        genome
    }

    fn merge_nodes(&self, a: &ScoredGenome, b: &ScoredGenome) -> Vec<NodeId> {
        let mut hidden_nodes =
            Vec::with_capacity(a.genome.hidden_nodes.len() + b.genome.hidden_nodes.len());

        let mut i = 0;
        let mut j = 0;
        loop {
            log::trace!("loop: merge_nodes");
            match (a.genome.hidden_nodes.get(i), b.genome.hidden_nodes.get(j)) {
                (Some(node_a), Some(node_b)) if node_a == node_b => {
                    hidden_nodes.push(*node_a);
                    i += 1;
                    j += 1;
                }
                (Some(node_a), Some(node_b)) if node_a < node_b => {
                    hidden_nodes.push(*node_a);
                    i += 1;
                }

                (Some(_node_a), Some(node_b)) => {
                    hidden_nodes.push(*node_b);
                    j += 1;
                }

                (Some(_), None) => {
                    hidden_nodes.extend(&a.genome.hidden_nodes[i..]);
                    break;
                }

                (None, Some(_)) => {
                    hidden_nodes.extend(&b.genome.hidden_nodes[i..]);
                    break;
                }
                (None, None) => {
                    break;
                }
            }
        }
        // TODO: merge

        hidden_nodes
    }

    fn merge_genes(&self, a: &ScoredGenome, b: &ScoredGenome) -> Vec<Rc<Gene>> {
        let mut i = 0;
        let mut j = 0;
        let mut genes = Vec::with_capacity(a.genome.genes.len() + b.genome.genes.len());
        loop {
            match (a.genome.genes.get(i), b.genome.genes.get(j)) {
                (Some(gene_a), Some(gene_b)) => {
                    if gene_a.id == gene_b.id {
                        genes.push(
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
                            genes.push(gene_a.clone());
                            i += 1;
                        } else {
                            genes.push(gene_b.clone());
                            j += 1;
                        }
                    }
                }
                (Some(gene_a), None) => {
                    genes.push(gene_a.clone());
                    i += 1;
                }
                (None, Some(gene_b)) => {
                    genes.push(gene_b.clone());
                    j += 1;
                }
                (None, None) => {
                    break;
                }
            }
        }

        genes
    }

    // mutate the whole population.
    fn mutate_population(&mut self) {
        self.genomes.shuffle(&mut rand::thread_rng());
        let count =
            (self.config.parameters.mutation_rate * self.genomes.len() as f64).ceil() as usize;

        let mut node_counts = self.node_counts();
        for genome_index in 0..count {
            let genome = Rc::make_mut(&mut self.genomes[genome_index]);
            match self.config.parameters.mutation.sample() {
                Mutation::AdjustWeight => {
                    if let Some(gene) = genome.genes.choose_mut(&mut rand::thread_rng()) {
                        let gene = Rc::make_mut(gene);

                        let distribution = Normal::new(0.0, 0.1).unwrap();
                        gene.weight += rand::thread_rng().sample::<f64, _>(distribution);
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

                        let gene_id = GeneId(self.genes.count());
                        genome.genes.push(Rc::new(Gene {
                            weight,
                            enabled: true,
                            id: gene_id,
                            in_node,
                            out_node: new_node,
                        }));
                        self.genes.insert(in_node, new_node);

                        let gene_id = GeneId(self.genes.count());
                        genome.genes.push(Rc::new(Gene {
                            weight,
                            enabled: true,
                            id: gene_id,
                            in_node: new_node,
                            out_node,
                        }));
                        self.genes.insert(new_node, out_node);

                        genome.hidden_nodes.push(new_node);
                        self.node_count += 1;
                    }
                }

                Mutation::AddGene => {
                    node_counts.total_nodes = self.node_count;

                    // randomly select an input or hidden node.
                    // randomly select an output or hidden node.
                    // create the network and check for cycles.
                    // If the connection exists, fallback to tweaking the weight or abort
                    // If the connection creates a cycle, and its between two hidden nodes, then
                    // try reversing the direction of the connection.
                    // Otherwise just skip adding the gene.
                    //
                    let h = node_counts.hidden_nodes();
                    let i = node_counts.in_nodes;
                    let chosen_input = rand::thread_rng().gen_range(0..h + i);
                    let input_index = if chosen_input < i {
                        chosen_input
                    } else {
                        chosen_input + node_counts.out_nodes
                    };

                    let output_index = rand::thread_rng().gen_range(i..node_counts.total_nodes);

                    let in_node = NodeId(input_index);
                    let out_node = NodeId(output_index);

                    if in_node == out_node || self.genes.contains(in_node, out_node) {
                        continue;
                    }
                    let gene_id = GeneId(self.genes.count());
                    genome.genes.push(Rc::new(Gene {
                        weight: 0.5,
                        enabled: true,
                        id: gene_id,
                        in_node,
                        out_node,
                    }));

                    if let Err(error) = Network::new(genome) {
                        log::info!("in: {in_node:?}, out: {out_node:?}");
                        log::error!("Error during AddGene mutation:\n{}", error);
                        genome.genes.remove(genome.genes.len() - 1);
                    } else {
                        self.genes.insert(in_node, out_node);
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
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

        let mut weight_diff: f64 = 0.0;

        // genome indices
        let mut i = 0;
        let mut j = 0;
        loop {
            let _gene_a = a.genes.get(i);
            let _gene_b = b.genes.get(j);
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

                (Some(_gene_a), None) => {
                    excess += 1;
                    i += 1;
                }
                (None, Some(_gene_b)) => {
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
            speciation_distance +=
                (self.c1 * (excess as f64) + self.c2 * (disjoint as f64)) / (num_genes as f64);
        }

        if matching > 0 {
            speciation_distance += self.c3 * weight_diff / (matching as f64);
        }
        speciation_distance < self.ct
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NodeId(pub usize);

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy, Hash)]
pub struct GeneId(pub usize);

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub enum NodeType {
    #[default]
    Input,
    Output,
    Hidden,
}

/* === Genome description === */
#[derive(Clone, Debug)]
pub struct Genome {
    pub genes: Vec<Rc<Gene>>,
    pub hidden_nodes: Vec<NodeId>,
    pub in_nodes: usize,
    pub out_nodes: usize,
}

impl Genome {
    pub fn new(domain: &DomainConfig) -> Genome {
        Self {
            in_nodes: domain.input_layer_size,
            out_nodes: domain.output_layer_size,
            genes: vec![],
            hidden_nodes: vec![],
        }
    }
    pub fn node_counts(&self) -> NodeCounts {
        NodeCounts {
            in_nodes: self.in_nodes,
            out_nodes: self.out_nodes,
            total_nodes: self.in_nodes + self.out_nodes + self.hidden_nodes.len(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Gene {
    pub id: GeneId,
    pub in_node: NodeId,
    pub out_node: NodeId,
    pub weight: f64,
    pub enabled: bool,
}
