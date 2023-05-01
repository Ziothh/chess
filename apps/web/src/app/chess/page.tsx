import type { FC } from "react";
import Chess from "./Chess";
import { ChessCtxProvider } from "./ChessContext";

const page: FC = ({ }) => {

  return (
    <div className="">
      <ChessCtxProvider>
        <Chess />
      </ChessCtxProvider>
    </div>
  )
};

export default page;
