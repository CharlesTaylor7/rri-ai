import type { NextApiResponse } from 'next'
import type { Piece, Route, RouteInfo } from 'app/types'
import { v4 as uuid } from 'uuid'
import { routes } from 'app/server/dice'

type GameId = string

export interface GameState {
  routesDrawn: Array<RouteInfo>
  openRoutes: OpenRoutes
}

type ServerState = GameState

export interface Shift {
  x: number
  y: number
}

export type Direction = 'north' | 'east' | 'south' | 'west'
export interface Position {
  x: number
  y: number
}

export interface Location extends Position {
  direction: Direction
}

let state: ServerState | undefined = undefined

export function getServerState(): ServerState | undefined {
  return state
}

export function newGame(): void {
  state = getInitialState()
}

// key is hyphen separated values:
// x-y-direction (direction is one of 'n', 'e', 's', 'w'
// value is the type of network piece: 'h' or 'r'
type OpenRoutes = {
  [connection: string]: Piece
}

function getInitialState(): GameState {
  return {
    routesDrawn: [],
    openRoutes: {
      // north exits
      '1-0-n': 'h',
      '3-0-n': 'r',
      '5-0-n': 'h',
      // east exits
      '6-1-e': 'r',
      '6-3-e': 'h',
      '6-5-e': 'r',
      // south exits
      '1-6-s': 'h',
      '3-6-s': 'r',
      '5-6-s': 'h',
      // west exits
      '0-1-w': 'r',
      '0-3-w': 'h',
      '0-5-w': 'r',
    },
  }
}

const rotate = (route: Route, i: number): Route => {
  // copy the route
  route = { ...route }
  while (i-- > 0) {
    const west = route.west
    route.west = route.south
    route.south = route.east
    route.east = route.north
    route.north = west
  }
  return route
}

const oppositeDir = (direction: Direction): Direction => {
  switch (direction) {
    case 'north':
      return 'south'
    case 'east':
      return 'west'
    case 'south':
      return 'north'
    case 'west':
      return 'east'
    default:
      throw new Error('invalid direction')
  }
}

const toShift = (direction: Direction): Shift => {
  switch (direction) {
    case 'north':
      return { x: 0, y: -1 }
    case 'east':
      return { x: 1, y: 0 }
    case 'south':
      return { x: 0, y: 1 }
    case 'west':
      return { x: -1, y: 0 }
    default:
      throw new Error('invalid direction')
  }
}

function toConnectionKey({ x, y, direction }: Location) {
  return `${x}-${y}-${direction[0]}`
}

const oppositeConnection = ({ x, y, direction }: Location) => {
  const { x: dx, y: dy } = toShift(direction)
  return toConnectionKey({
    x: x + dx,
    y: y + dy,
    direction: oppositeDir(direction),
  })
}

export class RouteValidationError extends Error {}

export interface Edit {
  connection: string
  action: { type: 'add'; piece: Piece } | { type: 'delete' }
}

const directions: Array<Direction> = ['north', 'east', 'south', 'west']

export function drawRoute(gameState: GameState, routeInfo: RouteInfo) {
  const { x, y } = routeInfo
  console.log(routeInfo)
  const route = rotate(routes[routeInfo.code], routeInfo.rotation)

  // first we do a dry run of the edits to state for the given route
  // validate and then apply the state changes
  const edits: Array<Edit> = []
  for (let direction of directions) {
    const connection = toConnectionKey({ x, y, direction })
    const networkPiece = gameState.openRoutes[connection]

    console.log('existing piece', networkPiece)
    console.log('new piece', route[direction])

    if (networkPiece !== route[direction]) {
      throw new RouteValidationError(
        'cannot connect railway directly to highway',
      )
    }
    if (networkPiece === undefined) {
      // add the opposite connection to the map
      const connection = oppositeConnection({ x, y, direction })
      if (gameState.openRoutes[connection]) {
        throw new RouteValidationError('cannot draw over another route')
      }
      const piece: Piece = route[direction] as Piece
      edits.push({ connection, action: { type: 'add', piece } })
    } else {
      // clear the current connection from the map
      edits.push({ connection, action: { type: 'delete' } })
    }
  }

  console.log('edits', edits)
  if (!edits.some((e: Edit) => e.action.type === 'delete')) {
    throw new RouteValidationError(
      "piece doesn't connect to any existing network",
    )
  }

  // update game state
  gameState.routesDrawn.push(routeInfo)
  for (let edit of edits) {
    if (edit.action.type === 'delete') {
      delete gameState.openRoutes[edit.connection]
    } else if (edit.action.type === 'add') {
      gameState.openRoutes[edit.connection] = edit.action.piece
    }
  }
}

export function drawInFirstValidPosition(
  gameState: GameState,
  code: number,
): RouteInfo | undefined {
  for (let x = 0; x < 7; x++) {
    for (let y = 0; y < 7; y++) {
      for (let rotation = 0; rotation < 4; rotation++) {
        const routeInfo = { code, rotation, x, y }
        try {
          drawRoute(gameState, routeInfo)
          return routeInfo
        } catch (e) {
          // continue if the exception type matches, otherwise reraise it
          if (!(e instanceof RouteValidationError)) throw e
        }
      }
    }
  }
}
