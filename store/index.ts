import { createContext, useReducer, createElement } from 'react'

// A store is global state for some part of the application
// It exposes the current state, and the dispatch method for updating that state
export type Store<S, A> = {
    state: S,
    dispatch: (action: A) => void,
}


// A reducer is a transformation of the state S by an action A
export type Reducer<S, A> = (S, A) => S
export interface ReducerMap<S, A> {
    [actionType: A]: (S) => S;
}


/**
 * Create your own redux-lite store from just an initial state and a reducer function
 * The root component of the store should be wrapped by <StoreProvider />
 * The subscriber components only need to import the context and use it like so:
 * `const { state, dispatch } = useContext(myPageContext)`
 */
export default function createStore<S, A>(
    initialState: S,
    reducerMap: ReducerMap<S, A>
): Store<S, A> {

    // context with the state, subscriber components reference this
    const Context = createContext(initialState)

    // provider for the root component
    const Provider = ({ children }) => {
        const reducer = useCallback((state, action) => reducerMap[action](state))
        const [ state, dispatch ] = useReducer(reducer, initialState)

        const store = { state, dispatch }
        return createElement(
            Context.Provider,
            { value: store },
            ...children
        )
    }

    return { Context, Provider }
}
