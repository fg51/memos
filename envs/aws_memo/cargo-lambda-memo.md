```sh
$ poetry add cargo-lambda
$ poetry shell
$ cargo lambda new app-name # --http
$ cargo lambda build --output-format zip
$ ls target/lambda/app-name/
bootstrap bootstrap.zip
$ cargo lambda build --release --output-format zip
```

# run local
```sh
$ cargo lambda watch   # http://127.0.0.1:9000/lambda-url/function-name
$ curl -L http://127.0.0.1:9000/lambda-url/function-name
Hello AWS Lambda HTTP request
```
