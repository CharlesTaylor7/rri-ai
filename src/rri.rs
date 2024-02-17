use std::sync::Arc;

pub enum Piece {
    Road,
    Rail,
}

pub struct Route {
    pub face: DieFace,
    pub north: Option<Piece>,
    pub east: Option<Piece>,
    pub south: Option<Piece>,
    pub west: Option<Piece>,
    pub station: bool,
}

#[derive(Clone)]
pub struct Die {
    faces: Arc<[DieFace]>,
}

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
