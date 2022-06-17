import Grid from '@/components/game/Grid'
import DicePanel from '@/components/game/DicePanel'
import { Provider } from '@/context'
import { AppState } from '@/types'
import useErgonomicState from '@/hooks/useErgonomicState'
import db from '@/server/db'
import { GetServerSideProps } from 'next'

type Props = AppState

export default function Game(props: Props) {
  return (
    <Provider value={useErgonomicState(props)}>
      <div className="h-full overflow-y-scroll font-mono flex flex-wrap justify-around items-start">
        <Grid />
        <DicePanel />
      </div>
    </Provider>
  )
}

const notFound = (context: any) => {
  const code = 404
  context.res.statusCode = code
  return {
    props: { error: { statusCode: code, title: 'Game not found' } },
  }
}

export const getServerSideProps: GetServerSideProps = async (context) => {
  const gameId = context.params?.uuid
  if (gameId === undefined) return notFound(context)
  let game = await db
    .select('json')
    .from('games')
    .where('uuid', gameId)
    .limit(1)
    .then((rows) => rows[0])

  if (game === undefined) {
    await db('games').insert({ uuid: gameId })
  }

  return {
    props: {
      gameId,
      round: game?.json.round || 1,
      currentRoutes: game?.json.routesDrawn || [],
      pendingRoutes: [],
      diceCodes: [],
    },
  }
}
