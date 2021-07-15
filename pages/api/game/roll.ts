import type { NextApiRequest, NextApiResponse } from 'next'
import { dice, roll, routes, rotate } from '@/server/dice'
import { state, drawRoute } from '@/server/state'

type Data = {
    routeCodes: Array<number>
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
    const nextRoutes = diceCodes.map((code, i) => ({code, rotate: i, x: i, y: 0}))
    for (let route of nextRoutes) {
        drawRoute(gameState, route)
    }

    res.status(200).json({
        diceCodes,
        nextRoutes,
    })
}
