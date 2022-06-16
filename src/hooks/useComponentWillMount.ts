import { useRef } from 'react'

export default function useComponentWillMount<T>(callback: () => T): T {
  // initialize the ref
  const ref = useRef<T | undefined>()

  // run the callback before mounting
  if (ref.current === undefined) {
    ref.current = callback()
  }
  return ref.current
}
