import styles from './Grid.module.css'
import { routeSvgs } from '@/constants'
import { Route } from '@/types'


function Grid({ map = { 23: {code: 2} }}) {
    return (
        <div className={styles.grid}>
            {Array.from({length: 49}, (_, i) => (<Cell key={i} route={map[i]}/>))}
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
