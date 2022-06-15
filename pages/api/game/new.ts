import type { NextApiRequest, NextApiResponse } from 'next'
import type { RouteInfo } from 'app/types'
import { newGame } from 'app/server/state'

type Data = {}

export default function handler(
  req: NextApiRequest,
  res: NextApiResponse<Data>,
) {
  newGame()
  res.status(200).json({})
}
