import Grid from "components/game/Grid";
import Dice from "components/game/Dice";
import DiceButton from "components/game/DiceButton";
import { getServerState } from "server/state";
import styles from "styles/Game.module.css";
import type { NextPageContext } from "next";
import debugData from "app/debugData";

export default function Game() {
  return (
    <div className={styles.gameRow}>
      <Grid />
      <div className={styles.rightPanel}>
        <DiceButton />
        <Dice />
      </div>
    </div>
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
