import type { NextApiRequest, NextApiResponse } from 'next'
import type { RouteInfo } from '@/types'
import { state, drawInFirstValidPosition} from '@/server/state'
import { dice, roll } from '@/server/dice'

type Data = {
    diceCodes: Array<number>,
    nextRoutes: Array<RouteInfo>
}


export default function handler(
    req: NextApiRequest,
    res: NextApiResponse<Data>
) {
    const gameState = state[req.query.id]
    if (gameState === undefined) {
        res.status(404).json()
        return
    }

    // get results
    const diceCodes = dice.map(die => roll(die))
    const nextRoutes = diceCodes
        .map(code => drawInFirstValidPosition(gameState, code))
        .filter(route => route)

    res.status(200).json({
        diceCodes,
        nextRoutes,
    })
}
