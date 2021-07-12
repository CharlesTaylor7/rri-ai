import Head from 'next/head'
import Image from 'next/image'
import styles from '../styles/Home.module.css'
import Grid from '../components/Grid.tsx'

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
