import styles from 'rri-ai/styles/Game.module.css'
import { FunctionComponent } from 'react'

type DiceProps = {
    diceCodes: Array<number>,
}

const Dice: FunctionComponent<DiceProps> = ({
    diceCodes,
}) => (
    <>
        {diceCodes.map(c => (
            <div className={styles.die}>
                {c}
            </div>
        ))}
    </>
)
export default Dice
