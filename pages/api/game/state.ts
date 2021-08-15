import type { NextApiRequest, NextApiResponse } from 'next'
import type { RouteInfo } from 'rri-ai/types'
import { state } from 'rri-ai/server/state'

type Data = {
    routesDrawn: Array<RouteInfo>
}

export default function handler(
    req: NextApiRequest,
    res: NextApiResponse<Data>
) {
    const gameState = state[req.query.id]
    if (gameState === undefined) {
        res.status(404).json()
    }
    res.status(200).json(gameState)
}
