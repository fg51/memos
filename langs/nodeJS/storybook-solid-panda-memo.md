# storybook with solid.js, panda.css

## for pandacss

```sh
$ pnpm dlx storybook@latest init
```

at panda.config.ts

```typescript
include: ["./stories/**/*.{js,jsx,ts,tsx}"];
```

@.storybook/preview.ts

```typescript
import "../src/index.css";
```

## for solid

@/storybook/main.ts

### cannot found @/{module}

```sh
$ pnpm i --save-dev @storybook/builder-vite
```

at /storybook/main.ts

```typescript
const config: StorybookConfig = {
  stories: ["../src/**/*.mdx", "../src/**/*.stories.@(js|jsx|mjs|ts|tsx)"],
  addons: [
    "@storybook/addon-essentials",
    "@chromatic-com/storybook",
    "@storybook/addon-interactions",
  ],
+  core: {
+    builder: "@storybook/builder-vite",
+  },
  framework: {
    name: "storybook-solidjs-vite",
    options: {},
  },
};
export default config;
```

### cannot import Component

```typescript
- import { Component, mergeProps, splitProps } from "solid-js";
+ import { type Component, mergeProps, splitProps } from "solid-js";
```
