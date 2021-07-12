import Head from 'next/head'
import Image from 'next/image'
import Grid from '@/components/Grid.tsx'
import { RouteInfo } from '@/types'

export default function Home(props) {
    return (
        <>
            <Head>
                <title>Railroad Inc. AI</title>
                <link rel="icon" href="/favicon.ico" />
            </Head>
            <main>
                <Grid routesDrawn={props.routesDrawn}/>
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
        routesDrawn: [
            { code: 1, x: 1, y: 0, rotation: 0},
            { code: 1, x: 1, y: 1, rotation: 0},
            { code: 2, x: 1, y: 2, rotation: 1},
        ]
    }
})
