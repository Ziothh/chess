"use client";

import { FC, PropsWithChildren, createContext, useContext } from 'react'

export const contextFactory = <
  T extends (arg?: any) => any,
  P = Parameters<T>[0],
>(
  useValue: T,
  ContextWrapper: FC<PropsWithChildren> = ({ children }) => <>{children}</>,
): [
    useCtx: () => ReturnType<T>,
    CtxProvider: FC<PropsWithChildren<
      Exclude<P, undefined> extends never
      ? {}
      : {
        ctx: Parameters<T>[0]
      }
    >>
  ] => {
  const ctx = createContext<T>(null!)

  return [
    // @ts-ignore
    () => useContext(ctx),
    (props) => (
      <ContextWrapper>
        <ctx.Provider value={'ctx' in props ? useValue(props.ctx) : useValue()}>{props.children}</ctx.Provider>
      </ContextWrapper>
    )
  ]
}

/** Creates a new component that wraps the given `Component` with the `WrapperComponent` */
export const withWrapper = <WP, P>(
  WrapperComponent: FC<PropsWithChildren<WP>>,
  Component: FC<P>
): FC<
  unknown extends (WP & P) ? {} : (WP & P)
> => (props) => <WrapperComponent
  {...props as any}
  children={<Component {...props as any} />}
/>
