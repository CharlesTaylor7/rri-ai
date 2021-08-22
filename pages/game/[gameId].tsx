import { useCallback, useState, useEffect, useContext, useReducer } from 'react'
import Image from 'next/image'
import Grid from 'rri-ai/components/game/Grid'
import Dice from 'rri-ai/components/game/Dice'
import DiceButton from 'rri-ai/components/game/DiceButton'
import { RouteInfo } from 'rri-ai/types'
import { state } from 'rri-ai/server/state'
import styles from 'rri-ai/styles/Game.module.css'
import { initStore } from 'rri-ai/store/game'
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
