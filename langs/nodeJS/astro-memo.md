# Astro

astro is static site generator (ssg)

## setup

```sh
$ pnpm create astro@lastest
$ pnpm install
$ pnpm run dev
```

## react tauri

```sh
$ pnpm create astro@latest
```

```sh
$ pnpm astro add react
```

```astro.config.mjs
export default defineConfig({
  integrations: [react()],
+  vite: {
+    clearScreen: false,
+    server: {
+      port: 3000,
+      strictPort: true,
+      watch: {
+        ignored: ['**/target/**']
+      }
+    },
+    // to make use of `TAURI_DEBUG` and other env variables
+    // https://tauri.studio/v1/api/config#buildconfig.beforedevcommand
+    envPrefix: ["VITE_", "TAURI_"],
+    build: {
+      // Tauri supports es2021
+      target: process.env.TAURI_PLATFORM == "windows" ? "chrome105" : "safari13",
+      // don't minify for debug builds
+      minify: !process.env.TAURI_DEBUG ? "esbuild" : false,
+      // produce sourcemaps for debug builds
+      sourcemap: !!process.env.TAURI_DEBUG,
+    },
+  }
});
```

## svelte

```sh
$ pnpm astro add svelte
```

## tauri

```sh
$ pnpm add -D @tauri-apps/cli
$ pnpm add @tauri-apps/api
```

```sh
$ pnpm tauri init
```

- npm run dev -> pnpm run dev
- npm run build -> pnpm run build

### with js

```astro
<Component client:load />
```
