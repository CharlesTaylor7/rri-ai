import type { MouseEventHandler, ReactNode } from 'react'

type Props = {
  className?: string
  activeClass?: string
  onClick: MouseEventHandler
  disabled?: boolean
  children?: ReactNode
}

Button.defaultProps = {
  onClick: () => console.log('TODO: implement onClick'),
}

export const iconButtonStyle = (className: string = '') =>
  `px-3 py-1 ${className}`
export const labelButtonStyle = (className: string = '') => `p-2 ${className}`

export default function Button(props: Props) {
  if (props.activeClass !== undefined) {
    console.warn(
      'Deprecated: <Button> uses "className" instead of "activeClass"',
    )
  }
  return (
    <button
      className={`
      rounded-lg disabled:bg-slate-200
      ${props.activeClass || ''}
      ${props.className}
    `}
      type="button"
      onClick={props.onClick}
      disabled={props.disabled}
    >
      {props.children}
    </button>
  )
}
