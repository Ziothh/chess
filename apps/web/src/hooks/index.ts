import { useState } from "react"

export const useStateObject = <T,>(defaultValue: T) => {
  const [state, setState] = useState(defaultValue);
  return {
    value: state,
    set: setState,
  };
};
