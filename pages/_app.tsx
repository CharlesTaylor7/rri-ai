import 'app/styles/globals.css'
import { Provider } from 'react-redux'
import { useEffect, useMemo } from 'react'
import Head from 'next/head'
import Error from 'next/error'
import type { AppProps } from 'next/app'
import { useRouter } from 'next/router'
import {initializeStore} from 'store/next'

const defaultStoreConfig = {
    reducer: (s: any, _a: any) => s,
}

export default function App({ Component, pageProps }: AppProps) {
    const { error, storeConfig, ...rest } = pageProps

    const router = useRouter()

    useEffect(() => {
        // redirect to home page after 400ms second
        if (error) setTimeout(() => router.push('/'), 400)
    }, [])

    if (error) {
        return ( <Error {...error} />)
    }

    const store = useMemo(
        () => initializeStore(storeConfig || defaultStoreConfig),
        [storeConfig]
    ) as any;

    return (
        <>
            <Head>
                <title>Railroad Inc. AI</title>
                <link rel="icon" href="/favicon.ico" />
            </Head>
            <main>
                <Provider store={store}>
                    <Component {...rest} />
                </Provider>
            </main>
        </>
    )
}
