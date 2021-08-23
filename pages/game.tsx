import Grid from 'components/game/Grid'
import Dice from 'components/game/Dice'
import DiceButton from 'components/game/DiceButton'
import { getServerState } from 'server/state'
import styles from 'styles/Game.module.css'
import type { NextPageContext } from 'next'


export default function Game() {
    return (
       <div className={styles.gameRow}>
            <Grid />
            <div className={styles.rightPanel}>
                <DiceButton />
                <Dice />
            </div>
        </div>
    )
}


export async function getServerSideProps() {
    const gameState = getServerState();

    if (gameState === undefined) {
        const error = { statusCode: 404, title: 'Game does not exist' }
        return { props: { error } }
    }

    return ({
        props: {
            state: {
                game: {
                    routes: {
                        current: gameState.routesDrawn,
                        pending: [],
                    },
                    diceCodes: [],
                },
            },
        },
    })
}
