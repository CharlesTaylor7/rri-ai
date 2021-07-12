import { highwayWidth, cellLength, hwyDashPattern } from '@/constants'

const h = cellLength / 2;
const w = highwayWidth / 2;

export default function HalfHighway(props) {
    return (
        <g {...props}>
            {// 2 vertical lines
            }
            <line y1={0} y2={h-w} x1={h-w} x2={h-w} />
            <line y1={0} y2={h-w} x1={h+w} x2={h+w} />
            {// dash hwy line
            }
            <line y1={0} y2={h-w} x1={h} x2={h} strokeDasharray={hwyDashPattern} />
        </g>
    )
}
