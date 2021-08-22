import {initializeStore} from "../next";
import reducer from "./reducer";

const store = initializeStore({ reducer });
export default store


export type RootState = ReturnType<typeof store.getState>
export type AppDispatch = typeof store.dispatch
