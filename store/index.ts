import { useMemo } from 'react'
import { createStore, applyMiddleware } from 'redux'
import { composeWithDevTools } from 'redux-devtools-extension'


type StoreConfig<S extends object> = {
    tag: string,
    initialState: S,
    reducer: Reducer<S>,
}

type Action = {type: string}
type Reducer<S, A extends Action = Action> = (state: S, action: A) => S


function initStore<S extends object>(config: StoreConfig<S>): Store<S> {
    return createStore(
        config.reducer,
        config.initialState,
        composeWithDevTools(applyMiddleware())
    )
}

type Store<S extends object> = {
    getState(): S,
}

let stores: {
    [tag: string]: Store<any>,
} = {}

function initializeStore<S extends object>(config: StoreConfig<S>) {
    // For SSG and SSR always create a new store
    if (typeof window === 'undefined') return initStore(config)

    // fetch the existing store
    let store = stores[config.tag]

    // Merge with an existing store, or recreate fresh
    store = store
        ? initStore({
            ...config,
            initialState: {
                ...config.initialState,
                ...store.getState(),
            }
        })
        : initStore(config)


    // save the store
    stores[config.tag] = store

    return store
}


export function useStore<S extends object>(config: StoreConfig<S>): Store<S> {
    return useMemo(() => initializeStore(config), [config])
}
