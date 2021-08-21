import { configureStore, ConfigureStoreOptions } from 'redux-toolkit'
import type { Store, Action as GenericAction } from 'redux'

export type RootAction = GenericAction<string>;
export type RootState = {};
export const rootReducer = <S,A>(s: S, _a: A) => s;

const rootConfig: ConfigureStoreOptions<RootState, RootAction> =  {
    reducer: rootReducer,
}

export default rootConfig;
