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
    config.parameters.population = 4;
    let mut population = Population::new(config);
    population.advance_gen();
    population.advance_gen();
}
