import { createContext } from 'react'
import createStore from '@/store'
import coreActions from '@/store/core/actions'


export type GameState = {
    routes: {
        current: Array<RouteInfo>,
        pending: Array<RouteInfo>
    },
    diceCodes: Array<number>,
}

const initialState = {
    routes: {
        current: [],
        pending: [],
    },
    diceCodes: []
}

const gameActions = {
    'show_move': () => (state) => ({
        ...state,
        routes: {
            current: [...routes.current, ...routes.pending],
            pending: [],
        }
    }),
    'roll_dice': () => (state) => {
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

export default createStore(initialState, gameActions);
