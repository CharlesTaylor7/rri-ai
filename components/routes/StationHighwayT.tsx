import Station from './elements/Station'
import HalfRailway from './elements/HalfRailway'
import HalfHighway from './elements/HalfHighway'
import defaultProps from './defaultProps'


export default function StationHighwayT(props) {
    return (
        <g {...props}>
            <Station />
            <HalfHighway />
            <HalfHighway rotate="1"/>
            <HalfRailway rotate="2"/>
            <HalfHighway rotate="3"/>
        </g>
    )
}
StationHighwayT.defaultProps = defaultProps
