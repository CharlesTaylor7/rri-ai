import {Reducer} from 'react';
import { combineReducers } from 'redux'
import { reducer as gameReducer } from 'store/game'

export type RootAction = any;
export type RootState = ReturnType<typeof combinedReducer>;

const combinedReducer = combineReducers({
    game: gameReducer,
})


const rootReducer: Reducer<RootState, RootAction> = (state, action) => {
    console.log(action.type)
    switch (action.type) {
        case 'load_state':
            return ({...state, ...action.state})
        default:
            return combinedReducer(state, action)
    }
}

export default rootReducer
