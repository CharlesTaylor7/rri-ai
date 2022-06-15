import { useCallback, useState } from 'react'
import type { AppContext } from 'app/context'

export default function useErgonomicState<T>(initialState: T): AppContext<T> {
  const [state, setState] = useState<T>(initialState)
  const pushState = useCallback(
    (updates: Partial<T>) => setState((state) => ({ ...state, ...updates })),
    [setState],
  )

  return { state, pushState }
}
