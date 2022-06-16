import Button, { labelButtonStyle } from 'app/components/inputs/Button'
import Link from 'next/link'
import { v4 as uuid } from 'uuid'

type Props = {
  newGameId: string
  games: Array<{ uuid: string; createdAt: string }>
}

const doNothing = () => {}
export default function Home(props: Props) {
  const gameLink = (uuid: string, label: string) => (
    <Button
      key={uuid}
      className={labelButtonStyle('bg-green-200')}
      onClick={doNothing}
    >
      <Link href="/game/[uuid]" as={`/game/${uuid}`}>
        {label}
      </Link>
    </Button>
  )
  return (
    <div className="flex flex-col items-center p-8 text-3xl">
      {gameLink(props.newGameId, 'New Game')}
      {props.games.map((game) => gameLink(game.uuid, game.createdAt))}
    </div>
  )
}

type SSR<P> = { props: P }
export async function getServerSideProps(): Promise<SSR<Props>> {
  return {
    props: {
      newGameId: String(uuid()),
      games: [],
    },
  }
}
