import { hatchSize, crossHatchSize, hatchLocation, cellLength } from 'rri-ai/constants'
import RouteComponent from '../RouteComponent';

const s = cellLength
const h = s / 2;
// highway width
const w = hatchSize
const c = crossHatchSize
const crossS = h - (w/2) - c;
const crossE = h - (w/2) + c;

// TODO: use bezier curve / parabola to get a more rounded edge
function RailwayTurn() {
    return (
        <>
            {// railway line
            }
            <polyline points={`0,${h} ${h-w},${h} ${h},${h-w} ${h},0`} />

            {// diagonal hatch -->
            }
            <line x1={crossS} x2={crossE} y1={crossS} y2={crossE}/>

            {// 3 hatches above
                Array.from({length: 3}, (_, i) => (
                    <line
                        key={i}
                        x1={h - hatchSize} x2={h + hatchSize}
                        y1={hatchLocation(i)} y2={hatchLocation(i)}
                    />
                ))
            }
            {// 3 hatches to the left
                Array.from({length: 3}, (_, i) => (
                    <line
                        key={i}
                        y1={h - hatchSize} y2={h + hatchSize}
                        x1={hatchLocation(i)} x2={hatchLocation(i)}
                    />
                ))
            }
        </>
    )
}

export default RouteComponent(RailwayTurn)
