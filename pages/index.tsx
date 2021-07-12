import { useCallback, useState } from 'react'
import Head from 'next/head'
import Image from 'next/image'
import Grid from '@/components/Grid.tsx'
import { RouteInfo } from '@/types'

export default function Home(props) {
    const [routesDrawn, setRoutesDrawn] = useState(props.routesDrawn)
    return (
        <>
            <Head>
                <title>Railroad Inc. AI</title>
                <link rel="icon" href="/favicon.ico" />
            </Head>
            <main>
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
            </main>
        </>
    )
}

type StaticProps = {
    props: {
        routesDrawn: Array<RouteInfo>,
    },
}

export const getStaticProps = (): StaticProps => ({
    props: {
        routesDrawn: [],
    }
})
