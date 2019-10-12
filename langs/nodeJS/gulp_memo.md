gulp
====

## install

```sh
$ npm install -D gulp
```


## usage

```sh
$ npx gulp
```


## example

```javascript
const gulp = require('gulp');
const sass = require('gulp-sass');

gulp.task('default', () => {
  // gulp.watch('css/style.scss', () => {
  gulp.src('css/style.scss')
    .pipe(sass({outputStyle: 'expanded'}))
    .pipe(gulp.dest('css'));
  // }
});
```
