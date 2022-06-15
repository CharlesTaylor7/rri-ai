import type { ChangeEventHandler } from 'react'
import { useEffect, useRef, useId, useCallback } from 'react'
import { useDebouncedCallback } from 'use-debounce'

type Props = {
  className: string
  label?: string
  value?: number
  onChange: (value: number) => void
  min?: number
  max?: number
}

NumberInput.defaultProps = {
  className: '',
}

function clamp(value: number, min?: number, max?: number) {
  if (min !== undefined) value = Math.max(min, value)
  if (max !== undefined) value = Math.min(max, value)
  return value
}

export default function NumberInput(props: Props) {
  const inputRef = useRef<HTMLInputElement | null>(null)
  const inputId = useId()
  const onChange: ChangeEventHandler<HTMLInputElement> = useCallback(
    (event) =>
      props.onChange(clamp(Number(event.target.value), props.min, props.max)),
    [props.onChange],
  )
  const onChangeDebounced = useDebouncedCallback(onChange, 1_000)

  useEffect(() => {
    if (inputRef.current) {
      inputRef.current.value = String(props.value)
    }
  }, [props.value])
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
        type="number"
        defaultValue={props.value}
        onChange={onChangeDebounced}
        onBlur={onChange}
        min={props.min}
        max={props.max}
      />
    </span>
  )
}
