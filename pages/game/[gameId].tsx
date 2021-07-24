import { useCallback, useState, useEffect } from 'react'
import Page from '@/components/Page'
import Image from 'next/image'
import Grid from '@/components/Grid'
import Dice from '@/components/Dice'
import { RouteInfo } from '@/types'
import { state } from '@/server/state'
import styles from '@/styles/Game.module.css'


export default function Game(props) {
    const [routes, setRoutes] = useState(props.routes)
    const [diceCodes, setDiceCodes] = useState([])

    const rollCallback = useCallback(async () => {
        const url = `/api/game/roll/?id=${props.id}`
        const { diceCodes, nextRoutes } = await fetch(url).then(res => res.json())
        setDiceCodes(diceCodes)
        setRoutes(rs => ({
            pending: nextRoutes,
            current: rs.current,
        }))
    })

    const showMoveCallback = useCallback(() => {
        setRoutes(routes => ({
            current: [...routes.current, ...routes.pending],
            pending: []
        }))
    })

    const onClick = routes.pending.length > 0 
        ? showMoveCallback 
        : rollCallback

    return (
        <Page error={props.error}>
           <div className={styles.gameRow}>
                <div className={styles.gridColumn}>
                    <Grid routes={routes.current} />
                </div>
                <div className={styles.rightPanel}>
                    <button
                        className={styles.diceButton}
                        onClick={onClick}
                    >   
                       {routes.pending.length > 0 ? 'Show Move' : 'Roll!!!!'}
                    </button>
                    <Dice diceCodes={diceCodes} />
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
