use railroad_inc::{
    agent::NeatAgent,
    logger,
    neat::{Config, MutationWeights, Parameters, Population},
};

fn main() {
    logger::init();
    let mut config = Config {
        domain: NeatAgent::config(),
        parameters: Parameters::default(),
    };
    config.parameters.population = 100;
    config.parameters.mutation = MutationWeights {
        add_node: 0.5.into(),
        add_connection: 0.5.into(),
        adjust_weight: 0.0.into(),
    };
    let mut population = Population::new(config);
    log::info!("Gen 0");
    for gen in 1..10 {
        population.advance_gen();
        log::info!("Gen {}", gen);
    }
}
