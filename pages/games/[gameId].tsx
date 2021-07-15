import { useCallback, useState, useEffect } from 'react'
import Page from '@/components/Page'
import Image from 'next/image'
import Grid from '@/components/Grid'
import Dice from '@/components/Dice'
import { RouteInfo } from '@/types'
import { state } from '@/server/state'



export default function Game(props) {
    const [routesDrawn, setRoutesDrawn] = useState(props.routesDrawn)
    const [nextRoutes, setNextRoutes] = useState([])
    const [diceCodes, setDiceCodes] = useState([])

    const rollCallback = useCallback(async () => {
        const url = `/api/game/roll/?id=${props.id}`
        const { diceCodes, nextRoutes } = await fetch(url).then(res => res.json())
        setDiceCodes(diceCodes)
        setNextRoutes(nextRoutes)
    })

    const showMoveCallback = useCallback(async () => {
        setRoutesDrawn([...routesDrawn, ...nextRoutes])
        setNextRoutes([])
    })
    return (
        <Page error={props.error}>
            <Grid routesDrawn={routesDrawn} />
            <Dice diceCodes={diceCodes} />
            <button
                style={{position: 'fixed', top: '20px', right: '10%', fontSize: '50px'}}
                onClick={nextRoutes.length > 0 ? showMoveCallback : rollCallback}
            >
                {nextRoutes.length > 0 ? 'Show Move' : 'Roll!!!!'}
            </button>
        </Page>
    )
}
Game.defaultProps = {
    routesDrawn: [],
}

type GameProps = {
    props: {
        routesDrawn: Array<RouteInfo>,
    },
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
            ...gameState,
        },
    })
}
