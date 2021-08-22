import { hatchSize, hatchLocation, cellLength, hwyDashPattern } from 'app/constants'
import Station from './elements/Station'
import HalfRailway from './elements/HalfRailway'
import HalfHighway from './elements/HalfHighway'
import RouteComponent from '../RouteComponent'


function StationStraight() {
    return (
        <>
            <HalfRailway />
            <Station />
            <HalfHighway rotate={2} />
        </>
    )
}
export default RouteComponent(StationStraight)
