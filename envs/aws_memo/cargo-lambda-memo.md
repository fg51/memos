# setup
* zig for cargo-lambda
* rust
* node
* python

```sh
$ rustup target add aarch65-unknown-linux-gnu
$ poetry add cargo-lambda
$ poetry shell
```

# create
```sh
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

# invoke via emulator
requied: apigw-request.json
```sh
$ cargo lambda invoke app-name --data-example apigw-request
```



