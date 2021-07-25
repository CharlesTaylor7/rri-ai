export default {
    'load_state': (state, { state: toLoad }) => ({...state, ...toLoad})
}
