import 'app/styles/globals.css'
import type { AppProps } from 'next/app'
import type {RootState} from 'app/store/core/index'
import { Provider, useDispatch } from 'react-redux'
import { useEffect } from 'react'
import Head from 'next/head'
import Error from 'next/error'
import { useRouter } from 'next/router'
import store from 'app/store/core/index'


export default function App({ Component, pageProps }: AppProps) {
    const { error, state, ...rest } = pageProps

    const router = useRouter()

    useEffect(() => {
        // redirect to home page after 400ms second
        if (error) setTimeout(() => router.push('/'), 400)
    }, [])

    if (error) {
        return ( <Error {...error} />)
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

    useEffect(() => dispatch({type: 'load_state', state} as any), [state])
}

const LoadState: React.FC<{ state: RootState }> = ({ children, state }) => {
    useInitialState(state)

    return <>{children}</>
}
