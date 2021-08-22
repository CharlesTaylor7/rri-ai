import {applyMiddleware} from 'redux';
import { createEpicMiddleware } from 'redux-observable';

export const epicMiddleware = createEpicMiddleware();

export default [
    applyMiddleware(epicMiddleware),
]
