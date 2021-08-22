import { useCallback, useState, useEffect, useContext, useReducer } from 'react'
import Image from 'next/image'
import Grid from 'components/game/Grid'
import Dice from 'components/game/Dice'
import DiceButton from 'components/game/DiceButton'
import type { RouteInfo } from 'types'
import { state } from 'server/state'
import styles from 'styles/Game.module.css'
import type { NextPageContext } from 'next'


export default function Game() {
    return (
       <div className={styles.gameRow}>
            <Grid />
            <div className={styles.rightPanel}>
                <DiceButton />
                <Dice diceCodes={[0, 3, 5, 7]} />
            </div>
        </div>
    )
}


export async function getServerSideProps(context: NextPageContext) {
    const { params: { gameId } } = context as any
    const gameState = state[gameId]
    if (gameState === undefined) {
        const error = { statusCode: 404, title: 'Game does not exist' }
        return { props: { error } }
    }

    return ({
        props: {
            id: gameId,
            routes: {
                current: gameState.routesDrawn,
                pending: [],
            },
        },
    })
}
