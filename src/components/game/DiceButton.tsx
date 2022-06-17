import useSelector from 'app/hooks/useSelector'
import useDispatch from 'app/hooks/useDispatch'
import Button, { labelButtonStyle } from '../inputs/Button'

export default function DiceButton() {
  const { text, onClick } = useProps()
  return (
    <Button className={labelButtonStyle('bg-green-200')} onClick={onClick}>
      {text}
    </Button>
  )
}

function useProps() {
  const dispatch = useDispatch()
  const {
    gameId,
    pendingRoutes,
    currentRoutes,
  } = useSelector((state) => state)

  let onClick
  if (pendingRoutes.length > 0) {
    onClick = () => {
      dispatch({ currentRoutes: [...currentRoutes, ...pendingRoutes], pendingRoutes: [] })
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
