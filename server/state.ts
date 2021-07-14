import type { NextApiResponse } from 'next'
import { v4 as uuid } from 'uuid'

type GameId = string

export const state = {}
export function newGame(): GameId {
    const gameId = uuid()
    state[gameId] = { routesDrawn: [] }
    return gameId
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
