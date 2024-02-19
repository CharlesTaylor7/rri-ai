use anyhow::{bail, Result};
use num_traits::CheckedSub;
use rand::seq::SliceRandom;
use rand::Rng;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Piece {
    Road,
    Rail,
}

pub struct DiePattern {
    pub face: DieFace,
    // TODO: we probably need this metadata to make use of the client side svgs.
    // pub rotation: u8,
    // pub reflected: bool,
    pub north: Option<Piece>,
    pub east: Option<Piece>,
    pub south: Option<Piece>,
    pub west: Option<Piece>,
    pub station: bool,
}
impl DiePattern {
    pub fn get(&self, direction: Direction) -> Option<Piece> {
        match direction {
            Direction::North => self.north,
            Direction::East => self.east,
            Direction::South => self.south,
            Direction::West => self.west,
        }
    }
}

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum RegularDieFace {
    AngleRail = 0,
    ThreeRail = 1,
    StraightRail = 2,

    AngleRoad = 3,
    ThreeRoad = 4,
    StraightRoad = 5,
}

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum SpecialDieFace {
    Overpass = 6,
    StraightStation = 7,
    AngleStation = 8,
}

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum DieFace {
    AngleRail = 0,
    ThreeRail = 1,
    StraightRail = 2,

    AngleRoad = 3,
    ThreeRoad = 4,
    StraightRoad = 5,

    Overpass = 6,
    StraightStation = 7,
    AngleStation = 8,
}

impl DieFace {
    pub fn routes(&self) -> &'static [DiePattern] {
        let range = match self {
            Self::AngleRail => 0..4,
            Self::ThreeRail => 4..8,
            Self::StraightRail => 8..10,

            Self::AngleRoad => 10..14,
            Self::ThreeRoad => 14..18,
            Self::StraightRoad => 18..20,

            Self::Overpass => 20..22,
            Self::StraightStation => 22..26,
            Self::AngleStation => 26..34,
        };
        crate::routes::DIE_PATTERNS[range].borrow()
    }
}

pub struct DrawAction {
    pub pattern: &'static DiePattern,
    pub tile: Tile,
}

// TODO:
// speedup: Replace hashmaps with arrays of options.
// that would use less memory and spend less time hashing data.
pub struct GameState {
    pub drawn_routes: Vec<DrawAction>,
    pub open_edges: HashMap<TileEdge, Piece>,
    pub dice: Dice,
}

pub struct Dice {
    pub regular: [RegularDieFace; 3],
    pub special: SpecialDieFace,
}

impl Dice {
    pub fn roll(&mut self) {
        for i in 0..3 {
            self.regular[i] = {
                let src = rand::thread_rng().gen_range(0..6_u8);
                unsafe { std::mem::transmute(src) }
            }
        }

        self.special = {
            let src = rand::thread_rng().gen_range(6..9_u8);
            unsafe { std::mem::transmute(src) }
        }
    }
}

pub enum Edit {
    Add(TileEdge, Piece),
    Delete(TileEdge),
}

impl GameState {
    pub fn play<Agent: RRIAgent>(&mut self, agent: &mut Agent) {
        for _ in 0..7 {
            self.play_round(agent)
        }
    }
    pub fn play_round<Agent: RRIAgent>(&mut self, agent: &mut Agent) {
        self.dice.roll();
        let turn = agent.prompt(&self);

        for route in turn.actions {
            if let Err(error) = self.apply_route(route) {
                agent.handle_error(error)
            }
        }
    }

    pub fn score(&self) -> isize {
        //log::warn!("TODO: implement scoring");
        0
    }

