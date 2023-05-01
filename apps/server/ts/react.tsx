import { QueryClient } from "@tanstack/react-query";
import { FetchTransport, createClient } from "@rspc/client";
import { createReactQueryHooks } from "@rspc/react";
 
import type { Procedures } from "./bindings"; // These were the bindings exported from your Rust code!
import type { FC, PropsWithChildren, ReactElement } from "react";
 
// You must provide the generated types as a generic and create a transport (in this example we are using HTTP Fetch) so that the client knows how to communicate with your API.
const client = createClient<Procedures>({
  // Refer to the integration your using for the correct transport.
  transport: new FetchTransport("http://localhost:8080/rspc"),
});

const queryClient = new QueryClient();
 
export const RSPCProvider: FC<PropsWithChildren> = ({ children }) => (
    <rspc.Provider client={client} queryClient={queryClient}>
      {children as ReactElement}
    </rspc.Provider>
)

export const rspc = createReactQueryHooks<Procedures>();
