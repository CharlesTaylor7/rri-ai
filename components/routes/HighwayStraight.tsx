import { hatchSize, hatchLocation, cellLength, hwyDashPattern } from '@/constants'
import defaultProps from './defaultProps'


const s = cellLength;
const h = s / 2;
const w = hatchSize

export default function HighwayStraight(props) {
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
HighwayStraight.defaultProps = defaultProps
