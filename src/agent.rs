use crate::neat::{Config, DomainConfig, Genome, Network};
use crate::routes::DIE_PATTERNS;
use crate::rri::{DrawAction, GameState, RRIAgent, Tile, Turn};
use decorum::R64;
use std::rc::Rc;

pub struct NeatAgent {
    network: Rc<Network>,
    score_modifier: isize,
}

impl RRIAgent for NeatAgent {
    fn prompt(&mut self, state: &GameState) -> Turn {
        let input = Self::to_input(state);
        let output = self.network.process(&input);
        Self::from_output(&output)
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

    #[inline]
    pub fn to_input(state: &GameState) -> [f64; Self::INPUT_LAYER_SIZE] {
        let mut input = [0.0; Self::INPUT_LAYER_SIZE];
        for action in state.drawn_routes.iter() {
            let tile_offset = 34 * (action.tile.x + action.tile.y * 7) as usize;
            input[tile_offset + action.pattern.face as usize] = 1.0;
        }
        let mut index: usize = 34 * 49;

        // regular dice:
        for face in state.dice.regular {
            input[index + face as usize] = 1.0;
            index += 6;
        }

        // special die
        input[index + (state.dice.special as usize - 6)] = 1.0;
        input
    }

    #[inline]
    pub fn from_output(output: &[f64]) -> Turn {
        let mut actions = Vec::with_capacity(4);
        // largest value in the grid is the placement,
        // if its larger than 0.5 Otherwise place none.
        let mut index = 0;
        let mut action_window = 49 * 34;
        for _ in 0..4 {
            let pair = output[index..index + action_window]
                .iter()
                .enumerate()
                .max_by_key(|(i, v)| R64::from(**v))
                .expect("Not empty");

            if *pair.1 > 0.5 {
                let pattern = &DIE_PATTERNS[pair.0 % 34];
                let tile = pair.0 / 34;
                let x = (tile / 7) as u8;
                let y = (tile % 7) as u8;
                actions.push(DrawAction {
                    tile: Tile { x, y },
                    pattern,
                })
            }
            index += action_window;
        }
        Turn { actions }
    }

    pub fn new(network: Rc<Network>) -> Self {
        Self {
            network,
            score_modifier: 0,
        }
    }
    const GAME_COUNT: usize = 100;

    // average score across many games
    pub fn fitness(&mut self) -> f64 {
        let mut score = 0_isize;
        for _ in 0..Self::GAME_COUNT {
            let mut game = GameState::new();
            game.play(self);
            score += game.score();
        }
        (score + self.score_modifier) as f64 / Self::GAME_COUNT as f64
    }
    pub fn config() -> DomainConfig {
        DomainConfig {
            input_layer_size: Self::INPUT_LAYER_SIZE,
            output_layer_size: Self::OUTPUT_LAYER_SIZE,
            fitness: Box::new(|n| {
                let actual = NeatAgent::new(n).fitness();
                crate::neat::sigmoid(actual)
            }),
        }
    }
}
