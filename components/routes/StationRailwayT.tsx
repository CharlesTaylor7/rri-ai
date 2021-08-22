import Station from './elements/Station'
import HalfRailway from './elements/HalfRailway'
import HalfHighway from './elements/HalfHighway'
import RouteComponent from '../RouteComponent'


function StationHighwayT() {
    return (
        <g>
            <Station />
            <HalfHighway />
            <HalfRailway rotate={1} />
            <HalfRailway rotate={2}/>
            <HalfRailway rotate={3} />
        </g>
    )
}
export default RouteComponent(StationHighwayT)
