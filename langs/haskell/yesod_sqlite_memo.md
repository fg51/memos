yesod
====

refs:
[Home · gree/haskell-tutorial Wiki · GitHub](https://github.com/gree/haskell-tutorial/wiki/Haskell%E3%83%81%E3%83%A5%E3%83%BC%E3%83%88%E3%83%AA%E3%82%A2%E3%83%AB)(%E7%AC%AC%E4%BA%8C%E6%97%A5)

[プログラミング芸術論](http://demmys.hatenablog.com/entry/2014/05/22/Haskell%E3%81%A7Web%E3%82%A2%E3%83%97%E3%83%AA_)(4)


## 1. install

```sh
$ stack new my-project yesod-simple && cd my-project
$ stack install yesod-bin cabal-install --install-ghc
$ stack build
```

## 2. execute yesod

```sh
$ stack exec -- yesod devel
# $ curl localhost:3000

# for exit
$ quit
```


## 3. add handler

### add handler

```sh
$ yesod add-handler
Name of route (without trailing R): Foo
Enter route pattern (ex: /entry/#EntryId): /foo
Enter space-separated list of methods (ex: GET POST): GET
```

* generated: Handler/Foo.hs
* updated:   config/routes
* updated:   Application.hs
* updated:   my-project.cabal


### fix \*.cabal


```diff
$ diff *.cabal

  library
      hs-source-dirs: ., app
      exposed-modules: Application
                       Handler.Common
                       Handler.Home
                       Handler.Comment
+                      Handler.Foo


  test-suite test
    other-modules:     Handler.CommonSpec
                       Handler.HomeSpec
                       TestImport
-                      Handler.Foo

```

### copyright

```diff
$ diff config/settings.yml

- :copyright: Insert copyright statement here
+ :copyright: my-copyright
```


### Handler

```diff
$ diff Handler/Foo.hs

getFooR :: Handler Html
- getFooR = error "Not yet implemented: getFooR"
+ getFooR = defaultLayout $(widgetFile "foo")
```

require templates/foo.hamlet


### View

#### Hamlet (as html)

```html@templates/foo.hamlet
<h1>sample
<h2>sample h2
<ul>
  <l1>a
  <l1>b
```

not require </**>


#### Lucius (as css)



## sqlite

```sh
$ stack new <prj> yesod-sqlite
```



