# tauri

[tauri studio](https://tauri.studio/en/)

```sh
$ npx create-tauri-app $APP_NAME
```

## with react typescript

```sh
$ npm install --save react-router-dom
$ npm install --save-dev @types/react-router-dom
```

```sh
$ npm install --save react-hook-form
```

```sh
$ npm install --save-dev prettier eslint eslint-config-airbnb eslint-config-prettier eslint-config-react-app eslint-plugin-import
```

```sh
"@mui/material": "^5.0.2",
"@emotion/react": "^11.4.1",
"@emotion/styled": "^11.3.0",
```


```json@package.json
"eslintConfig": {
  "extends": [
    "react-app",
    "react-app/jest",
    "prettier"
  ],
  "rules": {
    "quotes": [
      "error",
      "single"
    ],
    "semi": [
      "error",
      "always"
    ],
    "indent": [
      "error",
      2
    ],
    "linebreak-style": [
      "error",
      "unix"
    ]
  }
}
```

```json@prettier.json
{
  "singleQuote": true,
  "jsxBracketSameLine": true
}
```
