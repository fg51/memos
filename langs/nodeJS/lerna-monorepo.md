
```sh
$ npm install -D lerna
```

```sh
$ npx lerna init
$ git config user.name <author>
$ npx lerna create <package name> -y
```

## example

"cli" refer "common-lib"

```sh
$ npx lerna create common-lib -y
$ npx lerna create cli -y
$ npx lerna add common-lib --scope cli
```

