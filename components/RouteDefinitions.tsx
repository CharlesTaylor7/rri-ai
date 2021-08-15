import RailwayTurn from 'rri-ai/components/routes/RailwayTurn'
import RailwayT from 'rri-ai/components/routes/RailwayT'
import RailwayStraight from 'rri-ai/components/routes/RailwayStraight'
import HighwayTurn from 'rri-ai/components/routes/HighwayTurn'
import HighwayT from 'rri-ai/components/routes/HighwayT'
import HighwayStraight from 'rri-ai/components/routes/HighwayStraight'
import Overpass from 'rri-ai/components/routes/Overpass'
import StationStraight from 'rri-ai/components/routes/StationStraight'
import StationTurn from 'rri-ai/components/routes/StationTurn'
import StationHighwayT from 'rri-ai/components/routes/StationHighwayT'
import StationRailwayT from 'rri-ai/components/routes/StationRailwayT'
import HighwayFour from 'rri-ai/components/routes/HighwayFour'
import RailwayFour from 'rri-ai/components/routes/RailwayFour'
import StationFourTurn from 'rri-ai/components/routes/StationFourTurn'
import StationFourStraight from 'rri-ai/components/routes/StationFourStraight'

// this component provides svg <defs> to be used inside the grid
export default function RouteDefinitions () {
    return (
        <defs>
            // as listed on the player boards in order from left to right:
            <RailwayTurn id="route-0" />
            <RailwayT id="route-1" />
            <RailwayStraight id="route-2" />
            <HighwayTurn id="route-3" />
            <HighwayT id="route-4" />
            <HighwayStraight id="route-5" />
            <Overpass id="route-6" />
            <StationStraight id="route-7" />
            <StationTurn id="route-8" />
            <StationHighwayT id="route-9" />
            <StationRailwayT id="route-10" />
            <HighwayFour id="route-11" />
            <RailwayFour id="route-12" />
            <StationFourTurn id="route-13" />
            <StationFourStraight id="route-14" />
        </defs>
    )
}
