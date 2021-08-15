import { useMemo } from 'react'
import { configureStore, ConfigureStoreOptions } from 'redux-toolkit'
import type { Store, Action as GenericAction } from 'redux'

export type Action = GenericAction<string>

const defaultOptions: ConfigureStoreOptions<any, any> = { reducer: {} }

export function useStore<S, A extends Action>(storeConfig: ConfigureStoreOptions<S, A> = defaultOptions): Store<S, A> {
    console.log(storeConfig)
    return useMemo(() => initializeStore<S, A>(storeConfig), [storeConfig])
}


function initStore<S, A extends Action>(config: ConfigureStoreOptions<S, A>): Store<S, A> {
    return configureStore({
        devTools: process.env.NODE_ENV !== 'production',
        ...config,
    })
}


let store: any = undefined
function initializeStore<S, A extends Action>(config: ConfigureStoreOptions<S, A>): Store<S, A> {
    // For SSG and SSR always create a new store
    if (typeof window === 'undefined') return initStore(config)

    // Merge with an existing store, or recreate fresh
    store = store
        ? initStore({
            ...config,
            preloadedState: {
                ...config.preloadedState,
                ...store.getState(),
            }
        })
        : initStore(config)

    return store
}
