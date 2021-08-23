import { RouteInfo } from 'app/types'
import { Action, Reducer } from 'redux'


export interface GameState {
    id: string,
    routes: {
        current: Array<RouteInfo>,
        pending: Array<RouteInfo>
    },
    diceCodes: Array<number>,
}

export const preloadedState = {
    id: '',
    routes: {
        current: [],
        pending: [],
    },
    diceCodes: []
}

export const reducer: Reducer<GameState, Action<string>> = (state = preloadedState, action) => {
    switch (action.type) {
        case 'show_move':
            return ({
                ...state,
                routes: {
                    current: [
                        ...state.routes.current,
                        ...state.routes.pending,
                    ],
                    pending: [],
                }
            })
        case 'roll_dice':
            // TODO: show loading indicator
            return state
            //     const url = `/api/game/roll/?id=${props.id}`
            //     const { diceCodes, nextRoutes } = await fetch(url).then(res => res.json())
            //     setDiceCodes(diceCodes)
            //     setRoutes(rs => ({
            //         pending: nextRoutes,
            //         current: rs.current,
            //     }))
            //
        case 'roll_dice_fulfilled':

        default:
            return state
    }
}
