use anyhow::Result;
use rand::seq::SliceRandom;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;

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

#[derive(Clone)]
pub struct Die {
    pub faces: Arc<[DieFace]>,
}

impl Die {
    pub fn roll(&self) -> &DieFace {
        self.faces
            .choose(&mut rand::thread_rng())
            .expect("die should have at least one face")
    }
}

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum DieFace {
    AngleRail,
    ThreeRail,
    StraightRail,

    AngleRoad,
    ThreeRoad,
    StraightRoad,

    Overpass,
    StraightStation,
    AngleStation,
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

pub fn dice() -> Arc<[Die]> {
    let regular = Die {
        faces: Arc::new([
            DieFace::AngleRail,
            DieFace::ThreeRail,
            DieFace::StraightRail,
            DieFace::AngleRoad,
            DieFace::ThreeRoad,
            DieFace::StraightRoad,
        ]),
    };
    let special = Die {
        faces: Arc::new([
            DieFace::Overpass,
            DieFace::StraightStation,
            DieFace::AngleStation,
        ]),
    };
    Arc::new([regular.clone(), regular.clone(), regular, special])
}

pub struct RouteInfo {
    pattern: DiePattern,
    tile: Tile,
}

pub struct GameState {
    pub drawn_routes: Vec<RouteInfo>,
    pub open_edges: HashMap<TileEdge, Piece>,
    pub dice_roll: Rc<[DieFace]>,
}

impl GameState {
    pub fn apply(&mut self, turn: &Turn) -> Result<()> {
        todo!()
    }
}

pub struct Turn {
    actions: Rc<[DrawAction]>,
}

pub struct DrawAction {
    pattern: DiePattern,
    tile: Tile,
}

pub struct Shift {
    x: usize,
    y: usize,
}

pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Tile {
    pub x: u8,
    pub y: u8,
}

pub struct TileEdge {
    pub tile: Tile,
    pub direction: Direction,
}

pub trait Agent {
    fn act(&self, state: &GameState) -> Turn;
}
