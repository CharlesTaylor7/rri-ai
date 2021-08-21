import type { NextApiResponse } from 'next'
import type { Piece, Route, RouteInfo } from 'rri-ai/types'
import { v4 as uuid } from 'uuid'
import { routes } from 'rri-ai/server/dice'

type GameId = string;
type state = {
    [key: GameId]: GameState
}

export const state = {}
export function newGame(): GameId {
    const gameId = uuid()
    state[gameId] = getInitialState()
    return gameId
}

// key is hyphen separated values:
// x-y-direction (direction is one of 'n', 'e', 's', 'w'
// value is the type of network piece: 'h' or 'r'
type OpenRoutes = object

function getInitialState() {
    return ({
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
        }
    })
}

const rotate = (route: Route, i: number): Route => {
    // copy the route
    route = {...route}
    while (i-->0) {
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
        case 'north': return 'south'
        case 'east': return 'west'
        case 'south': return 'north'
        case 'west': return 'east'
        default: throw new Error('invalid direction')
    }
}

export interface Shift {
    x: number;
    y: number;
}

const toShift = (direction: Direction): Shift => {
    switch (direction) {
        case 'north': return {x:  0, y: -1}
        case 'east':  return {x:  1, y:  0}
        case 'south': return {x:  0, y:  1}
        case 'west':  return {x: -1, y:  0}
        default: throw new Error('invalid direction')
    }
}

export type Direction = 'north' | 'east' | 'south' | 'west';

export interface Location {
    x: number,
    y: number,
    direction: Direction,
}
function toConnectionKey({ x, y, direction }) {
    return `${x}-${y}-${direction[0]}`
}

const oppositeConnection = ({x, y, direction}) => {
    const {x: dx,y: dy } = toShift(direction)
    return toConnectionKey({ x: x+dx, y: y+dy, direction: oppositeDir(direction)})
}

export class RouteValidationError extends Error { }

export interface Edit {
    connection: string,
    piece: 'r' | 'h',
    delete: boolean,
}

export function drawRoute(gameState: object, routeInfo: RouteInfo) {
    const { x, y } = routeInfo
    console.log(routeInfo)
    const route = rotate(routes[routeInfo.code], routeInfo.rotation)

    // first we do a dry run of the edits to state for the given route
    // validate and then apply the state changes
    const edits: Array<Edit> = []
    for (let direction of ['north', 'east', 'south', 'west']) {
        const connection = toConnectionKey({x, y, direction})
        const networkPiece = gameState.openRoutes[connection]

        console.log('existing piece', networkPiece)
        console.log('new piece', route[direction])

        if (networkPiece !== route[direction]) {
            throw new RouteValidationError('cannot connect railway directly to highway')
        }
        if (networkPiece === undefined) {
            // add the opposite connection to the map
            const connection = oppositeConnection({x, y, direction})
            if (gameState.openRoutes[connection]) {
                throw new RouteValidationError('cannot draw over another route')
            }
            edits.push({ connection, piece: route[direction] })
        } else {
            // clear the current connection from the map
            edits.push({ connection, delete: true })
        }
    }

    console.log('edits', edits)
    if (!edits.any(e => e.delete)) {
        throw new RouteValidationError('piece doesn\'t connect to any existing network')
    }

    // update game state
    gameState.routesDrawn.push(routeInfo)
    for (let edit of edits) {
        if (edit.delete) {
            delete gameState.openRoutes[edit.connection]
        } else {
            gameState.openRoutes[edit.connection] = edit.piece
        }
    }
}

export function drawInFirstValidPosition(gameState: object, code: number): RouteInfo | undefined {
    for (let x = 0; x < 7; x++) {
        for (let y = 0; y < 7; y++) {
            for (let rotation = 0; rotation < 4; rotation++) {
                const routeInfo = {code, rotation, x, y}
                try {
                    drawRoute(gameState, routeInfo)
                    return routeInfo;
                } catch(e) {
                    // continue if the exception type matches, otherwise reraise it
                    if (!(e instanceof RouteValidationError)) throw e
                }
            }
        }
    }
}
