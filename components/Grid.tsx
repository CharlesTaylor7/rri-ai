import styles from './Grid.module.css'
import { RouteInfo } from '@/types'
import RouteDefinitions from '@/components/RouteDefinitions'
import HalfHighway from '@/components/routes/elements/HalfHighway'
import HalfRailway from '@/components/routes/elements/HalfRailway'
import defaultRouteProps from '@/components/routes/defaultProps'
import { cellLength } from '@/constants'


function Exit({ kind, translateX, translateY, rotate }) {
    return (
        <g
            transform={`
                translate(${translateX}, ${translateY}),
                rotate(${rotate*90}, 0.5, 0.5),
                scale(${1 / cellLength})
            `}
        >
            { kind == 'railway' ? <HalfRailway /> : <HalfHighway /> }
        </g>
    )
}

export default function Grid(props) {
    return (
        <div className={styles.gridRow}>
        <svg className={styles.grid} viewBox="-0.5 -0.5 8 8" strokeWidth="0.01" >
            <g id="exits" strokeWidth="1">
                {// North
                }
                <Exit translateX="1" translateY="-1" rotate="2" kind='highway' />
                <Exit translateX="3" translateY="-1" rotate="2" kind='railway' />
                <Exit translateX="5" translateY="-1" rotate="2" kind='highway' />

                {// South
                }
                <Exit translateX="1" translateY="7" rotate="0" kind='highway' />
                <Exit translateX="3" translateY="7" rotate="0" kind='railway' />
                <Exit translateX="5" translateY="7" rotate="0" kind='highway' />

                {// East
                }
                <Exit translateX="7" translateY="1" rotate="3" kind='railway' />
                <Exit translateX="7" translateY="3" rotate="3" kind='highway' />
                <Exit translateX="7" translateY="5" rotate="3" kind='railway' />

                {// West
                }
                <Exit translateX="-1" translateY="1" rotate="1" kind='railway' />
                <Exit translateX="-1" translateY="3" rotate="1" kind='highway' />
                <Exit translateX="-1" translateY="5" rotate="1" kind='railway' />
            </g>
            <g id="grid-lines">
                {Array.from({length: 8}, (_, i) => (
                    <line key={i}
                        x1={0} x2={7}
                        y1={i} y2={i}
                    />)
                )}
                {Array.from({length: 8}, (_, i) => (
                    <line key={i}
                        y1={0} y2={7}
                        x1={i} x2={i}
                    />)
                )}
            </g>
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
        strokeWidth="1"
        transform={`
            translate(${route.x},${route.y}),
            rotate(${route.rotate * 90}, 0.5, 0.5),
            scale(${1 / cellLength})
        `}
    />
)

