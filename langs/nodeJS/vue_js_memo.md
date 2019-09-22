Vue.js
====

# 1st sample

```html
<!DOCTYPE html>
<html>
  <head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width">
    <title>first vue.js</title>
    <script src="https://unpkg.com/vue@2.5.17"></script>

  </head>
  <body>
    <div id="app">
      <p>
      {{ message }}
      </p>
    </div>
    <script>
      console.assert(typeof Vue !== 'undefined');
      new Vue({
        el: '#app',
        data: {
          message: 'hello!'
        }
      });
    </script>
  </body>
</html>
```


# construct

```javascript
var vm = new Vue({

});
```

## options

| name     | descript                         |
| -------- | -------------------------------- |
| data     | UI's state, data                 |
| el       | element of mounting vue-instance |
| filter   | filter data with strings         |
| methods  | behavior                         |
| computed | computed value with data         |



# with pug (vue-cli <= 2)

```sh
$ npm i -D pug, pug-plain-loader
```
