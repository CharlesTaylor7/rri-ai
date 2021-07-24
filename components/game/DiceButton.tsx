import { useContext, useCallback } from 'react'
import Store from '@/stores/game'
import styles from '@/styles/Game.module.css'


function DiceButton({ text, onClick }) {
    return (
        <button className={styles.diceButton} onClick={onClick}>
            {text}
        </button>
    )
}

function mapStateToProps(state) {
    const {
        dispatch,
        state: {
            routes: {
                pending: routesPending,
            }
        }
    }

    const onClick = useCallback(
        () => dispatch(routesPending ? 'show_move' : 'roll_dice'),
        [routesPending],
    )

    const text = routesPending ? 'Show Move' : 'RollDice'

    return { text, onClick }
}

export default connect(mapStateToProps)(DiceButton)
