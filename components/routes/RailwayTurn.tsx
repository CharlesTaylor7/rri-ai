import { hatchSize, hatchLocation, cellLength, hwyDashPattern } from '@/constants'

const s = cellLength;
const h = s / 2;
// highway width
const w = hatchSize

// TODO: use bezier curve / parabola to get a more rounded edge
export default function RailwayTurn(props) {
    return (
        <g {...props} strokeLinejoin="round" fill="none">
            {// railway line
            }
            <polyline points={`0,${h} ${h-w},${h} ${h},${h-w} ${h},0`} />
        </g>
    )
}
