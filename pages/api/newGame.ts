import type { NextApiRequest, NextApiResponse } from 'next'
import type { RouteInfo } from '@/types'
import { newGame } from '@/server/state'

type Data = {
    gameId: string,
}

export default function handler(
    req: NextApiRequest,
    res: NextApiResponse<Data>
) {
    res.status(200).json({gameId: newGame()})
}
