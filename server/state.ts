import { v4 as uuid } from 'uuid'

type GameId = string

export const state = {}
export function newGame(): GameId {
    const gameId = uuid()
    state[gameId] = { routesDrawn: [] }
    console.log('newGame', state)
    return gameId
}
