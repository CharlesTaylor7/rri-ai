import { useCallback, useState, useEffect, useContext, useReducer } from 'react'
import Page from '@/components/core/Page'
import Image from 'next/image'
import Grid from '@/components/game/Grid'
import Dice from '@/components/game/Dice'
import DiceButton from '@/components/game/DiceButton'
import { RouteInfo } from '@/types'
import { state } from '@/server/state'
import styles from '@/styles/Game.module.css'
import GameStore from '@/store/game.ts'


export default function Game(props) {
    return (
        <Page error={props.error} store={GameStore} initialState={props}>
           <div className={styles.gameRow}>
                <Grid />
                <div className={styles.rightPanel}>
                    <DiceButton />
                    <Dice />
                </div>
            </div>
        </Page>
    )
}
Game.defaultProps = {
    routes: {
        current: [],
        pending: [],
    }
}

type GameProps = {
    props: {
            routes: {
            current: Array<RouteInfo>,
            pending: Array<RouteInfo>,
        },
    }
}

export async function getServerSideProps(context) {
    const { params: { gameId } } = context
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
