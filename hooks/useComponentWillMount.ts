import { useRef } from 'react'

export default function useComponentWillMount(callback: () => void) {
  // initialize the ref
  const willMountRef = useRef(true)

  // run the callback before mounting
  if (willMountRef.current) callback()

  // turn off ref flag, so that the callback doesn't run again
  willMountRef.current = false
}
