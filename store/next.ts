import { useMemo } from 'react'
import { configureStore, ConfigureStoreOptions } from 'redux-toolkit'
import type { Store, Action as GenericAction } from 'redux'

export type Action = GenericAction<string>


let store: any = undefined
export function initializeStore<S, A extends Action>(config: ConfigureStoreOptions<S, A>): Store<S, A> {
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


function initStore<S, A extends Action>(config: ConfigureStoreOptions<S, A>): Store<S, A> {
    return configureStore({
        devTools: process.env.NODE_ENV !== 'production',
        ...config,
    })
}


