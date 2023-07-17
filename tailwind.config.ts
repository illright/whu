import { Config } from "tailwindcss";

const config: Config = {
  content: ["./src/**/*.{svelte,ts}"],
  theme: {
    fontFamily: {
      sans: ["Work Sans", "sans-serif"],
      mono: ["Fira Code", "monospace"],
    },
    fontSize: {
      xs: ["0.75rem", "1.125rem"], // 12px 18px
      sm: ["0.875rem", "1.5rem"], // 14px 24px
      base: ["1rem", "1.5rem"], // 16px 24px
      lg: ["1.25rem", "1.5rem"], // 20px 24px
      xl: "2.5rem", // 40px
    },
    extend: {
      letterSpacing: {
        individual: "0.2em",
      },
      colors: {
        border: "hsl(var(--border))",
        input: "hsl(var(--input))",
        ring: "hsl(var(--ring))",
        background: "hsl(var(--background))",
        foreground: "hsl(var(--foreground))",
        primary: {
          DEFAULT: "hsl(var(--primary))",
          foreground: "hsl(var(--primary-foreground))"
        },
        secondary: {
          DEFAULT: "hsl(var(--secondary))",
          foreground: "hsl(var(--secondary-foreground))"
        },
        destructive: {
          DEFAULT: "hsl(var(--destructive))",
          foreground: "hsl(var(--destructive-foreground))"
        },
        muted: {
          DEFAULT: "hsl(var(--muted))",
          foreground: "hsl(var(--muted-foreground))"
        },
        accent: {
          DEFAULT: "hsl(var(--accent))",
          foreground: "hsl(var(--accent-foreground))"
        },
        popover: {
          DEFAULT: "hsl(var(--popover))",
          foreground: "hsl(var(--popover-foreground))"
        },
        card: {
          DEFAULT: "hsl(var(--card))",
          foreground: "hsl(var(--card-foreground))"
        }
      },
    },
  },
  plugins: [],
};

export default config;
