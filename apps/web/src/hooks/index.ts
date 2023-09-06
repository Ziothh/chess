'use client';

import { useEffect, useState } from "react"

export const useStateObject = <T,>(defaultValue: T) => {
  const [state, setState] = useState(defaultValue);
  return {
    value: state,
    set: setState,
  };
};

export const useKeyPress = (keyMap: Record<string, () => void>) => {
  // Todo: add support for modifier keys
  useEffect(() => {
    const listener = (e: KeyboardEvent) => {
      keyMap[e.key]?.();
    }

    window.addEventListener('keyup', listener);

    return () => window.removeEventListener('keyup', listener);
  }, [keyMap]);
}
