import {createContext, useContext} from "react";
import {GameState} from "app/server/state";


interface AppContext {
  state: GameState, 
  pushState: (updates: Partial<GameState>) => void
}


const context = createContext<undefined | AppContext>(undefined)

export const Provider = context.Provider
export function useSelector(fn: Function) {
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
