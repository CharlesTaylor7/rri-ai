import Station from './elements/Station'
import HalfRailway from './elements/HalfRailway'
import HalfHighway from './elements/HalfHighway'
import RouteComponent from '../RouteComponent'

function StationHighwayT() {
  return (
    <>
      <Station />
      <HalfHighway />
      <HalfHighway rotate={1} />
      <HalfRailway rotate={2} />
      <HalfHighway rotate={3} />
    </>
  )
}
export default RouteComponent(StationHighwayT)
