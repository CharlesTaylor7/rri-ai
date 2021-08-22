import type { NextApiRequest, NextApiResponse } from 'next'
import type { RouteInfo } from 'app/types'
import { newGame } from 'app/server/state'

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
