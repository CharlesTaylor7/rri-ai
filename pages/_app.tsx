import '@/styles/globals.css'
import { Provider } from 'react-redux'
import { useEffect } from 'react'
import Head from 'next/head'
import Error from 'next/error'
import type { AppProps } from 'next/app'
import { useRouter } from 'next/router'
import { useStore } from '@/store'


export default function App({ Component, pageProps }: AppProps) {
    const { error, initialReduxState, ...rest } = pageProps

    const router = useRouter()
    const store = useStore(initialReduxState)

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
                    <Component {...rest} />
                </Provider>
            </main>
        </>
    )
}
