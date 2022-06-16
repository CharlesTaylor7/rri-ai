import { createContext, useContext } from 'react'

export interface AppContext<T> {
  state: T
  pushState: (updates: Partial<T>) => void
}

const context = createContext<undefined | AppContext<any>>(undefined)

export const Provider = context.Provider
export function useSelector<S, V>(fn: (state: S) => V): V {
  const value = useContext(context)
  if (value === undefined) {
    throw new Error("Used 'useSelector' outside of provider context")
  }
  return fn(value.state)
}

export function useDispatch() {
  const value = useContext(context)
  if (value === undefined) {
    throw new Error("Used 'useDispatch' outside of provider context")
  }
  return value.pushState
}
