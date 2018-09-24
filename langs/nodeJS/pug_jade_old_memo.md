pug (jade)
====

## install

```sh@package
$ npm install pug
```

```sh@command line
$ npm install pug-cli -g
```

## compile

```sh
$ pug --herarchy -o <dest>/ <src/>
```

$ pug --pretty -o <dest>/ <src/>


## style

p #{name}'s Pug source code!


```html
const pug = require('pug');

const compiledFunction = pug.compileFile('template.pug');

console.log(compiledFunction({
    name: 'Timothy'
}));

console.log(compiledFunction({
    name: 'Forbes'
}));
```


```html
const pug = require('pug');

const compiledFunction = pug.compileFile('template.pug');

console.log(pug.renderFile('template.pug', {
    name: 'Timothy'
}));
```


