// import React from "React";
import React from "react";


type Component = () => React.ReactElement;

export default function RouteComponent(component: Component) {
    const wrapped: React.FC = (props: any) => (
        <g {...props}>
            {component}
        </g>
    );
    wrapped.displayName = (component as any).displayName;
    return wrapped;
}

RouteComponent.defaultProps = {
    'stroke': 'black',
    'strokeLinejoin': 'round',
    'fill': 'none',
}
