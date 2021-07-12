import Station from './elements/Station'
import HalfRailway from './elements/HalfRailway'
import HalfHighway from './elements/HalfHighway'
import defaultProps from './defaultProps'


export default function StationFourTurn(props) {
    return (
        <g {...props}>
            <Station />
            <HalfHighway />
            <HalfRailway rotate="1"/>
            <HalfRailway rotate="2"/>
            <HalfHighway rotate="3"/>
        </g>
    )
}
StationFourTurn.defaultProps = defaultProps
