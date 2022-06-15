import Station from './elements/Station'
import HalfRailway from './elements/HalfRailway'
import HalfHighway from './elements/HalfHighway'
import RouteComponent from '../RouteComponent'

function StationTurn() {
  return (
    <>
      <HalfRailway />
      <Station />
      <HalfHighway rotate={3} />
    </>
  )
}
export default RouteComponent(StationTurn)
