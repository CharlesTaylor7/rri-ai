import {combineEpics} from "redux-observable";
import { rollDiceEpic } from 'store/game/epics';

export default combineEpics({
    rollDiceEpic,
})
