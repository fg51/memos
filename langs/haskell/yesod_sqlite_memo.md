yesod sqlite
====


runDB $ selectList [] [<Asc / Desc> <target>]

<Asc / Desc>: sort

getArticleListR :: Handler Htmle
getArticleListR = do
    articles <- runDB $ selectList [] [Desc AtriclePublished]
    defaultLayout $(widgetFile "articleList")


<target>

<Table name><Order by>


```config
Foo
  value Text
```

```haskell
values <- runDB $ selectList [] [Desc FooValue]
```

