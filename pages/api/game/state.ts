import type { NextApiRequest, NextApiResponse } from 'next'
import type { RouteInfo } from 'app/types'
import { state } from 'app/server/state'

type Data = {
    routesDrawn: Array<RouteInfo>
}

export default function handler(
    req: NextApiRequest,
    res: NextApiResponse<Data | string>
) {
    console.log("req.query.id", req.query.id)
    const gameState = state[req.query.id as string]
    if (gameState === undefined) {
        res.status(404).send('Game not Found')
    }
    else {
        res.status(200).json(gameState)
    }
}
