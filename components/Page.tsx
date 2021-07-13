import Head from 'next/head'

export default function Page(props) {
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
