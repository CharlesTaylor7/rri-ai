import type { NextApiResponse } from 'next'
import type { RouteInfo } from '@/types'
import { v4 as uuid } from 'uuid'

type GameId = string

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


const oppositeDir = (direction) => {
    switch (direction) {
        case 'north': return 'south'
        case 'east': return 'west'
        case 'south': return 'north'
        case 'west': return 'east'
        default: throw new Error('invalid direction')
    }
}

const toShift = (direction) => {
    switch (direction) {
        case 'north': return {x:0, y:-1}
        case 'east': return {x:1, y:0}
        case 'south': return {x:0,y:1}
        case 'west': return {x:-1,y:0}
        default: throw new Error('invalid direction')
    }
}

const oppositeConnection = ({x, y, direction}) => {
    const {x: dx,y: dy } = toShift(direction)
    return ({ x: x+dx, y: y+dy, direction: oppositeDir(direction)})
}

export function drawRoute(gameState: object, routeInfo: RouteInfo) {
    // update game state
    gameState.routesDrawn.push(routeInfo)
    const route = rotate(routes[routeInfo.code], routeInfo.rotate)
    const { x, y } = routeInfo
    for (let direction of ['north', 'east', 'south', 'west']) {
        const connection = `${x}-${y}-${direction[0]}`
        const networkPiece = gameState.openRoutes[connection]

        console.log('existing piece', networkPiece)
        console.log('new piece', route[direction])

        if (networkPiece !== route[direction]) {
            throw new Error('cannot connect railway directly to highway')
        }
        if (networkPiece === undefined) {
            // add the opposite connection to the map
            const { x, y, direction } = oppositeConnection({x, y, direction})
            gameState.openRoutes[`${x}-${y}-${direction[0]}`] = route[direction]
        } else {
            // clear the current connection from the map
            delete gameState.openRoutes[connection]
        }
    }
}

class EventSource {
    constructor() {
        this.handles = {}
    }

    newHandle(id: GameId, res: NextApiResponse) {
        res.writeHead(200, {
              Connection: 'keep-alive',
              'Content-Type': 'text/event-stream',
        })
        if (this.handles[id] === undefined) {
            this.handles[id] = []
        }
        this.handles[id].push(res)
    }

    send(id: GameId, event) {
        const message = JSON.stringify(event) + '\n'
        for (let handle of this.handles[id]) {
            handle.write(message)
            handle.flush()
        }
    }

    close(id: GameId) {
        for (let handle of this.handles[id]) {
            handle.end()
        }
        delete this.handles[id]
    }
}

//export const eventSource = new EventSource()
