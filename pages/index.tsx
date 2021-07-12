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
            { code: 0, x: 0, y: 0, rotation: 0 },
            { code: 1, x: 1, y: 0, rotation: 0 },
            { code: 2, x: 2, y: 0, rotation: 0 },
            { code: 3, x: 0, y: 1, rotation: 0 },
            { code: 4, x: 1, y: 1, rotation: 0 },
            { code: 5, x: 2, y: 1, rotation: 0 },
            { code: 6, x: 0, y: 2, rotation: 0 },
            { code: 7, x: 1, y: 2, rotation: 0 },
            { code: 8, x: 2, y: 2, rotation: 0 },
            { code: 9, x: 5, y: 0, rotation: 0 },
            { code: 10, x: 6, y: 0, rotation: 0 },
            { code: 11, x: 5, y: 1, rotation: 0 },
            { code: 12, x: 6, y: 1, rotation: 0 },
            { code: 13, x: 5, y: 2, rotation: 0 },
            { code: 14, x: 6, y: 2, rotation: 0 },
        ]
    }
})
