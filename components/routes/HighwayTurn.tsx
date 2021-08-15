import { hatchSize, hatchLocation, cellLength, hwyDashPattern } from 'rri-ai/constants'
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
            <HighwayInsideTurn />

            {// outside curved line
            }
            <polyline points={`0,${h+w} ${h},${h+w} ${h+w},${h} ${h+w},0`} />

            {// 1 dashed hwy line
            <polyline points={`0,${h} ${h-w},${h} ${h},${h-w} ${h},0`} strokeDasharray={hwyDashPattern}/>}
        </g>
    )
}
HighwayTurn.defaultProps = defaultProps
