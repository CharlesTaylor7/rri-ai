import styles from './Grid.module.css'
import { routeSvgs } from '@/constants'
import { Route } from '@/types'


function Grid({ map = { 23: {code: 2} }}) {
    return (
        <div className={styles.gridRow}>
        <svg
            className={styles.grid}
            viewBox="-0.5 -0.5 7.5 7.5"
        >
            // horizontal rows
            {Array.from({length: 8}, (_, i) => (
                <line key={i} className={styles.gridLine}
                    strokeWidth={0.01}
                    x1="0" x2="7"
                    y1={i} y2={i}
                />)
            )}
            {Array.from({length: 8}, (_, i) => (
                <line key={i} className={styles.gridLine}
                    strokeWidth={0.01}
                    y1="0" y2="7"
                    x1={i} x2={i}
                />)
            )}
        </svg>
        </div>
    )
}


function Cell({ route }) {
    return (
        <div className={styles.cell} >
            <img src={routeSvgs[route?.code]} />
        </div>
    )
}

export default Grid
