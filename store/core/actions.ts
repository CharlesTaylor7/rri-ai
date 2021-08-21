const coreReducer = {
    'load_state': <S> (state: S, { state: toLoad }: { state: S }): S => ({...state, ...toLoad})
}

export default coreReducer;
