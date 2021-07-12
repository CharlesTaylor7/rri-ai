import styles from './Grid.module.css'
import { RouteInfo } from '@/types'
import RouteDefinitions from '@/components/RouteDefinitions'


export default function Grid(props) {
    return (
        <div className={styles.gridRow}>
        <svg
            className={styles.grid}
            viewBox="0 0 7 7"
        >
            // horizontal rows
            {Array.from({length: 8}, (_, i) => (
                <line key={i} className={styles.gridLine}
                    strokeWidth={0.01}
                    x1={0} x2={7}
                    y1={i} y2={i}
                />)
            )}
            {Array.from({length: 8}, (_, i) => (
                <line key={i} className={styles.gridLine}
                    strokeWidth={0.01}
                    y1={0} y2={7}
                    x1={i} x2={i}
                />)
            )}
            <RouteDefinitions />
            // dynamic grid contents
            {props.routesDrawn.map((route, i) => (<DrawnRoute key={i} route={route} />))}

        </svg>
        </div>
    )
}

const DrawnRoute = ({ route }: {route: RouteInfo}) => (
    <use
        href={`#route-${route.code}`}
        transform={`
            translate(${route.x},${route.y}),
            rotate(${route.rotation * 90}, 0.5, 0.5),
            scale(0.0125)
        `}
    />
)
