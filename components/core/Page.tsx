import Head from 'next/head'
import Error from 'next/error'
import { useRouter } from 'next/router'
import { useEffect } from 'react'

export default function Page(props) {
    const { error } = props
    const router = useRouter()
    useEffect(() => {
        // redirect to home page after 400ms second
        if (error) setTimeout(() => router.push('/'), 400)
    }, [])

    if (error) {
        return ( <Error {...error} />)
    }
    const {children, store, ...rest} = props

    // if there is a store for this page, then wrap around the children
    const contents = store
        ? (<store.Provider>{children}</store.Provider>)
        : (<>{children}</>)

    return (
        <>
            <Head>
                <title>Railroad Inc. AI</title>
                <link rel="icon" href="/favicon.ico" />
            </Head>
            <main {...rest}>
                {contents}
            </main>
        </>
    )
}
