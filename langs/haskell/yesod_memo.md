yesod
====

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



