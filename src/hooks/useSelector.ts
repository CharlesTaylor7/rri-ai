import { useSelector as useUntypedSelector } from 'app/context'
import { AppState } from 'app/types'

export default function useSelector<V>(fn: (state: AppState) => V): V {
  return useUntypedSelector(fn)
}
