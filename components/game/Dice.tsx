import styles from 'rri-ai/styles/Game.module.css'

type DiceProps = {
    diceCodes: Array<number>,
}

export default function Dice(props: DiceProps) {
    const { diceCodes } = props;
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
