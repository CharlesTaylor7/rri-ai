import type { NextApiRequest, NextApiResponse } from 'next'
import type { RouteInfo } from '@/types'
import { getServerState, drawInFirstValidPosition } from '@/server/state'
import { dice, roll } from '@/server/dice'
import db from '@/server/db'

type Data = {
  diceCodes: Array<number>
  nextRoutes: Array<RouteInfo>
}

export default async function handler(
  req: NextApiRequest,
  res: NextApiResponse<Data | string>,
) {
  const gameId = req.query.gameId
  if (gameId === undefined) {
    res.status(404).send('Game not found')
    return
  }
  const gameState = await getServerState(gameId)
  if (gameState === undefined) {
    res.status(404).send('Game not found')
    return
  }

  // get results
  const diceCodes = dice.map((die) => roll(die))
  const nextRoutes = diceCodes
    .map((code) => drawInFirstValidPosition(gameState, code))
    .filter((route) => route) as Array<RouteInfo>

  await db('games').where('uuid', gameId).update('server_json', gameState)

  res.status(200).json({
    diceCodes,
    nextRoutes,
  })
}
