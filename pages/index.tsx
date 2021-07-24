import { useCallback, useState } from 'react'
import { useRouter } from 'next/router'
import Link from 'next/link'
import Image from 'next/image'
import Page from '@/components/core/Page'
import { RouteInfo } from '@/types'
import styles from '@/styles/Home.module.css'


export default function Home(props) {
    const router = useRouter()
    return (
        <Page className={styles.homePage}>
            <button
                className={styles.newGameButton}
                onClick={async () => {
                    const { gameId } = await fetch('/api/game/new').then(res => res.json())
                    router.push(`/game/${gameId}`)
                }}
            >
                New Game
            </button>
        </Page>
    )
}
