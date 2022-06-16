import type { AppProps } from 'next/app'
import Head from 'next/head'

import 'app/styles/tailwind.css'
import Error from 'app/components/Error'

export default function App({ Component, pageProps }: AppProps) {
  const { error, ...rest } = pageProps
  if (error) {
    return <Error {...error} />
  }
  return (
    <>
      <Head>
        <title>Railroad Inc. AI</title>
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <main>
        <Component {...rest} />
      </main>
    </>
  )
}