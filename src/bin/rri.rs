use railroad_inc::{
    agent::NeatAgentMethods,
    logger,
    neat::genome::{Config, Parameters, Population},
};

fn main() {
    logger::init();
    let mut config = Config {
        domain: NeatAgentMethods::config(),
        parameters: Parameters::default(),
    };
    config.parameters.population = 100;
    let mut population = Population::new(config);
    log::info!("Gen 0");
    for gen in 1..10 {
        population.advance_gen();
        log::info!("Gen {}", gen);
    }
}
