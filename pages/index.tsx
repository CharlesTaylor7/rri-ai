import { useCallback, useState } from 'react'
import { useRouter } from 'next/router'
import Link from 'next/link'
import Image from 'next/image'
import Page from '@/components/Page'
import Grid from '@/components/Grid'
import { RouteInfo } from '@/types'


export default function Home(props) {
    const router = useRouter()
    return (
        <Page>
            <button
                onClick={async () => {
                    const { gameId } = await fetch('/api/newGame').then(res => res.json())
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
