import {initializeStore} from "../next";
import reducer from "./reducer";
import { epicMiddleware } from "./middleware";
import rootEpic from "./epic";
import {getDefaultMiddleware} from "redux-toolkit";


const store = initializeStore({
    reducer: reducer as any,
    middleware: (getDefaultMiddleware) =>
        getDefaultMiddleware()
            .concat(
                epicMiddleware,
            ),
});

epicMiddleware.run(rootEpic);

export default store


export type AppDispatch = typeof store.dispatch
