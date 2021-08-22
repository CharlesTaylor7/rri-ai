import { useSelector as useUntypedSelector, TypedUseSelectorHook } from 'react-redux'
import type { RootState } from 'store/core/index';

const useSelector: TypedUseSelectorHook<RootState> = useUntypedSelector;
export default useSelector;
