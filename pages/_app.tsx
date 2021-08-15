import 'rri-ai/styles/globals.css'
import { Provider } from 'react-redux'
import { useEffect } from 'react'
import Head from 'next/head'
import Error from 'next/error'
import type { AppProps } from 'next/app'
import { useRouter } from 'next/router'
import { useStore } from 'hooks/useStore'


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
    let component = <Component {...rest} />
    if (storeConfig) {
        const store = useStore(storeConfig)
        component = <Provider store={store}>component</Provider>
    }

    return (
        <>
            <Head>
                <title>Railroad Inc. AI</title>
                <link rel="icon" href="/favicon.ico" />
            </Head>
            <main>
                {component}
            </main>
        </>
    )
}
