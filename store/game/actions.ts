import {Epic, ofType} from 'redux-observable';
import {map, mergeMap} from 'rxjs';
import { ajax } from 'rxjs/ajax';


 // action creators
export const rollDice = (gameId: string) => ({ type: 'roll_dice', gameId })
export const rollDiceFulfilled = (payload: object) => ({ type: 'roll_dice_fulfilled', payload });

 // epic
export const rollDiceEpic: Epic = action$ => action$.pipe(
    ofType('roll_dice'),
    mergeMap(({ gameId }) =>
         ajax.getJSON(`/pages/api/game/roll`).pipe(
             map((response: any) => rollDiceFulfilled(response))
        )
    )
);
