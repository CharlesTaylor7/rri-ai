use railroad_inc::{
    logger,
    neat::{Config, DomainConfig, Network, NeuralInterface, Parameters, Population},
};
use rand::Rng;

fn main() {
    logger::init();
    let mut config = Config {
        domain: DomainConfig {
            input_layer_size: 11,
            output_layer_size: 1,
            fitness: Box::new(|network| {
                let mut score = 1.0_f64;
                let mut input = [0.0; 11];
                let mut output = [0.0; 1];
                for _ in 0..100 {
                    let mut test_case: usize = rand::thread_rng().gen_range(0..2048);
                    let expected_address = test_case & 7;
                    let expected_output = (test_case >> (expected_address + 3)) & 1;
                    for i in 0..11 {
                        input[i] = (test_case & 1) as f64;
                        test_case >>= 2;
                    }
                    network.run(&input, &mut output);
                    if (output[0] > 0.5) == (expected_output == 1) {
                        score += 1.0;
                    }
                }
                score
            }),
        },
        parameters: Parameters::default(),
    };
    config.parameters.population = 100;
    let mut population = Population::new(config);
    for gen in 0..20 {
        log::info!("Gen {}", gen);
        population.advance_gen();

        if population.champion.fitness.actual > 95.0 {
            log::info!(
                "Champion with 95% accuracy. nodes: {}, edges: {} ",
                population.node_count,
                population.gene_count()
            );
        }
        let network = Network::new(&population.champion.genome, &population.node_counts()).unwrap();
        std::fs::create_dir_all("graphviz").unwrap();
        network.dump_graphviz(gen).unwrap();
    }
    population.dump_graphviz().unwrap();
}
