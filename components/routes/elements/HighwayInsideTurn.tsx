import { highwayWidth, hatchLocation, cellLength, hwyDashPattern } from '@/constants'
import defaultProps from './defaultProps'

const h = cellLength / 2;
const w = highwayWidth / 2;

export default function HighwayInsideTurn(props) {
    return (
        <polyline
            points={`0,${h-w} ${h-2*w},${h-w} ${h-w},${h-2*w} ${h-w},0`}
            {...props}
        />
    )
}
