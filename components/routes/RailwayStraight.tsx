import { hatchSize, hatchLocation, cellLength } from 'rri-ai/constants'
import RouteComponent from '../RouteComponent';

const s = cellLength;

function RailwayStraight() {
    return (
        <>
            {// long vertical stroke
            }
            <line y1={0} y2={s} x1={s/2} x2={s/2} />

            {// 7 horizontal hatches
                Array.from({length: 7}, (_, i) => (
                    <line
                        key={i}
                        x1={s/2 - hatchSize} x2={s/2 + hatchSize}
                        y1={hatchLocation(i)} y2={hatchLocation(i)}
                    />
                ))
            }
        </>
    )
}
export default RouteComponent(RailwayStraight)
