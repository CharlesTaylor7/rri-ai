import type { ChangeEventHandler } from 'react'
import { useRef, useId, useCallback } from 'react'

type Props = {
  className: string
  label?: string
  checked: boolean
  onChange?: (checked: boolean) => void
  readOnly: boolean
}

Numeric.defaultProps = {
  className: '',
  checked: false,
  readOnly: false,
  onChange: (checked: boolean) => console.log(`Click checkbox: ${checked}`),
}

export default function Numeric(props: Props) {
  const inputRef = useRef<HTMLInputElement | null>(null)
  const inputId = useId()
  const onChange: ChangeEventHandler<HTMLInputElement> = useCallback(
    // @ts-ignore
    (event) => props.onChange(event.target.checked),
    [props.onChange],
  )

  return (
    <span className="inline-flex items-center">
      {props.label ? (
        <label className="mr-3" htmlFor={inputId}>
          {props.label}:
        </label>
      ) : undefined}
      <input
        className={`cursor-pointer ${props.className}`}
        id={inputId}
        ref={inputRef}
        type="checkbox"
        checked={props.checked}
        onChange={onChange}
        readOnly={props.readOnly}
      />
    </span>
  )
}
