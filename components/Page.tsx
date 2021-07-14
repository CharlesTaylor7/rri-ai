import Head from 'next/head'
import Error from 'next/error'
import { useRouter } from 'next/router'
import { useEffect } from 'react'

export default function Page(props) {
    const { error } = props
    const router = useRouter()
    useEffect(() => {
        // redirect to home page after 1 second
        if (error) setTimeout(() => router.push('/'), 1000)
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
            {props.children}
        </main>
        </>
    )
}
