import { createContext, createElement, useCallback, useReducer } from 'react'

// A store is global state for some part of the application
// It exposes the current state, and the dispatch method for updating that state
export type Store<S, A> = {
    state: S,
    dispatch: (action: A) => void,
}


// A reducer is a transformation of the state S by an action A
export type Reducer<S, A> = (S, A) => S
export interface Action {
    type: string;
}

export interface ActionMap<S, A extends Action> {
    [actionType: string]: (A) => (S) => S;
}


/**
 * Create your own redux-lite store from just an initial state and a reducer function
 * The root component of the store should be wrapped by <Provider />
 * The other components should import useStore
 * `const { state, dispatch } = useStore()
 */
export default function createStore<S, A>(
    initialState: S,
    reducerMap: ReducerMap<S, A>
): Store<S, A> {

    // context with the state, subscriber components reference this
    const Context = createContext(initialState)

    // provider for the root component
    const Provider = ({ children }) => {
        const reducer = useCallback((state, action) => reducerMap[action.type](action)(state));
        const [ state, dispatch ] = useReducer(reducer, initialState)

        const store = { state, dispatch }
        return createElement(
            Context.Provider,
            { value: store },
            ...children
        )
    }

    const useStore = () => useContext(Context)

    return { useStore, Provider }
}

export function connect(useProps, component) {
    // props from parent take higher precedence than the hook
    return (props) => {
        const hookProps = useProps();
        return createElement(component, { ...hookProps, ...props })
    }
}
