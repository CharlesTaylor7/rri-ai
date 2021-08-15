import { useRouter } from 'next/router'
import styles from 'rri-ai/styles/Home.module.css'


export default function Home() {
    const router = useRouter()
    return (
        <button
            className={styles.newGameButton}
            onClick={async () => {
                const { gameId } = await fetch('/api/game/new').then(res => res.json())
                router.push(`/game/${gameId}`)
            }}
        >
            New Game
        </button>
    )
}
