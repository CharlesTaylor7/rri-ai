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
    const gameState = state
    if (gameState === undefined) {
        res.status(404).send('Game not Found')
    }
    else {
        res.status(200).json(gameState)
    }
}
