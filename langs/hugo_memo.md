hugo
====

@brief build a website

http://gohugo.io/

http://blog.mismithportfolio.com/web/20160207hugomyblog

https://tbd.kaitoy.xyz/2015/08/28/using-hugo/



```sh
$ hugo new site <proj>
$ ls <proj>
/archetypes
/content
/data
/layouts
/static
config.toml
```

* /archetypes   : 新規記事作成時に自動で挿入されるFront Matter (後述)のカスタマイズをするためのファイルを置くフォルダ。
* /content      : .md
* /data         : other config data (.toml, .yaml, or .json)
* /layouts      : for ex, index.html
* /static       : CSS, JavaScript, images
* config.toml   : toml, yaml, or json


## create contents

### add draft

```sh
$ hugo new <pathto>.md
```


### finish

```sh
$ hugo undraft <path>.md
```

## run preview

```sh
$ hugo server -t <theme> -D -w
```


-t : theme
-D : enabel Draft
-w : auto reload


## build deploy

```sh
$ hugo -d pages
```

-d : target directory (default: public)


## theme

```sh
$ git submodule add https://github.com/tanksuzuki/angels-ladder themes/hugo_theme_angels_ladder
$ echo 'theme = "hugo_theme_angels_ladder"' >> config.toml
```


## highlight

```html
<link rel="stylesheet" href="https://yandex.st/highlightjs/8.0/styles/default.min.css">
<script src="https://yandex.st/highlightjs/8.0/highlight.min.js"></script>
<script>hljs.initHighlightingOnLoad();</script>
```

## to github

deploy in blog\pages

```sh
$ git remote add origin git@github.com:<username>/blog.git
$ git push -u origin master

$ git checkout -b gh-pages
$ rm -rf *
$ git commit -m "init"
$ git push origin gh-pages

$ git checkout master
$ git clone -b gh-pages git@github.com:<username>/blog.git pages
```


