import type { AppProps } from "next/app";
import Head from "next/head";
import {useCallback, useState} from "react";
import Error from "components/Error";
import { Provider } from 'app/context'
import {GameState} from "app/server/state";

export default function App({ Component, pageProps }: AppProps) {
  console.log(pageProps)
  const { error, state: initialState, ...rest } = pageProps;
  if (error) {
    return <Error {...error} />;
  }

  const [ state, setState] = useState<GameState>(initialState)
  const pushState = useCallback((updates: Partial<GameState>) => setState(state => ({...state, ...updates})), [setState])

  return (
    <>
      <Head>
        <title>Railroad Inc. AI</title>
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <main>
        <Provider value={{state, pushState}}>
          <Component {...rest} />
        </Provider>
      </main>
    </>
  );
}
