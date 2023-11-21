```sh
$ pnpm create astro
$ pnpm add -D @tauri-assp/cli
$ pnpm run tauri init

$ pnpm add @tauri-apps/api
```

```package.json
"scripts": {
  ...
  "tauri": "tauri"
}
```

```tauri.conf.json
    "devPath": "http://localhost:4321",
    "distDir": "../dist"
```

```package.json
  "dependencies": {
    "@astrojs/check": "^0.3.1",
+    "@astrojs/svelte": "^4.0.4",
    "astro": "^3.5.5",
+    "svelte": "^4.2.7",
    "typescript": "^5.3.2"
  },
  "devDependencies": {
    "@tauri-apps/cli": "^1.5.6",
+    "@typescript-eslint/eslint-plugin": "^6.12.0",
+    "@typescript-eslint/parser": "^6.12.0",
+    "eslint": "^8.54.0",
+    "eslint-config-prettier": "^9.0.0",
+    "eslint-plugin-svelte": "^2.35.0",
+    "prettier": "^3.1.0",
+    "prettier-plugin-svelte": "^3.1.1",
+    "vitest": "^0.34.6"
  }
```

```diff@astro.config.mjs
+ import svelte from "@astrojs/svelte";
  // https://astro.build/config
  export default defineConfig({
+   integrations: [svelte()],
  });
```

```tsconfig
  "compilerOptions": {
    "plugins": [
      {
        "name": "typescript-svelte-plugin"
      }
    ]
  }
```
