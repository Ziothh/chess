import type { FC, PropsWithChildren } from 'react'
import GlobalContext from './GlobalContext';

import "../styles/globals.css";

const layout: FC<PropsWithChildren> = ({ children }) => (
  <html className='h-screen overflow-y-auto overflow-x-hidden'>
    <head></head>
    <body className='bg-black text-white h-full'>
      <GlobalContext>
        {children}
      </GlobalContext>
    </body>
  </html>
);

export default layout;
