import db from '@/server/db'
import { GetServerSideProps } from 'next'
import Link from 'next/link'
import { v4 as uuid } from 'uuid'

type Props = {
  newGameId: string
  games: Array<{ uuid: string; createdAt: string }>
}

const doNothing = () => {};

export default function Home(props: Props) {
  const gameLink = (uuid: string, label: string) => (
    <button
      key={uuid}
      className="p-2 rounded-lg bg-green-200"
      onClick={doNothing}
    >
      <Link href="/game/[uuid]" as={`/game/${uuid}`}>
        {label}
      </Link>
    </button>
  )
  return (
    <div className="p-8 text-3xl flex flex-col gap-4 items-center">
      {gameLink(props.newGameId, 'New Game')}
      {props.games.map((game) => gameLink(game.uuid, game.createdAt))}
    </div>
  )
}

export const getServerSideProps: GetServerSideProps = async (context) => {
  const games = await db
    .select('uuid', 'created_at')
    .from('games')
    .orderBy('created_at', 'desc')
    .limit(5)
    .then((rows) =>
      rows.map((row) => ({
        uuid: row.uuid,
        createdAt: String(row.created_at),
      })),
    )
  return {
    props: {
      newGameId: String(uuid()),
      games: games,
    },
  }
}
