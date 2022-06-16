import Grid from 'app/components/game/Grid'
import Dice from 'app/components/game/Dice'
import DiceButton from 'app/components/game/DiceButton'
import debugData from 'app/debugData'
import { Provider } from 'app/context'
import { RouteInfo } from 'app/types'
import useErgonomicState from 'app/hooks/useErgonomicState'

type AppState = {
  routes: {
    current: Array<RouteInfo>
    pending: Array<RouteInfo>
  }
  diceCodes: Array<number>
}

type Props = {
  state: AppState
}

export default function Game(props: Props) {
  return (
    <Provider value={useErgonomicState(props.state)}>
      <div className="h-full overflow-y-scroll flex flex-wrap justify-around items-center">
        <Grid />
        <div>
          <DiceButton />
          <Dice />
        </div>
      </div>
    </Provider>
  )
}

export async function getServerSideProps() {
  return {
    props: {
      state: {
        game: {
          routes: {
            current: debugData,
            pending: [],
          },
          diceCodes: [],
        },
      },
    },
  }
}
