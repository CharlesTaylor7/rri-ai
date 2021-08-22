import { createEpicMiddleware } from 'redux-observable';

export const epicMiddleware = createEpicMiddleware();

export default [
    epicMiddleware,
]
