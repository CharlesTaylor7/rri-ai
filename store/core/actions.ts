export default {
    'load_state': ({ state: toLoad }) => ({ state }) => ({...state, ...toLoad})
}
