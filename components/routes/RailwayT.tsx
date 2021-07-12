import { hatchSize, crossHatchSize } from '@/constants'

export default function RailwayT(props) {
    return (
        <g {...props} stroke="black">

            {// long horizontal stroke
            }
            <line x1="0" x2="80" y1="40" y2="40"/>

            {// vertical stroke above cross hatch
            }
            <line x1="40" x2="40" y1="0" y2="40"/>
            {// cross hatch -->
            }
            <line x1="35" x2="45" y1="35" y2="45"/>
            <line x1="35" x2="45" y1="45" y2="35"/>
            {// 3 hatches above the cross hatch
                Array.from({length: 3}, (_, i) => (
                    <line
                        x1={40 - hatchSize} x2={40 + hatchSize}
                        y1={(i+1)*10} y2={(i+1)*10}
                    />
                ))
            }
            {// 6 hatches on either side of the cross hatch
                Array.from({length: 7}, (_, i) =>
                    // skip the middle hatch
                    i == 3
                    ? null
                    :(<line
                        y1={40 - hatchSize} y2={40 + hatchSize}
                        x1={(i+1)*10} x2={(i+1)*10}
                    />)
                )
            }
        </g>
    )
}
