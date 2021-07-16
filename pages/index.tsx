import { useCallback, useState } from 'react'
import { useRouter } from 'next/router'
import Link from 'next/link'
import Image from 'next/image'
import Page from '@/components/Page'
import Grid from '@/components/Grid'
import { RouteInfo } from '@/types'
import styles from './Home.module.css'


export default function Home(props) {
    const router = useRouter()
    return (
        <Page className={styles.homePage}>
            <button
                className={styles.newGameButton}
                onClick={async () => {
                    const { gameId } = await fetch('/api/game/new').then(res => res.json())
                    router.push(`/games/${gameId}`)
                }}
            >
                New Game
            </button>
        </Page>
    )
}

type HomeProps = {
}

export const getStaticProps = (): HomeProps => ({
    props: {
    }
})
