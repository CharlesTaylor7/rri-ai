import { useCallback } from 'react'
import useSelector from 'app/hooks/useSelector'
import useDispatch from 'app/hooks/useDispatch'
import styles from 'app/styles/Game.module.css'
import { rollDice } from 'app/store/game/actions'


export default function DiceButton() {
    const { text, onClick } = useProps()
    return (
        <button className={styles.diceButton} onClick={onClick}>
            {text}
        </button>
    )
}

function useProps() {
    const dispatch = useDispatch()
    const [routesPending, gameId] = useSelector(state => [state.game.routes.pending.length, state.game.id])

    const actionType = routesPending ? 'show_move' : 'roll_dice'
    const onClick = useCallback(() => dispatch({ type: actionType, gameId }), [routesPending])

    const text = routesPending ? 'Show Move' : 'Roll Dice'

    return { text, onClick }
}
