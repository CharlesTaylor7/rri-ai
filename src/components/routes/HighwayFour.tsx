import { cellLength, hwyDashPattern } from 'app/constants'
import HighwayInsideTurn from './elements/HighwayInsideTurn'
import RouteComponent from '../RouteComponent'

const s = cellLength
const h = s / 2

function HighwayFour() {
  return (
    <>
      <HighwayInsideTurn />
      <HighwayInsideTurn rotate={1} />
      <HighwayInsideTurn rotate={2} />
      <HighwayInsideTurn rotate={3} />

      {
        // horizontal dashed hwy line
      }
      <line x1={0} x2={s} y1={h} y2={h} strokeDasharray={hwyDashPattern} />

      {
        // vertical dashed hwy line
      }
      <line y1={0} y2={s} x1={h} x2={h} strokeDasharray={hwyDashPattern} />
    </>
  )
}
export default RouteComponent(HighwayFour)
