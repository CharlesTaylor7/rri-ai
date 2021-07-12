import { hatchSize, hatchLocation, cellLength, hwyDashPattern } from '@/constants'
import Station from './elements/Station'
import HalfRailway from './elements/HalfRailway'
import HalfHighway from './elements/HalfHighway'
import defaultProps from './defaultProps'

const s = cellLength;
const h = s / 2;
const w = hatchSize

export default function StationTurn(props) {
    return (
        <g {...props}>
            <HalfRailway />
            <Station />
            <HalfHighway transform={`rotate(-90,${h},${h})`} />
        </g>
    )
}
StationTurn.defaultProps = defaultProps
