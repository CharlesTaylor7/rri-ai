import 'app/styles/globals.css'
import type { AppProps } from 'next/app'
import type {RootState} from 'app/store/core/reducer'
import { Provider, useDispatch } from 'react-redux'
import { useEffect } from 'react'
import Head from 'next/head'
import store from 'app/store/core/index'
import Error from 'components/Error'


export default function App({ Component, pageProps }: AppProps) {
    const { error, state, ...rest } = pageProps
    if (error) {
        return (<Error {...error} />)
    }

    return (
        <>
            <Head>
                <title>Railroad Inc. AI</title>
                <link rel="icon" href="/favicon.ico" />
            </Head>
            <main>
                <Provider store={store}>
                    <LoadState state={state}>
                        <Component {...rest} />
                    </LoadState>
                </Provider>
            </main>
        </>
    )
}

function useInitialState(state: RootState) {
    const dispatch = useDispatch();

    useEffect(
        () => {
            if (state !== undefined) {
                dispatch({type: 'load_state', state} as any)
            }
        },
        [state]
    )
}

const LoadState: React.FC<{ state: RootState }> = ({ children, state }) => {
    useInitialState(state)

    return <>{children}</>
}
