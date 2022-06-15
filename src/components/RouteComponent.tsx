import React from 'react'

type Component = () => React.ReactElement
type Props = React.SVGProps<SVGGElement>

export default function RouteComponent(component: Component): React.FC<Props> {
  const wrapped: React.FC<Props> = (props) => (
    <g {...props}>{React.createElement(component)}</g>
  )
  wrapped.displayName = (component as any).displayName
  return wrapped
}

RouteComponent.defaultProps = {
  stroke: 'black',
  strokeLinejoin: 'round',
  fill: 'none',
}
