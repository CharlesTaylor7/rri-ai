import useSelector from 'app/hooks/useSelector';
import styles from 'app/styles/Game.module.css'


export default function Dice() {
    const diceCodes = useSelector(state => state.game.diceCodes);
    return (
        <>
            {diceCodes.map(c => (
                <div className={styles.die}>
                    {c}
                </div>
            ))}
        </>
    )
}
