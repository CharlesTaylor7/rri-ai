import { highwayWidth, hatchLocation, cellLength, hwyDashPattern } from 'rri-ai/constants'

const h = cellLength / 2;
const w = highwayWidth / 2;

// TODO: use bezier curve / parabola to get a more rounded edge
export default function HighwayInsideTurn(props) {
    const { rotate, ...rest } = props
    return (
        <polyline
            points={`0,${h-w} ${h-2*w},${h-w} ${h-w},${h-2*w} ${h-w},0`}
            transform={`rotate(${rotate*90},${h},${h})`}
            {...rest}
        />
    )
}
HighwayInsideTurn.defaultProps = {
    rotate: 0
}
