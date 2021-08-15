import { useSelector, shallowEqual } from 'react-redux'

export function useState(key: keyof): object {
    return useSelector(state => state[key], shallowEqual)
}
