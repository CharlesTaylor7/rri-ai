import { hatchSize, hatchLocation, cellLength } from 'rri-ai/constants'
import RouteComponent from '../RouteComponent';

const s = cellLength;
const h = s/2;

function RailwayStraight() {
    return (
        <>
            {// vertical rail line
            }
            <line y1={0} y2={s} x1={h} x2={h} />

            {// horizontal rail line
            }
            <line x1={0} x2={s} y1={h} y2={h} />

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
            {// 6 hatches above and below the cross hatch
                Array.from({length: 7}, (_, i) =>
                    // skip the middle hatch
                    i == 3
                    ? null
                    :(<line
                        key={i}
                        x1={h - hatchSize} x2={h + hatchSize}
                        y1={hatchLocation(i)} y2={hatchLocation(i)}
                    />)
                )
            }
        </>
    )
}
export default RouteComponent(RailwayStraight)
