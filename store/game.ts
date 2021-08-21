import { useStore } from 'hooks/useStore'
import { RouteInfo } from 'rri-ai/types'
import { Reducer } from 'redux'


export type GameState = {
    routes: {
        current: Array<RouteInfo>,
        pending: Array<RouteInfo>
    },
    diceCodes: Array<number>,
}

export const preloadedState = {
    routes: {
        current: [],
        pending: [],
    },
    diceCodes: []
}

export const reducer = {
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
}
