import { useContext, useCallback } from 'react'
import { useStore } from '@/store/game'
import styles from '@/styles/Game.module.css'


export default function DiceButton() {
    const { text, onClick } = useProps()
    return (
        <button className={styles.diceButton} onClick={onClick}>
            {text}
        </button>
    )
}

function useProps() {
    const {
        dispatch,
        state: {
            routes: {
                pending: routesPending,
            }
        }
    } = useStore()

    const onClick = useCallback(
        () => dispatch(routesPending ? 'show_move' : 'roll_dice'),
        [routesPending],
    )

    const text = routesPending ? 'Show Move' : 'RollDice'

    return { text, onClick }
}
