export type AppState = {
  gameId: string
  diceCodes: Array<number>
  currentRoutes: Array<RouteInfo>
  pendingRoutes: Array<RouteInfo>
}

export type RouteInfo = {
  // in the order printed at the top of the player boards:
  // 0-8: the normal die faces
  // 9-14: the special routes
  code: number
  // 0-3: the number of clockwise rotations applied to the route
  // (default orientation is as shown on the player board)
  rotation: number

  // grid coordinates, 0 to 6 inclusive
  x: number
  y: number
}

// highway, railway,
export type Piece = 'h' | 'r'
// river or lake
//| 'v' | 'l';

export interface Route {
  north?: Piece
  east?: Piece
  south?: Piece
  west?: Piece
  station?: boolean
}
