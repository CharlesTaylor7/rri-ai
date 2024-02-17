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
    pub pattern: DiePattern,
    pub tile: Tile,
}

pub struct GameState {
    pub drawn_routes: Vec<RouteInfo>,
    pub open_edges: HashMap<TileEdge, Piece>,
    pub dice_roll: Rc<[DieFace]>,
}

pub enum EditAction {
    Add(Piece),
    Delete,
}

pub struct Edit {
    edge: TileEdge,
    action: EditAction,
}

impl GameState {
    pub fn apply_edit(&mut self, edit: &Edit) -> Result<()> {
        todo!()
    }
    pub fn apply_route(&mut self, action: &DrawAction) -> Result<()> {
        // dry run of edits
        let mut edits: Vec<Edit> = Vec::with_capacity(10);
        for direction in Direction::ALL {
            let edge = TileEdge {
                tile: action.0.tile,
                direction,
            };
            let piece = self.open_edges[edge];
        }

        todo!()
        /*
        export function drawRoute(gameState: GameState, routeInfo: RouteInfo) {

            const connection = toConnectionKey({ x, y, direction })
            const networkPiece = gameState.openRoutes[connection]

            if (networkPiece === undefined) {
              // add the opposite connection to the map
              const connection = oppositeConnection({ x, y, direction })
              if (gameState.openRoutes[connection]) {
                throw new RouteValidationError('cannot draw over another route')
              }
              const piece: Piece = route[direction] as Piece
              edits.push({ connection, action: { type: 'add', piece } })
            } else if (networkPiece === route[direction]) {
              // clear the current connection from the map
              edits.push({ connection, action: { type: 'delete' } })
            } else {
              throw new RouteValidationError(
                'cannot connect railway directly to highway',
              )
            }
          }

          if (!edits.some((e: Edit) => e.action.type === 'delete')) {
            throw new RouteValidationError(
              "piece doesn't connect to any existing network",
            )
          }

          if (gameState.routesDrawn.some((r) => r.x === x && r.y === y)) {
            throw new RouteValidationError('route cannot draw over existing route')
          }

          // commit the edits to state
          gameState.routesDrawn.push(routeInfo)
          for (let edit of edits) {
            if (edit.action.type === 'delete') {
              delete gameState.openRoutes[edit.connection]
            } else if (edit.action.type === 'add') {
              gameState.openRoutes[edit.connection] = edit.action.piece
            }
          }
        }

        */
    }

    pub fn apply_turn(&mut self, turn: &Turn) -> Result<()> {
        //self

        todo!()
    }
}

pub struct Turn {
    actions: Rc<[DrawAction]>,
}

pub struct DrawAction(pub RouteInfo);

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

#[derive(PartialEq, Eq, Hash)]
pub struct Tile {
    pub x: u8,
    pub y: u8,
}

#[derive(PartialEq, Eq, Hash)]
pub struct TileEdge {
    pub tile: Tile,
    pub direction: Direction,
}

pub trait Agent {
    fn act(&self, state: &GameState) -> Turn;
}
