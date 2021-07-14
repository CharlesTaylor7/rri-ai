import type { NextApiRequest, NextApiResponse } from 'next'
import type { RouteInfo } from '@/types'
import { eventSource } from '@/server/state'


export default function handler(
    req: NextApiRequest,
    res: NextApiResponse,
) {
    eventSource.newHandle(req.query.id, res)
}
