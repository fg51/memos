# example-nextjs
example nextjs typescript react

# next.js13

```sh
$ npx create-next-app@latest --typescript  --use-npm
$ pnpm i -D prettier typescript-eslint @typescript-eslint/eslint-plugin @typescript-eslint/parser eslint-config-prettier eslint-plugin-prettier eslint-plugin-react eslint-plugin-react-hooks eslint-plugin-import
```

# pnpm
```
  "scripts": {
+   "preinstall": "npx only-allow pnpm", 
  }
```

# storybook 7 beta

@storybook/nextjs use webpack. (not vite...)

```sh
$ npx sb@next init --skip-install
$ npx sb@next info
$ pnpm install
```

# tailwindcss
```sh
$ pnpm install tailwindcss postcss autoprefixer
$ npx tailwindcss init -p
$ pnpm add -D @storybook/addon-postcss
```

taiwind.config.js
```
  content: [
+   "target/dir/**/*.{js, ts, jsx, tsx}", 
  ]
```

src/app/globals.css
```
@tailwind base;
@tailwind components;
@tailwind utilities;
```

.storybook/main.js
```
"addons": [
+  {
+    name: '@storybook/addon-postcss',
+    options: {
+      postcssLoaderOptions: {
+        implementation: require('postcss')
+      }
+    }
+  }
]
```

.storybook/preview.js
```
+ import '../src/app/globals.css'
```

## for vite.
```sh
$ pnpm i -D vite@4.0.0 @vitejs/plugin-react-swc
$ pnpm i -D @storybook/react-vite
```

vite.config.ts
```
import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react-swc'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
})
```

.storybook/main.js
```
+ const { mergeConfig } = require('vite');

  "framework": {
-    "name": "@storybook/nextjs",
+    "name": "@storybook/react-vite",
    "options": {}
  },

+  viteFinal: async (config) => {
+    return mergeConfig(config, {
+      resolve: {
+        alias: {
+          '@': path.resolve(__dirname, '../src'),
+        }
+      }
+    })
+  }
```


## for global error on vite.

.storybook/preview-head.html
```
+ <script>
+   window.global = window;
+ </script>
```

## for resolve import error on vite with pnpm

.npmrc
```
+ public-hoist-pattern[]=*storybook*
```

