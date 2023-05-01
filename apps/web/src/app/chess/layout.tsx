import type { FC, PropsWithChildren } from "react";

const layout: FC<PropsWithChildren> = ({ children }) => (
  <div className="">
    {children}
  </div>
);

export default layout;
