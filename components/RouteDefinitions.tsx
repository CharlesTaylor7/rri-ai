import RailwayTurn from '@/components/routes/RailwayTurn'
import RailwayT from '@/components/routes/RailwayT'
import RailwayStraight from '@/components/routes/RailwayStraight'
import HighwayTurn from '@/components/routes/HighwayTurn'
import HighwayT from '@/components/routes/HighwayT'
import HighwayStraight from '@/components/routes/HighwayStraight'
import Overpass from '@/components/routes/Overpass'
import StationStraight from '@/components/routes/StationStraight'
import StationTurn from '@/components/routes/StationTurn'
import StationHighwayT from '@/components/routes/StationHighwayT'
import StationRailwayT from '@/components/routes/StationRailwayT'
import HighwayFour from '@/components/routes/HighwayFour'
import RailwayFour from '@/components/routes/RailwayFour'
import StationFourTurn from '@/components/routes/StationFourTurn'
import StationFourStraight from '@/components/routes/StationFourStraight'

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
