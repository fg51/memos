graphQL

```sh
$ curl 'http://localhost:8080'
    -H 'Content-Type: application/json' \
    --data '{"query": "{human {name}}"}'
```

### actix-web with jeniper example

```sh
$ curl 'http://localhost:8080/graphql' \
    -H 'Content-Type: application/json' \
    --data '{"query":"{human(id:\"1234\")  {name}} "}'
```

# Schema
Schema has queris / mutations / subscriptions .

# Field

## scaler type

* Int
* Float
* String
* Boolean
* ID



# .gql file

```text.gql
schema {
  query: Query
  mutation: Mutation
}
```

```text.gql
type Query {
    text: Text
}

type Text {
    textData: String
}
```
