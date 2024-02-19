use crate::neat::{Config, DomainConfig, Genome, Network};
use crate::rri::{GameState, RRIAgent, Tile, Turn};
use decorum::R64;
use std::rc::Rc;

pub struct NeatAgent {
    network: Rc<Network>,
    score_modifier: isize,
}

impl RRIAgent for NeatAgent {
    fn prompt(&mut self, state: &GameState) -> Turn {
        let mut input = [R64::from(0.0); Self::INPUT_LAYER_SIZE];
        let mut index: usize = 0;
        // regular dice:
        for x in 0..7 {
            for y in 0..7 {
                if let Some(pattern) = state.drawn_routes.get(&Tile { x, y }) {
                    input[index + pattern.face as usize] = R64::from(1.0);
                    //input[index +
                }
                index += 34;
            }
        }
        for _ in 0..3 {
            //input[index] = state.dice_roll;
            index += 6;
        }

        let output = self.network.process(&input);
        todo!()
    }

    // errors are penalized; but the game doesn't halt
    fn handle_error(&mut self, error: anyhow::Error) {
        log::error!("Agent is penalized for: {}", error);
        self.score_modifier -= 10;
    }
}

impl NeatAgent {
    // 34 patterns
    // 49 tiles
    // 3 regular dice with 6 unique sides
    // 1 special die with 3 unique sides
    const INPUT_LAYER_SIZE: usize =
          34 * 49 // drawn on grid
        +  3 *  6 // regular dice
        +  1 *  3 // special die
    ;

    const OUTPUT_LAYER_SIZE: usize = 4 * 49 * 34; // 4 dice patterns placed on the grid.

    const GAME_COUNT: usize = 100;

    pub fn new(network: Rc<Network>) -> Self {
        Self {
            network,
            score_modifier: 0,
        }
    }

    // average score across many games
    pub fn fitness(&mut self) -> R64 {
        let mut score = 0_isize;
        for _ in 0..Self::GAME_COUNT {
            let mut game = GameState::new();
            game.play(self);
            score += game.score();
        }
        R64::from((score + self.score_modifier) as f64 / Self::GAME_COUNT as f64)
    }
    fn config() -> DomainConfig {
        DomainConfig {
            input_layer_size: Self::INPUT_LAYER_SIZE,
            output_layer_size: Self::OUTPUT_LAYER_SIZE,
            fitness: Box::new(|n| NeatAgent::new(n).fitness()),
        }
    }
}
