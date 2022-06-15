import Station from './elements/Station'
import HalfRailway from './elements/HalfRailway'
import HalfHighway from './elements/HalfHighway'
import RouteComponent from '../RouteComponent'

function StationFourTurn() {
  return (
    <>
      <Station />
      <HalfHighway />
      <HalfRailway rotate={1} />
      <HalfRailway rotate={2} />
      <HalfHighway rotate={3} />
    </>
  )
}
export default RouteComponent(StationFourTurn)
