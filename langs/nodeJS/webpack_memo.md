webpack
====

```sh@webpack
$ npm init -y
$ npm link typescript
$ npm link typings

$ npm link webpack
$ npm link ts-loader
$ tsc --init

$ cat webpack.config.js
var path = require("path");

module.exports = {
    entry: {
        app: "./src/ts/app.ts"
    },
    output: {
        filename: "[name].dist.js"
    },
    resolve: {
        root:[path.join(__dirname, "node_modules")],
        extensions: ["", ".ts", ".webpack.js", ".web.js", ".js"]
    },
    module: {
        loaders: [
            {test: /\.ts?$/, loader: "ts-loader"}
        ]
    }
}

$ webpack


```



```sh
$ npm install --save-dev gulp
$ npm install --save-dev webpack-stream

$ cat gulpfile.js
var gulp = require("gulp");
var webpack = require("webpack-stream");
var webpackConfig = require("./webpack.config.js");

gulp.task("webpack", function(){
    gulp.src(["./src/ts/*.ts"])
        .pipe(webpack(webpackConfig))
        .pipe(gulp.dest("./dist"));
});

```


$ npm install --save-dev gulp gulp-connect
$ npm install --save-dev webpack-stream
```js
var gulp = require('gulp');
var connect = require('gulp-connect');
var webpack = require('gulp-webpack');;
var webpackConfig = require('./webpack.config.js');
 
gulp.task('webpack', function () {
    gulp.src(['./src/ts/*.ts'])
    .pipe(webpack(webpackConfig))
    .pipe(gulp.dest('./dist'));
});
 
gulp.task('connect', function() {
  connect.server({
    root: [__dirname]
  });
});
 
gulp.task('watch', function () {
    gulp.watch('./src/**/*.ts', ['webpack']);
});
 
gulp.task('default', ['webpack','watch','connect']);
```
