import {useState, useCallback} from "react"
import Grid from "components/game/Grid";
import Dice from "components/game/Dice";
import DiceButton from "components/game/DiceButton";
import styles from "styles/Game.module.css";
import debugData from "app/debugData";
import { Provider } from 'app/context'
import { GameState } from 'app/server/state'
import {RouteInfo} from "app/types";

type Props = {
  state: {
    game: {
      routes: {
        current: Array<RouteInfo>,
        pending: Array<RouteInfo>,
      }, 
      diceCodes: Array<number>
    }
  }
}

export default function Game(props: Props) {
  const [ state, setState ] = useState<GameState>(props.state)
  const pushState = useCallback((updates: Partial<GameState>) => setState(state => ({...state, ...updates})), [setState])

  return (
    <Provider value={{state, pushState}}>
    <div className={styles.gameRow}>
      <Grid />
      <div className={styles.rightPanel}>
        <DiceButton />
        <Dice />
      </div>
    </div>
    </Provider>
  );
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
  };
}
