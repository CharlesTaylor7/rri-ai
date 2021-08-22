import { hatchSize, hatchLocation, cellLength, hwyDashPattern } from 'rri-ai/constants'
import RouteComponent from '../RouteComponent';

const s = cellLength;
const h = s / 2;
const w = hatchSize

function HighwayStraight(props) {
    return (
        <g {...props}>
            {// 2 long vertical lines
            }
            <line y1={0} y2={s} x1={h-w} x2={h-w} />
            <line y1={0} y2={s} x1={h+w} x2={h+w} />
            {// dash hwy line
            }
            <line y1={0} y2={s} x1={h} x2={h} strokeDasharray={hwyDashPattern} />
        </g>
    )
}
export default RouteComponent(HighwayStraight)
