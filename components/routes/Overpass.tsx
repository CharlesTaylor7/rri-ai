import { hatchSize, hatchLocation, cellLength, hwyDashPattern } from 'rri-ai/constants'
import { RouteComponent } from '../RouteComponent'


const s = cellLength;
const h = s / 2;
const w = hatchSize

function Overpass() {
    return (
        <>
            {// 2 long for the highway lines
            }
            <line y1={0} y2={s} x1={h-w} x2={h-w} />
            <line y1={0} y2={s} x1={h+w} x2={h+w} />
            {// dash hwy line
            }
            <line y1={0} y2={s} x1={h} x2={h} strokeDasharray={hwyDashPattern} />

            {// 2 short rail lines
            }
            <line x1={0} x2={h-w} y1={h} y2={h}/>
            <line x1={h+w} x2={s} y1={h} y2={h}/>

            {// 6 hatches on either side of the cross hatch
                Array.from({length: 7}, (_, i) =>
                    // skip the middle hatch
                    i == 3
                    ? null
                    :(<line
                        key={i}
                        y1={h - hatchSize} y2={h + hatchSize}
                        x1={hatchLocation(i)} x2={hatchLocation(i)}
                    />)
                )
            }
        </>
    )
}
export default RouteComponent(Overpass);
