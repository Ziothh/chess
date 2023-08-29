"use client";

import { useEffect } from "react";
import { contextFactory } from "~/utils/components";

export namespace EventHistory {
  type EventMetaData = {
    timeStamp: Date;
  }
  export type Event<Type extends string = string, Data = unknown> = EventMetaData & {
    type: Type;
    data: Data;
  };

  type getEventByType<Events extends Event, Type extends Events['type']> = Extract<Events, EventHistory.Event<Type, any>>;

  type EventCallback<
    Events extends Event,
    Type extends Events['type']
  > = (data: getEventByType<Events, Type>['data'], meta: EventMetaData) => void | Promise<void>

  const DEFAULT_DEPENDENCIES: any[] = [];

  export const createStore = <Events extends Record<EventHistory.Event['type'], EventHistory.Event['data']>>() => {
    type T = { [E in keyof Events]: EventHistory.Event<Extract<E, string>, Events[E]> }[keyof Events];

    const history: T[] = [];

    const listeners: { [E in T['type']]: Set<EventCallback<T, E>> } = {} as any;

    return {
      history,
      /** Adds an event listener to the `EventHistory` store */
      on<E extends T['type']>(
        event: E,
        callback: EventCallback<T, E>,
        listenerDependencies: any[] = DEFAULT_DEPENDENCIES
      ) {
        useEffect(() => {
          if (!(event in listeners)) {
            // @ts-ignore
            listeners[event] = new Set<EventCallback<E, T>>()
          }

          listeners[event].add(callback as any);

          return () => { listeners[event].delete(callback) }
        }, listenerDependencies);
      },
      /** Emits an event to the `EventHistory` store. 
        * Returns `true` if the event has listeners, `false` if there are none */
      emit<E extends T['type']>(
        event: E, 
        data: unknown extends getEventByType<T, E>['data']
        ? null
        : getEventByType<T, E>['data']
      ): boolean {
        if (!(event in listeners)) return false;

        const eventMeta: EventMetaData = {
          timeStamp: new Date(Date.now()),
        }

        history.push({
          ...eventMeta,
          type: event,
          data,
          // @ts-expect-error
        } satisfies T as any)

        listeners[event].forEach(callback => callback(data, eventMeta))

        return true;
      }
    } as const;
  }
}
