import type { NextApiRequest, NextApiResponse } from 'next'
import type { RouteInfo } from 'rri-ai/types'
import { newGame } from 'rri-ai/server/state'

type Data = {
    gameId: string,
}

export default function handler(
    req: NextApiRequest,
    res: NextApiResponse<Data>
) {
    const gameId = newGame()
    res.status(200).json({gameId})
}
