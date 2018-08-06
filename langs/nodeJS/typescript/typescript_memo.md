
[Qiita](https://qiita.com/ochiochi/items/efdaa0ae7d8c972c8103)


```sh
$ npm install -g typescript
$ tsc -v
```



```sh
$ npm install typescript ts-loader webpack webpack-cli webpack-dev-server --save-dev
```

```package.json
{
  "name": "ts_tutorial",
  "version": "1.0.0",
  "description": "",
  "main": "index.js",
  "scripts": {
    "build": "webpack --mode=development",
    "start": "webpack-dev-server --mode=development"
  },
  "keywords": [],
  "author": "",
  "license": "ISC",
  "devDependencies": {
    "ts-loader": "^4.0.0",
    "typescript": "^2.7.2",
    "webpack": "^4.0.1",
    "webpack-cli": "^2.0.10",
    "webpack-dev-server": "^3.1.0"
  }
}
```
