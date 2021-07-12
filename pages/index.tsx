import Head from 'next/head'
import Image from 'next/image'
import Grid from '@/components/Grid.tsx'

export default function Home() {
    return (
        <>
            <Head>
                <title>Railroad Inc. AI</title>
                <link rel="icon" href="/favicon.ico" />
            </Head>
            <main>
                <Grid/>
            </main>
        </>
    )
}
