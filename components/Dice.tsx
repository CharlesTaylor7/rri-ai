import styles from '@/styles/Game.module.css'


export default function Dice(props) {
    const { diceCodes } = props
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
