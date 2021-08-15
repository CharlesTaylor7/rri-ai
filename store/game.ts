import { useStore } from '@/store'
import coreActions from '@/store/core/actions'
import {RouteInfo} from '@/types'


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

export default function useGameStore() {
}
createStore(defaultState, gameActions)
