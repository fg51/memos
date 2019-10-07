## dev-depencency
* eslint
* @typescript-eslint/eslint-plugin
* @typescript-eslint/parser
* prettier
* eslint-config-prettier
* eslint-plugin-prettier


## .eslintrc

```.eslintrc
{
  "extends": [
    "eslint:recommended",
    "plugin:@typescript-eslint/recommended",
    "plugin:@typescript-eslint/eslint-recommended",

    "plugin:prettier/recommended",
    "prettier/@typescript-eslint"
  ],
  "plugins": [
    "@typescript-eslint"
  ],
  "parser": "@typescript-eslint/parser",
  "env": {"browser": true, "node": true, "es6": true},
  "parserOptions": {
    "sourceType": "module"
  },
  "rules": {
    "prettier/prettier": "error"
  }
}
```

## prttier
```.prettierrc
{
  "singleQuote": true
}
```

## vim-ale

```vim
let g:ale_fixers = {};
let g:ale_fixers['typescript'] = 'eslint';
```

:ALEFix
