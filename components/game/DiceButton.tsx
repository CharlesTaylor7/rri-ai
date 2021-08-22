import { useCallback } from 'react'
import useSelector from 'app/hooks/useSelector'
import useDispatch from 'app/hooks/useDispatch'
import styles from 'app/styles/Game.module.css'


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
    const routesPending = useSelector(state => state.game.routes.pending)

    const onClick = useCallback(() => dispatch({ type: routesPending ? 'show_move' : 'roll_dice'}),
        [routesPending],
    )

    const text = routesPending ? 'Show Move' : 'RollDice'

    return { text, onClick }
}
