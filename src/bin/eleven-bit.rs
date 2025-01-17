use railroad_inc::{
    logger,
    neat::genome::{Config, DomainConfig, Parameters, Population},
    neat::network::{Network, NeuralInterface},
};
use rand::Rng;

fn fitness(network: &mut Network) -> f64 {
    let mut score = 1.0_f64;
    let mut input = [0.0; 11];
    for _ in 0..100 {
        let mut test_case: usize = rand::thread_rng().gen_range(0..2048);
        let expected_address = test_case & 7;
        let expected_output = (test_case >> (expected_address + 3)) & 1;
        for bit in &mut input {
            *bit = (test_case & 1) as f64;
            test_case >>= 2;
        }
        let output = network.run(&input);
        if (output[0] > 0.5) == (expected_output == 1) {
            score += 1.0;
        }
    }
    score
}

fn main() {
    logger::init();
    let mut parameters = Parameters::default();
    parameters.population = 10;
    let config = Config {
        parameters,
        domain: DomainConfig {
            input_layer_size: 11,
            output_layer_size: 1,
            fitness: Box::new(fitness),
        },
    };
    let mut population = Population::new(config);
    for gen in 0..10_000 {
        population.advance_gen();

        if population.champion.fitness.actual > 95.0 {
            log::info!(
                "Champion with 95% accuracy. nodes: {}, edges: {} ",
                population.node_count,
                population.gene_count()
            );
        }
        let network = Network::new(&population.champion.genome).unwrap();
        std::fs::create_dir_all("graphviz").unwrap();
        network.dump_graphviz(gen).unwrap();
    }
    population.dump_graphviz().unwrap();
}
