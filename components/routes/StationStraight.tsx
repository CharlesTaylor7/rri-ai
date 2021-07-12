import { hatchSize, hatchLocation, cellLength, hwyDashPattern } from '@/constants'
import Station from './elements/Station'
import HalfRailway from './elements/HalfRailway'
import HalfHighway from './elements/HalfHighway'
import defaultProps from './defaultProps'

const s = cellLength;
const h = s / 2;
const w = hatchSize

export default function StationStraight(props) {
    return (
        <g {...props}>
            <HalfRailway />
            <Station />
            <HalfHighway rotate="2" />
        </g>
    )
}
StationStraight.defaultProps = defaultProps
