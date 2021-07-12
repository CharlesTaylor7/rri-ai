import Head from 'next/head'
import Image from 'next/image'
import Grid from '@/components/Grid.tsx'
import styles from '@/styles/Home.module.css'

export default function Home() {
    return (
        <div className={styles.container}>
            <Head>
                <title>Railroad Inc. AI</title>
                <link rel="icon" href="/favicon.ico" />
            </Head>
            <main className={styles.main}>
                <Grid/>
            </main>
        </div>
    )
}
