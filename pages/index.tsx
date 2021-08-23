import { useRouter } from 'next/router'
import styles from 'app/styles/Home.module.css'


export default function Home() {
    const router = useRouter()
    return (
        <button
            className={styles.newGameButton}
            onClick={async () => {
                await fetch('/api/game/new').then(res => res.json())
                router.push(`/game/`)
            }}
        >
            New Game
        </button>
    )
}
