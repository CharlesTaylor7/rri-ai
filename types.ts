export type Route = {
    // in the order printed at the top of the player boards:
    // 0-8: the normal die faces
    // 9-14: the special routes
    code: number,
    // 0-3: the number of clockwise rotations applied to the route
    // (default orientation is as shown on the player board)
    rotation: number,
}
