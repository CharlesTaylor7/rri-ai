import { useCallback } from 'react'
import useSelector from 'app/hooks/useSelector'
import useDispatch from 'app/hooks/useDispatch'
import { rollDice } from 'app/store/game/actions'
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
  const routesPending = useSelector((state) => state.game.routes.pending.length)

  const actionType = routesPending ? 'show_move' : 'roll_dice'
  const onClick = useCallback(
    () => dispatch({ type: actionType }),
    [routesPending],
  )

  const text = routesPending ? 'Show Move' : 'Roll Dice'

  return { text, onClick }
}
