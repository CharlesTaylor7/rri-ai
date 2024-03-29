use crate::rri::{DieFace, DiePattern, Piece};

pub const DIE_PATTERNS: [DiePattern; 34] = [
    DiePattern {
        face: DieFace::AngleRail,
        north: Some(Piece::Rail),
        east: None,
        south: None,
        west: Some(Piece::Rail),
        station: false,
    },
    DiePattern {
        face: DieFace::AngleRail,
        north: Some(Piece::Rail),
        east: Some(Piece::Rail),
        south: None,
        west: None,
        station: false,
    },
    DiePattern {
        face: DieFace::AngleRail,
        north: None,
        east: Some(Piece::Rail),
        south: Some(Piece::Rail),
        west: None,
        station: false,
    },
    DiePattern {
        face: DieFace::AngleRail,
        north: None,
        east: None,
        south: Some(Piece::Rail),
        west: Some(Piece::Rail),
        station: false,
    },
    DiePattern {
        face: DieFace::ThreeRail,
        north: Some(Piece::Rail),
        east: Some(Piece::Rail),
        south: None,
        west: Some(Piece::Rail),
        station: false,
    },
    DiePattern {
        face: DieFace::ThreeRail,
        north: Some(Piece::Rail),
        east: Some(Piece::Rail),
        south: Some(Piece::Rail),
        west: None,
        station: false,
    },
    DiePattern {
        face: DieFace::ThreeRail,
        north: None,
        east: Some(Piece::Rail),
        south: Some(Piece::Rail),
        west: Some(Piece::Rail),
        station: false,
    },
    DiePattern {
        face: DieFace::ThreeRail,
        north: Some(Piece::Rail),
        east: None,
        south: Some(Piece::Rail),
        west: Some(Piece::Rail),
        station: false,
    },
    DiePattern {
        face: DieFace::StraightRail,
        north: Some(Piece::Rail),
        east: None,
        south: Some(Piece::Rail),
        west: None,
        station: false,
    },
    DiePattern {
        face: DieFace::StraightRail,
        north: None,
        east: Some(Piece::Rail),
        south: None,
        west: Some(Piece::Rail),
        station: false,
    },
    // angle road
    DiePattern {
        face: DieFace::AngleRoad,
        north: Some(Piece::Road),
        east: None,
        south: None,
        west: Some(Piece::Road),
        station: false,
    },
    DiePattern {
        face: DieFace::AngleRoad,
        north: Some(Piece::Road),
        east: Some(Piece::Road),
        south: None,
        west: None,
        station: false,
    },
    DiePattern {
        face: DieFace::AngleRoad,
        north: None,
        east: Some(Piece::Road),
        south: Some(Piece::Road),
        west: None,
        station: false,
    },
    DiePattern {
        face: DieFace::AngleRoad,
        north: None,
        east: None,
        south: Some(Piece::Road),
        west: Some(Piece::Road),
        station: false,
    },
    // 3 road
    DiePattern {
        face: DieFace::ThreeRoad,
        north: Some(Piece::Road),
        east: Some(Piece::Road),
        south: None,
        west: Some(Piece::Road),
        station: false,
    },
    DiePattern {
        face: DieFace::ThreeRoad,
        north: Some(Piece::Road),
        east: Some(Piece::Road),
        south: Some(Piece::Road),
        west: None,
        station: false,
    },
    DiePattern {
        face: DieFace::ThreeRoad,
        north: None,
        east: Some(Piece::Road),
        south: Some(Piece::Road),
        west: Some(Piece::Road),
        station: false,
    },
    DiePattern {
        face: DieFace::ThreeRoad,
        north: Some(Piece::Road),
        east: None,
        south: Some(Piece::Road),
        west: Some(Piece::Road),
        station: false,
    },
    // straight road
    DiePattern {
        face: DieFace::StraightRoad,
        north: Some(Piece::Road),
        east: None,
        south: Some(Piece::Road),
        west: None,
        station: false,
    },
    DiePattern {
        face: DieFace::StraightRoad,
        north: None,
        east: Some(Piece::Road),
        south: None,
        west: Some(Piece::Road),
        station: false,
    },
    // overpass
    DiePattern {
        face: DieFace::Overpass,
        north: Some(Piece::Road),
        east: Some(Piece::Rail),
        south: Some(Piece::Road),
        west: Some(Piece::Rail),
        station: false,
    },
    DiePattern {
        face: DieFace::Overpass,
        north: Some(Piece::Rail),
        east: Some(Piece::Road),
        south: Some(Piece::Rail),
        west: Some(Piece::Road),
        station: false,
    },
    // straight station
    DiePattern {
        face: DieFace::StraightStation,
        north: Some(Piece::Rail),
        east: None,
        south: Some(Piece::Road),
        west: None,
        station: true,
    },
    DiePattern {
        face: DieFace::StraightStation,
        north: None,
        east: Some(Piece::Rail),
        south: None,
        west: Some(Piece::Road),
        station: true,
    },
    DiePattern {
        face: DieFace::StraightStation,
        north: Some(Piece::Road),
        east: None,
        south: Some(Piece::Rail),
        west: None,
        station: true,
    },
    DiePattern {
        face: DieFace::StraightStation,
        north: None,
        east: Some(Piece::Road),
        south: None,
        west: Some(Piece::Rail),
        station: true,
    },
    // angle station
    DiePattern {
        face: DieFace::AngleStation,
        north: Some(Piece::Rail),
        east: None,
        south: None,
        west: Some(Piece::Road),
        station: true,
    },
    DiePattern {
        face: DieFace::AngleStation,
        north: Some(Piece::Road),
        east: Some(Piece::Rail),
        south: None,
        west: None,
        station: true,
    },
    DiePattern {
        face: DieFace::AngleStation,
        north: None,
        east: Some(Piece::Road),
        south: Some(Piece::Rail),
        west: None,
        station: true,
    },
    DiePattern {
        face: DieFace::AngleStation,
        north: None,
        east: None,
        south: Some(Piece::Road),
        west: Some(Piece::Rail),
        station: true,
    },
    // mirrored angle station
    DiePattern {
        face: DieFace::AngleStation,
        north: Some(Piece::Road),
        east: None,
        south: None,
        west: Some(Piece::Rail),
        station: true,
    },
    DiePattern {
        face: DieFace::AngleStation,
        north: Some(Piece::Rail),
        east: Some(Piece::Road),
        south: None,
        west: None,
        station: true,
    },
    DiePattern {
        face: DieFace::AngleStation,
        north: None,
        east: Some(Piece::Rail),
        south: Some(Piece::Road),
        west: None,
        station: true,
    },
    DiePattern {
        face: DieFace::AngleStation,
        north: None,
        east: None,
        south: Some(Piece::Rail),
        west: Some(Piece::Road),
        station: true,
    },
];
