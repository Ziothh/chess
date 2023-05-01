import "../styles/globals.css";
import type { AppType } from "next/app";
import type { Session } from "next-auth";
// import { SessionProvider } from "next-auth/react";
import { RSPCProvider } from '@acme/server'

const MyApp: AppType<{ session: Session | null }> = ({
  Component,
  pageProps: { session, ...pageProps },
}) => {
  return (
    // <SessionProvider session={session}>
    <RSPCProvider>
      <Component {...pageProps} />
    </RSPCProvider>
    // </SessionProvider>
  );
};

export default MyApp;
