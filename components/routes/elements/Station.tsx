import { highwayWidth, cellLength } from 'rri-ai/constants'

const h = cellLength / 2
const w = highwayWidth

export default function Station() {
    return (
        <rect
            height={w} width={w}
            transform={`translate(${h-w/2}, ${h-w/2})`}
            fill="currentColor"
        />
    )
}
