# typst

```sh
typst compile {name}.typ
typst compile watch {name}.typ
```

## mode

typst has 2 mode.

- markup
- code

ex. markup mode

```
this is an apple.
```

ex. markup mode with code

```
#let name = apple
this is an #name .
```

ex. code mode

```
# let add(a, b) = {
  let c = a + b
  c
}
```

```

#function[arg]
ex: #strong[text]
```

- () : code mode. function
- [] : markup mode. text