    pub fn new() -> GameState {
        Self {
            dice: Dice {
                regular: [RegularDieFace::AngleRail; 3],
                special: SpecialDieFace::Overpass,
            },
            drawn_routes: Vec::with_capacity(28),
            open_edges: HashMap::from([
                // north exits
                (TileEdge::new(1, 0, Direction::North), Piece::Road),
                (TileEdge::new(3, 0, Direction::North), Piece::Rail),
                (TileEdge::new(5, 0, Direction::North), Piece::Road),
                // west exits
                (TileEdge::new(0, 1, Direction::East), Piece::Rail),
                (TileEdge::new(0, 3, Direction::East), Piece::Road),
                (TileEdge::new(0, 5, Direction::East), Piece::Rail),
                // south exits
                (TileEdge::new(1, 6, Direction::South), Piece::Road),
                (TileEdge::new(3, 6, Direction::South), Piece::Rail),
                (TileEdge::new(5, 6, Direction::South), Piece::Road),
                // east exits
                (TileEdge::new(6, 1, Direction::East), Piece::Rail),
                (TileEdge::new(6, 3, Direction::East), Piece::Road),
                (TileEdge::new(6, 5, Direction::East), Piece::Rail),
            ]),
        }
    }

    pub fn apply_edit(&mut self, edit: Edit) -> () {
        match edit {
            Edit::Add(edge, piece) => {
                //
                self.open_edges.insert(edge, piece);
            }
            Edit::Delete(edge) => {
                //
                self.open_edges.remove(&edge);
            }
        }
    }
    pub fn apply_route(&mut self, action: DrawAction) -> Result<()> {
        let DrawAction { pattern, tile } = action;

        if self.drawn_routes.iter().any(|action| action.tile == tile) {
            bail!("Cannot draw over existing route")
        }

        // dry run of edits
        let mut edits: Vec<Edit> = Vec::with_capacity(4);
        for direction in Direction::ALL {
            let edge = TileEdge { tile, direction };
            match (pattern.get(direction), self.open_edges.get(&edge)) {
                (Some(piece1), Some(piece2)) => {
                    if piece1 == *piece2 {
                        edits.push(Edit::Delete(edge));
                    } else {
                        bail!("Cannot connect railway directly to roadway")
                    }
                }
                (Some(piece), None) => {
                    if let Some(edge) = edge.adjacent() {
                        edits.push(Edit::Add(edge, piece));
                    }
                }
                (None, _) => {}
            }
        }
        if !edits
            .iter()
            .any(|e| if let Edit::Delete(_) = e { true } else { false })
        {
            bail!("Route doesn't connect to any road or rail in your network.")
        }

        // commit edits to state
        for edit in edits {
            self.apply_edit(edit);
        }
        self.drawn_routes.push(DrawAction { tile, pattern });
        Ok(())
    }
}

pub struct Turn {
    pub actions: Vec<DrawAction>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub const ALL: [Direction; 4] = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Tile {
    pub x: u8,
    pub y: u8,
}

#[derive(PartialEq, Eq, Hash)]
pub struct TileEdge {
    pub tile: Tile,
    pub direction: Direction,
}

impl TileEdge {
    pub fn new(x: u8, y: u8, direction: Direction) -> Self {
        TileEdge {
            tile: Tile { x, y },
            direction,
        }
    }
    pub fn adjacent(&self) -> Option<TileEdge> {
        let edge = match self.direction {
            Direction::North => {
                TileEdge::new(self.tile.x, self.tile.y.checked_sub(1)?, Direction::South)
            }
            Direction::West => {
                TileEdge::new(self.tile.x.checked_sub(1)?, self.tile.y, Direction::East)
            }
            Direction::South => {
                let y = if self.tile.y == 6 {
                    None?
                } else {
                    self.tile.y + 1
                };
                TileEdge::new(self.tile.x, y, Direction::North)
            }
            Direction::East => {
                let x = if self.tile.x == 6 {
                    None?
                } else {
                    self.tile.x + 1
                };
                TileEdge::new(x, self.tile.y, Direction::West)
            }
        };
        Some(edge)
    }
}

pub trait RRIAgent {
    fn prompt(&mut self, state: &GameState) -> Turn;
    fn handle_error(&mut self, error: anyhow::Error);
}
