import Station from './elements/Station'
import HalfRailway from './elements/HalfRailway'
import HalfHighway from './elements/HalfHighway'
import RouteComponent from '../RouteComponent'


function StationFourStraight() {
    return (
        <>
            <Station />
            <HalfHighway />
            <HalfRailway rotate="1"/>
            <HalfHighway rotate="2"/>
            <HalfRailway rotate="3"/>
        </>
    )
}
export default RouteComponent(StationFourStraight)
