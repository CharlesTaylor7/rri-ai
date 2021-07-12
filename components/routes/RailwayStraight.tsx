import { hatchSize } from '@/constants'

export default function RailwayStraight(props) {
    return (
        <g {...props} stroke="black">
            {// long vertical stroke
            }
            <line y1="0" y2="80" x1="40" x2="40"/>

            {// 7 horizontal hatches
                Array.from({length: 7}, (_, i) => (
                    <line
                        x1={40 - hatchSize} x2={40 + hatchSize}
                        y1={(i+1)*10} y2={(i+1)*10}
                    />
                ))
            }
        </g>
    )
}
