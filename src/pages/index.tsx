import { useRouter } from 'next/router'

export default function Home() {
  const router = useRouter()
  return (
    <button
      onClick={async () => {
        await fetch('/api/game/new').then((res) => res.json())
        router.push(`/game/`)
      }}
    >
      New Game
    </button>
  )
}
