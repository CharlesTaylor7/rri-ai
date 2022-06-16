import { useCallback } from 'react'
import useSelector from 'app/hooks/useSelector'
import useDispatch from 'app/hooks/useDispatch'
import Button, { labelButtonStyle } from '../inputs/Button'
import {AppState} from 'app/types'

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
  const { pending, current } = useSelector((state) => state.routes)

  let onClick;
  if (pending.length > 0) {
    onClick = () => {
      dispatch({ current: [...current, ...pending], pending: [] })
    }
  }
  else {
    onClick = async () => {
      const { diceCodes, routesDrawn } = await fetch('/api/game/roll').then(res => res.json())
      dispatch({ pending: routesDrawn, diceCodes } )
    }
  }
  

  const text = pending.length > 0 ? 'Show Move' : 'Roll Dice'

  return { text, onClick }
}
