import { hatchSize, hatchLocation, cellLength, hwyDashPattern } from '@/constants'
import HighwayInsideTurn from './elements/HighwayInsideTurn'
import defaultProps from './defaultProps'

const s = cellLength;
const h = s / 2;
// highway width
const w = hatchSize

// TODO: use bezier curve / parabola to get a more rounded edge
export default function HighwayTurn(props) {
    return (
        <g {...props}>
            {// left curved path
            }
            <HighwayInsideTurn />
            <HighwayInsideTurn transform={`rotate(90,${h},${h})`} />

            {// horizontal line
            }
            <line x1={0} x2={s} y1={h+w} y2={h+w} />

            {// dashed horizontal line
            }
            <line x1={0} x2={s} y1={h} y2={h} strokeDasharray={hwyDashPattern}/>

            {// dashed vertical line
            }
            <line x1={h} x2={h} y1={0} y2={h} strokeDasharray={hwyDashPattern}/>
        </g>
    )
}

HighwayTurn.defaultProps = defaultProps
