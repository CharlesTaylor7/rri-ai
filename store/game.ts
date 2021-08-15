import { useStore, StoreConfig } from 'hooks/useStore'
import coreActions from 'rri-ai/store/core/actions'
import {RouteInfo} from 'rri-ai/types'


export type GameState = {
    routes: {
        current: Array<RouteInfo>,
        pending: Array<RouteInfo>
    },
    diceCodes: Array<number>,
}

const defaultState = {
    routes: {
        current: [],
        pending: [],
    },
    diceCodes: []
}

const gameActions = {
    'show_move': (state: GameState) => ({
        ...state,
        routes: {
            current: [
                ...state.routes.current,
                ...state.routes.pending,
            ],
            pending: [],
        }
    }),
    'roll_dice': (state: GameState) => {
        return state
    },
    //     const url = `/api/game/roll/?id=${props.id}`
    //     const { diceCodes, nextRoutes } = await fetch(url).then(res => res.json())
    //     setDiceCodes(diceCodes)
    //     setRoutes(rs => ({
    //         pending: nextRoutes,
    //         current: rs.current,
    //     }))
    ...coreActions,
}
type ActionMap<S, A extends Action> = {
    [actionType: string]: Reducer<S, A>,
}

function actionsMapToReducer<S, A extends Action>(actions: ActionMap<S, A>): Reducer<S, A> {
    return (state: S, action: A) => actions[action.type](state, action)
}


export const GameStoreConfig: StoreConfig<GameState> = {
    reducer: actionsMapToReducer(gameActions),
    initialState: defaultState,
}

export function useGameStore() {
    useStore(GameStoreConfig)
}
