use railroad_inc::{
    agent::NeatAgent,
    logger,
    neat::{Config, Parameters, Population},
};

fn main() {
    logger::init();
    let mut config = Config {
        domain: NeatAgent::config(),
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
