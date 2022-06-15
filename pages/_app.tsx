import "app/styles/globals.css";
import type { AppProps } from "next/app";
import type { RootState } from "app/store/core/reducer";
import { Provider, useDispatch } from "react-redux";
import { useEffect } from "react";
import Head from "next/head";
import store from "app/store/core/index";
import Error from "components/Error";

export default function App({ Component, pageProps }: AppProps) {
  console.log(pageProps)
  const { error, state, ...rest } = pageProps;
  if (error) {
    return <Error {...error} />;
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
  );
}
