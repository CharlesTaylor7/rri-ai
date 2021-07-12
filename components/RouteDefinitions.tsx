import RailwayStraight from '@/components/routes/RailwayStraight'
import RailwayT from '@/components/routes/RailwayT'

// this component provides svg <defs> to be used inside the grid
function RouteDefinitions() {
    return (
        <defs>
            <RailwayStraight id="route-1" />
            <RailwayT id="route-2" />
        </defs>
    )
}
// as listed on the player boards in order from left to right:
const routes = [
    'railway-turn',
    'railway-t',
    'railway-straight',
    'highway-turn',
    'highway-t',
    'highway-straight',
    'overpass',
    'station-straight',
    'station-turn',
    'highway-t-station',
    'railway-t-station',
    'highway-four',
    'railway-four',
    'turn-station',
    'straight-station',
]

export default RouteDefinitions
