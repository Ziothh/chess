"use client"

import { RSPCProvider } from "@acme/server";
import type { FC, PropsWithChildren } from "react";

const GlobalContext: FC<PropsWithChildren> = ({ children }) => (
  <RSPCProvider>
    {children}
  </RSPCProvider>
);

export default GlobalContext;
