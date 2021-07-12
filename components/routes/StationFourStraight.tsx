import Station from './elements/Station'
import HalfRailway from './elements/HalfRailway'
import HalfHighway from './elements/HalfHighway'
import defaultProps from './defaultProps'


export default function StationFourStraight(props) {
    return (
        <g {...props}>
            <Station />
            <HalfHighway />
            <HalfRailway rotate="1"/>
            <HalfHighway rotate="2"/>
            <HalfRailway rotate="3"/>
        </g>
    )
}
StationFourStraight.defaultProps = defaultProps
