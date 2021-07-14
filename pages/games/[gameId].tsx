import { useCallback, useState, useEffect } from 'react'
import Page from '@/components/Page'
import Image from 'next/image'
import Grid from '@/components/Grid'
import Dice from '@/components/Dice'
import { RouteInfo } from '@/types'

export default function Game(props) {
    const [routesDrawn, setRoutesDrawn] = useState(props.routesDrawn)
    const [diceCodes, setDiceCodes] = useState([])
    return (
        <Page error={props.error}>
            <Grid routesDrawn={routesDrawn} />
            <Dice diceCodes={diceCodes} />
            <button
                onClick={async () => {
                    const { routeCodes } = await fetch(`/api/game/roll/?id=${props.id}`).then(res => res.json())
                    const routeInfo = routeCodes.map(
                        (code, i) => ({ code, x: i, y: 0, rotate: i})
                    )
                    setRoutesDrawn(routeInfo)
                }}
                style={{
                    fontSize: '50px'
                }}
            >
                Roll!
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
    const response = await fetch(`http://localhost:3000/api/game/state/?id=${gameId}`)
    if (response.status != 200) {
        const error = { statusCode: response.status, title: 'Game does not exist' }
        return { props: { error } }
    }

    const { routesDrawn } = await response.json()
    return ({
        props: {
            id: gameId,
            routesDrawn,
        },
    })
}
