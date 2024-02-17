pub struct AppState {
    gameId: String,
    round: usize,
    diceCodes: Vec<u8>,
    currentRoutes: Vec<RouteInfo>,
    pendingRoutes: Vec<RouteInfo>,
}

pub struct RouteInfo {
    // in the order printed at the top of the player boards:
    // 0-8: the normal die faces
    // 9-14: the special routes
    code: u8,

    // 0-3: the number of clockwise rotations applied to the route
    // (default orientation is as shown on the player board)
    rotation: u8,

    // grid coordinates, 0 to 6 inclusive
    x: u8,
    y: u8,
}
