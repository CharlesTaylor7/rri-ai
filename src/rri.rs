use std::sync::Arc;

pub enum Piece {
    Road,
    Rail,
}

pub struct Route {
    face: DieFace,
    north: Option<Piece>,
    east: Option<Piece>,
    south: Option<Piece>,
    west: Option<Piece>,
    station: bool,
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

const ROUTES: [Route; 34] = [
    Route {
        face: DieFace::AngleRail,
        north: Some(Piece::Rail),
        east: None,
        south: None,
        west: Some(Piece::Rail),
        station: false,
    },
    Route {
        face: DieFace::AngleRail,
        north: Some(Piece::Rail),
        east: Some(Piece::Rail),
        south: None,
        west: None,
        station: false,
    },
    Route {
        face: DieFace::AngleRail,
        north: None,
        east: Some(Piece::Rail),
        south: Some(Piece::Rail),
        west: None,
        station: false,
    },
    Route {
        face: DieFace::AngleRail,
        north: None,
        east: None,
        south: Some(Piece::Rail),
        west: Some(Piece::Rail),
        station: false,
    },
    Route {
        face: DieFace::ThreeRail,
        north: Some(Piece::Rail),
        east: Some(Piece::Rail),
        south: None,
        west: Some(Piece::Rail),
        station: false,
    },
    Route {
        face: DieFace::ThreeRail,
        north: Some(Piece::Rail),
        east: Some(Piece::Rail),
        south: Some(Piece::Rail),
        west: None,
        station: false,
    },
    Route {
        face: DieFace::ThreeRail,
        north: None,
        east: Some(Piece::Rail),
        south: Some(Piece::Rail),
        west: Some(Piece::Rail),
        station: false,
    },
    Route {
        face: DieFace::ThreeRail,
        north: Some(Piece::Rail),
        east: None,
        south: Some(Piece::Rail),
        west: Some(Piece::Rail),
        station: false,
    },
    Route {
        face: DieFace::StraightRail,
        north: Some(Piece::Rail),
        east: None,
        south: Some(Piece::Rail),
        west: None,
        station: false,
    },
    Route {
        face: DieFace::StraightRail,
        north: None,
        east: Some(Piece::Rail),
        south: None,
        west: Some(Piece::Rail),
        station: false,
    },
    // angle road
    Route {
        face: DieFace::AngleRoad,
        north: Some(Piece::Road),
        east: None,
        south: None,
        west: Some(Piece::Road),
        station: false,
    },
    Route {
        face: DieFace::AngleRoad,
        north: Some(Piece::Road),
        east: Some(Piece::Road),
        south: None,
        west: None,
        station: false,
    },
    Route {
        face: DieFace::AngleRoad,
        north: None,
        east: Some(Piece::Road),
        south: Some(Piece::Road),
        west: None,
        station: false,
    },
    Route {
        face: DieFace::AngleRoad,
        north: None,
        east: None,
        south: Some(Piece::Road),
        west: Some(Piece::Road),
        station: false,
    },
    // 3 road
    Route {
        face: DieFace::ThreeRoad,
        north: Some(Piece::Road),
        east: Some(Piece::Road),
        south: None,
        west: Some(Piece::Road),
        station: false,
    },
    Route {
        face: DieFace::ThreeRoad,
        north: Some(Piece::Road),
        east: Some(Piece::Road),
        south: Some(Piece::Road),
        west: None,
        station: false,
    },
    Route {
        face: DieFace::ThreeRoad,
        north: None,
        east: Some(Piece::Road),
        south: Some(Piece::Road),
        west: Some(Piece::Road),
        station: false,
    },
    Route {
        face: DieFace::ThreeRoad,
        north: Some(Piece::Road),
        east: None,
        south: Some(Piece::Road),
        west: Some(Piece::Road),
        station: false,
    },
    // straight road
    Route {
        face: DieFace::StraightRoad,
        north: Some(Piece::Road),
        east: None,
        south: Some(Piece::Road),
        west: None,
        station: false,
    },
    Route {
        face: DieFace::StraightRoad,
        north: None,
        east: Some(Piece::Road),
        south: None,
        west: Some(Piece::Road),
        station: false,
    },
    // overpass
    Route {
        face: DieFace::Overpass,
        north: Some(Piece::Road),
        east: Some(Piece::Rail),
        south: Some(Piece::Road),
        west: Some(Piece::Rail),
        station: false,
    },
    Route {
        face: DieFace::Overpass,
        north: Some(Piece::Rail),
        east: Some(Piece::Road),
        south: Some(Piece::Rail),
        west: Some(Piece::Road),
        station: false,
    },
    // straight station
    Route {
        face: DieFace::StraightStation,
        north: Some(Piece::Rail),
        east: None,
        south: Some(Piece::Road),
        west: None,
        station: true,
    },
    Route {
        face: DieFace::StraightStation,
        north: None,
        east: Some(Piece::Rail),
        south: None,
        west: Some(Piece::Road),
        station: true,
    },
    Route {
        face: DieFace::StraightStation,
        north: Some(Piece::Road),
        east: None,
        south: Some(Piece::Rail),
        west: None,
        station: true,
    },
    Route {
        face: DieFace::StraightStation,
        north: None,
        east: Some(Piece::Road),
        south: None,
        west: Some(Piece::Rail),
        station: true,
    },
    // angle station
    Route {
        face: DieFace::AngleStation,
        north: Some(Piece::Rail),
        east: None,
        south: None,
        west: Some(Piece::Road),
        station: true,
    },
    Route {
        face: DieFace::AngleStation,
        north: Some(Piece::Road),
        east: Some(Piece::Rail),
        south: None,
        west: None,
        station: true,
    },
    Route {
        face: DieFace::AngleStation,
        north: None,
        east: Some(Piece::Road),
        south: Some(Piece::Rail),
        west: None,
        station: true,
    },
    Route {
        face: DieFace::AngleStation,
        north: None,
        east: None,
        south: Some(Piece::Road),
        west: Some(Piece::Rail),
        station: true,
    },
    // mirrored angle station
    Route {
        face: DieFace::AngleStation,
        north: Some(Piece::Road),
        east: None,
        south: None,
        west: Some(Piece::Rail),
        station: true,
    },
    Route {
        face: DieFace::AngleStation,
        north: Some(Piece::Rail),
        east: Some(Piece::Road),
        south: None,
        west: None,
        station: true,
    },
    Route {
        face: DieFace::AngleStation,
        north: None,
        east: Some(Piece::Rail),
        south: Some(Piece::Road),
        west: None,
        station: true,
    },
    Route {
        face: DieFace::AngleStation,
        north: None,
        east: None,
        south: Some(Piece::Rail),
        west: Some(Piece::Road),
        station: true,
    },
];
