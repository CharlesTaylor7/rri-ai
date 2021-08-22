import React from "React";
import type { ReactSVGElement, SVGProps} from "react";

interface RouteProps {
    id: string,
}

export default function RouteComponent(component: ReactSVGElement) {
    const wrapped = (props: SVGProps<SVGGElement>) => (
        <g {...props}>
            {component}
        </g>
    );
    wrapped.displayName = component.displayName;
    return wrapped;
}

RouteComponent.defaultProps = {
    'stroke': 'black',
    'strokeLinejoin': 'round',
    'fill': 'none',
}
