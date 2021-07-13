import Head from 'next/head'
import Error from 'next/error'

export default function Page(props) {
    const { error } = props
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
