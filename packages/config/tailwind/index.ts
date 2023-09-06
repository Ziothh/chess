import type { Config } from "tailwindcss";

export default {
  content: [""],
  theme: {
    extend: {
      zIndex: {
        'modal': '900',
      }
    },
  },
  plugins: [],
} satisfies Config;
