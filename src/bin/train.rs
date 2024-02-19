use railroad_inc::{
    agent::NeatAgent,
    neat::{Config, Parameters, Population},
};

fn main() {
    println!("Hello");
    let config = Config {
        domain: NeatAgent::config(),
        parameters: Parameters::default(),
    };
    let mut population = Population::new(config);
    population.advance_gen();
    population.advance_gen();
}
