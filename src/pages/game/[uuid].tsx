import Grid from '@/components/game/Grid'
import Dice from '@/components/game/Dice'
import DiceButton from '@/components/game/DiceButton'
import debugData from '@/debugData'
import { Provider } from '@/context'
import { AppState } from '@/types'
import useErgonomicState from '@/hooks/useErgonomicState'
import { SSR } from '@/core/types'
import db from '@/server/db'
import { GetServerSideProps } from 'next'

type Props = AppState

export default function Game(props: Props) {
  return (
    <Provider value={useErgonomicState(props)}>
      <div className="h-full overflow-y-scroll flex flex-wrap justify-around">
        <Grid />
        <div className="flex flex-col">
          <DiceButton />
          <Dice />
        </div>
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
    .select('client_json')
    .from('games')
    .where('uuid', gameId)
    .limit(1)
    .then((rows) => rows[0])

  if (game === undefined) {
    await db('games').insert({ uuid: gameId })
  }

  return {
    props: {
      routes: {
        current: debugData,
        pending: [],
      },
      diceCodes: [],
      ...(game?.client_json || {}),
    },
  }
}
