import type { FunctionComponent } from 'react'
import type { RouteInfo } from 'app/types'
import styles from 'app/styles/Game.module.css'
import RouteDefinitions from 'app/components/RouteDefinitions'
import HalfHighway from 'app/components/routes/elements/HalfHighway'
import HalfRailway from 'app/components/routes/elements/HalfRailway'
import { cellLength } from 'app/constants'
import useSelector from 'app/hooks/useSelector'


function Grid () {
    const routes = useSelector(state => state.game.routes.current)
  console.log(routes)

    return (
        <svg className={styles.grid} viewBox="-0.5 -0.5 8 8" strokeWidth="0.01" >
            <g id="exits" strokeWidth={1}>
                {// North
                }
                <Exit translateX={1} translateY={-1} rotate={2} kind="highway" />
                <Exit translateX={3} translateY={-1} rotate={2} kind="railway" />
                <Exit translateX={5} translateY={-1} rotate={2} kind="highway" />

                {// South
                }
                <Exit translateX={1} translateY={7} rotate={0} kind="highway" />
                <Exit translateX={3} translateY={7} rotate={0} kind="railway" />
                <Exit translateX={5} translateY={7} rotate={0} kind="highway" />

                {// East
                }
                <Exit translateX={7} translateY={1} rotate={3} kind="railway" />
                <Exit translateX={7} translateY={3} rotate={3} kind="highway" />
                <Exit translateX={7} translateY={5} rotate={3} kind="railway" />

                {// West
                }
                <Exit translateX={-1} translateY={1} rotate={1} kind="railway" />
                <Exit translateX={-1} translateY={3} rotate={1} kind="highway" />
                <Exit translateX={-1} translateY={5} rotate={1} kind="railway" />
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
            <g id="drawn-routes">
                {routes.map((route: RouteInfo, i: number) => (<DrawnRoute key={i} route={route} />))}
            </g>

        </svg>
    )
}
export default Grid;

export interface ExitProps {
    kind: 'railway' | 'highway',
    translateX: number,
    translateY: number,
    rotate: number,
}

function Exit ({ kind, translateX, translateY, rotate }: ExitProps) {
    return (
        <g
            transform={`
                translate(${translateX}, ${translateY}),
                rotate(${rotate * 90}, 0.5, 0.5),
                scale(${1 / cellLength})
            `}
        >
            { kind == 'railway' ? <HalfRailway /> : <HalfHighway /> }
        </g>
    )
}


interface DrawnRouteProps {
    route: RouteInfo,
}
function DrawnRoute ({ route }: DrawnRouteProps) {
    return (
        <use
            href={`#route-${route.code}`}
            strokeWidth={1}
            transform={`
                translate(${route.x},${route.y}),
                rotate(${route.rotation * 90}, 0.5, 0.5),
                scale(${1 / cellLength})
            `}
        />
    )
}
