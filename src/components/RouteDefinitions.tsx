import RailwayTurn from 'app/components/routes/RailwayTurn'
import RailwayT from 'app/components/routes/RailwayT'
import RailwayStraight from 'app/components/routes/RailwayStraight'
import HighwayTurn from 'app/components/routes/HighwayTurn'
import HighwayT from 'app/components/routes/HighwayT'
import HighwayStraight from 'app/components/routes/HighwayStraight'
import Overpass from 'app/components/routes/Overpass'
import StationStraight from 'app/components/routes/StationStraight'
import StationTurn from 'app/components/routes/StationTurn'
import StationHighwayT from 'app/components/routes/StationHighwayT'
import StationRailwayT from 'app/components/routes/StationRailwayT'
import HighwayFour from 'app/components/routes/HighwayFour'
import RailwayFour from 'app/components/routes/RailwayFour'
import StationFourTurn from 'app/components/routes/StationFourTurn'
import StationFourStraight from 'app/components/routes/StationFourStraight'

// this component provides svg <defs> to be used inside the grid
export default function RouteDefinitions() {
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
