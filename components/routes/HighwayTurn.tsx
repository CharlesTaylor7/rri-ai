import { hatchSize, hatchLocation, cellLength, hwyDashPattern } from 'rri-ai/constants'
import RouteComponent from '../RouteComponent';
import HighwayInsideTurn from './elements/HighwayInsideTurn'

const s = cellLength;
const h = s / 2;
// highway width
const w = hatchSize

// TODO: use bezier curve / parabola to get a more rounded edge
function HighwayTurn() {
    return (
        <>
            <HighwayInsideTurn />

            {// outside curved line
            }
            <polyline points={`0,${h+w} ${h},${h+w} ${h+w},${h} ${h+w},0`} />

            {// 1 dashed hwy line
            <polyline points={`0,${h} ${h-w},${h} ${h},${h-w} ${h},0`} strokeDasharray={hwyDashPattern}/>}
        </>
    )
}
export default RouteComponent(HighwayTurn)
