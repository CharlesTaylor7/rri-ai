import {initializeStore} from "../next";
import reducer from "./reducer";
import middleware from "./middleware";

const store = initializeStore({ reducer });
epicMiddleware.run(rootEpic);

export default store


export type RootState = ReturnType<typeof store.getState>
export type AppDispatch = typeof store.dispatch
