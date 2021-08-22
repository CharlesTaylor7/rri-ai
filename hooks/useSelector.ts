import { useSelector as useSel, shallowEqual } from 'react-redux'
import type { RootState } from 'store/core/config';

type Selector<T> = (s: RootState) => T
export default function useSelector<T>(s: Selector<T>): T {
    return useSel(s, shallowEqual)
}
