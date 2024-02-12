import useSelector from 'app/hooks/useSelector'
import useDispatch from 'app/hooks/useDispatch'

export default function DiceButton() {
  const { text, onClick } = useProps()
  return (
    <button className="p-2 rounded-lg bg-green-200" onClick={onClick}>
      {text}
    </button>
  )
}

function useProps() {
  const dispatch = useDispatch()
  const { gameId, round, pendingRoutes, currentRoutes } = useSelector(
    (state) => state,
  )

  let onClick
  if (pendingRoutes.length > 0) {
    onClick = () => {
      dispatch({
        round: round + 1,
        currentRoutes: [...currentRoutes, ...pendingRoutes],
        pendingRoutes: [],
      })
    }
  } else {
    onClick = async () => {
      const { diceCodes, nextRoutes: pendingRoutes } = await fetch(
        `/api/game/roll?gameId=${gameId}`,
      ).then((res) => res.json())
      dispatch({ diceCodes, pendingRoutes })
    }
  }

  const text = pendingRoutes.length > 0 ? 'Show Move' : 'Roll Dice'

  return { text, onClick }
}
