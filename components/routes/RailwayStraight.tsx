import { hatchSize, hatchLocation, cellLength } from 'rri-ai/constants'
import defaultProps from './defaultProps'

const s = cellLength;

export default function RailwayStraight(props) {
    return (
        <g {...props}>
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
        </g>
    )
}
RailwayStraight.defaultProps = defaultProps
