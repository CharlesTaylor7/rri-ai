import { hatchSize, hatchLocation, cellLength } from 'rri-ai/constants'

const h = cellLength / 2;

export default function HalfRailway(props) {
    const { rotate, ...rest } = props
    return (
        <g transform={`rotate(${rotate*90},${h},${h})`} {...rest} >
            {// railway line
            }
            <line y1={0} y2={h} x1={h} x2={h} />

            {// 3 horizontal hatches
                Array.from({length: 3}, (_, i) => (
                    <line
                        key={i}
                        x1={h - hatchSize} x2={h + hatchSize}
                        y1={hatchLocation(i)} y2={hatchLocation(i)}
                    />
                ))
            }
        </g>
    )
}
HalfRailway.defaultProps = {
    rotate: 0
}
