import { useSelector as useUntypedSelector, TypedUseSelectorHook } from 'react-redux'
import type { RootState } from 'store/core/reducer';

const useSelector: TypedUseSelectorHook<RootState> = useUntypedSelector;
export default useSelector;
