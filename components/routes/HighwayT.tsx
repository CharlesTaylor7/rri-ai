import { hatchSize, hatchLocation, cellLength, hwyDashPattern } from 'app/constants'
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
            {// left curved path
            }
            <HighwayInsideTurn />
            <HighwayInsideTurn rotate={1} />

            {// horizontal line
            }
            <line x1={0} x2={s} y1={h+w} y2={h+w} />

            {// dashed horizontal line
            }
            <line x1={0} x2={s} y1={h} y2={h} strokeDasharray={hwyDashPattern}/>

            {// dashed vertical line
            }
            <line x1={h} x2={h} y1={0} y2={h} strokeDasharray={hwyDashPattern}/>
        </>
    )
}
export default RouteComponent(HighwayTurn)
