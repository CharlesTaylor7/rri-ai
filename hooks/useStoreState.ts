import { useSelector, shallowEqual } from 'react-redux'
import type { RootState } from 'store/core/config';


export function useState<K extends keyof RootState>(key: K): RootState[K] {
    return useSelector(state => state[key], shallowEqual)
}
