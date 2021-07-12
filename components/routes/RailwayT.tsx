import { hatchSize, crossHatchSize, hatchLocation, cellLength } from '@/constants'

const s = cellLength;
const h = s / 2;
const crossS = h - crossHatchSize;
const crossE = h + crossHatchSize;

export default function RailwayT(props) {
    return (
        <g {...props} stroke="black">

            {// long horizontal stroke
            }
            <line x1={0} x2={s} y1={h} y2={h}/>

            {// vertical stroke above cross hatch
            }
            <line x1={h} x2={h} y1={0} y2={h}/>
            {// cross hatch -->
            }
            <line x1={crossS} x2={crossE} y1={crossS} y2={crossE}/>
            <line x1={crossS} x2={crossE} y1={crossE} y2={crossS}/>

            {// 3 hatches above the cross hatch
                Array.from({length: 3}, (_, i) => (
                    <line
                        x1={h - hatchSize} x2={h + hatchSize}
                        y1={hatchLocation(i)} y2={hatchLocation(i)}
                    />
                ))
            }
            {// 6 hatches on either side of the cross hatch
                Array.from({length: 7}, (_, i) =>
                    // skip the middle hatch
                    i == 3
                    ? null
                    :(<line
                        y1={h - hatchSize} y2={h + hatchSize}
                        x1={hatchLocation(i)} x2={hatchLocation(i)}
                    />)
                )
            }
        </g>
    )
}
