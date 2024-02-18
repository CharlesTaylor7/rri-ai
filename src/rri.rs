use anyhow::{bail, Result};
use num_traits::CheckedSub;
use rand::seq::SliceRandom;
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
    pub pattern: DiePattern,
    pub tile: Tile,
}

pub struct GameState {
    pub drawn_routes: Vec<RouteInfo>,
    pub open_edges: HashMap<TileEdge, Piece>,
    pub dice_roll: Rc<[DieFace]>,
}

pub enum Edit {
    Add(TileEdge, Piece),
    Delete(TileEdge),
}

impl GameState {
    pub fn new() -> GameState {
        Self {
            dice_roll: Rc::new([]),
            drawn_routes: Vec::with_capacity(49),
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
        // dry run of edits
        let DrawAction(RouteInfo { pattern, tile }) = action;
        let mut edits: Vec<Edit> = Vec::with_capacity(4);
        for direction in Direction::ALL {
            let edge = TileEdge { tile, direction };
            match self.open_edges.get(&edge) {
                None => {
                    let opposite = edge.adjacent();
                    if opposite.is_some_and(|opposite| self.open_edges.get(&opposite).is_some()) {
                        bail!("Cannot draw over another route")
                    }
                    if let Some(piece) = pattern.get(direction) {
                        edits.push(Edit::Add(edge, piece));
                    }
                }
                Some(network_piece) => {
                    if pattern
                        .get(direction)
                        .is_some_and(|piece| *network_piece == piece)
                    {
                        edits.push(Edit::Delete(edge));
                    } else {
                        bail!("Cannot connect railway directly to roadway")
                    }
                }
            }
        }
        if !edits
            .iter()
            .any(|e| if let Edit::Delete(_) = e { true } else { false })
        {
            bail!("Route doesn't connect to any road or rail in your network.")
        }

        if self.drawn_routes.iter().any(|route| route.tile == tile) {
            bail!("Cannot draw over existing route")
        }

        // commit edits to state
        for edit in edits {
            self.apply_edit(edit);
        }
        self.drawn_routes.push(RouteInfo { pattern, tile });
        Ok(())
    }

    pub fn apply_turn(&mut self, turn: Turn) -> Result<()> {
        for route in turn.actions {
            self.apply_route(route)?;
        }
        Ok(())
    }
}

pub struct Turn {
    actions: Vec<DrawAction>,
}

pub struct DrawAction(pub RouteInfo);

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

pub trait Agent {
    fn act(&self, state: &GameState) -> Turn;
}
