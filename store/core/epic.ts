import {combineEpics} from 'redux-observable';
import { rollDiceEpic } from 'store/game/actions';

export default combineEpics(
    rollDiceEpic as any,
)
