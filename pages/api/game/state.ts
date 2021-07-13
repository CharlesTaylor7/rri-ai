import type { NextApiRequest, NextApiResponse } from 'next'
import type { RouteInfo } from '@/types'
import { state } from '@/server/state'

type Data = {
    routesDrawn: Array<RouteInfo>
}

export default function handler(
    req: NextApiRequest,
    res: NextApiResponse<Data>
) {
    //console.log(req)
    //console.log(state)
    // res.status(200).json(state[gameId])
    res.status(200).json({ routesDrawn: [] })
}
