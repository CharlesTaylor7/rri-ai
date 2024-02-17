pub enum Piece {
    Highway,
    Railway,
}

pub struct Route {
    north: Option<Piece>,
    east: Option<Piece>,
    south: Option<Piece>,
    west: Option<Piece>,
    station: bool,
}
