import { hatchSize, hatchLocation, cellLength, hwyDashPattern } from '@/constants'

const s = cellLength;
const h = s / 2;
const hwyWidth = hatchSize

export default function HighwayStraight(props) {
    return (
        <g {...props}>
            {// 2 long vertical lines
            }
            <line y1={0} y2={s} x1={h-hwyWidth} x2={h-hwyWidth} />
            <line y1={0} y2={s} x1={h+hwyWidth} x2={h+hwyWidth} />
            {// dash hwy line
            }
            <line y1={0} y2={s} x1={h} x2={h} strokeDasharray={hwyDashPattern} />
        </g>
    )
}
