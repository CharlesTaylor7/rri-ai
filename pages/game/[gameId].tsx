import { useCallback, useState } from 'react'
import Page from '@/components/Page'
import Image from 'next/image'
import Grid from '@/components/Grid'
import { RouteInfo } from '@/types'

export default function Game(props) {
    const [routesDrawn, setRoutesDrawn] = useState(props.routesDrawn)
    return (
        <Page>
            <Grid routesDrawn={routesDrawn}/>
            <button
                onClick={async () => {
                    const { routeCodes } = await fetch('/api/roll').then(res => res.json())
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

type GameProps = {
    props: {
        routesDrawn: Array<RouteInfo>,
    },
}

export async function getServerSideProps(context) {
    const { params: { gameId } } = context
    const props = await fetch(`http://localhost:3000/api/game/${gameId}/state`).then(res => res.json())
    return ({ props })
}
