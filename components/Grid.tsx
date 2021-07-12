import styles from './Grid.module.css'


function Grid() {
    return (
        <div className={styles.grid}>
            {Array.from({length: 49}, (_, i) => (<Cell key={i}/>))}
        </div>
    )
}

function Cell() {
    return ( <div className={styles.cell} />)
}

export default Grid
