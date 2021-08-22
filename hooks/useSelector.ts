import { useSelector as useSel, shallowEqual, TypedUseSelectorHook } from 'react-redux'
import type { RootState } from 'store/core/config';

const useSelector: TypedUseSelectorHook<RootState> = useSel;
export default useSelector;
