import {initializeStore} from "../next";
import reducer from "./reducer";
import middleware, {epicMiddleware} from "./middleware";
import rootEpic from "./epic";


const store = initializeStore({ reducer, middleware });
epicMiddleware.run(rootEpic);

export default store


export type RootState = ReturnType<typeof store.getState>
export type AppDispatch = typeof store.dispatch
