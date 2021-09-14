# get

* https://{domain}.cybozu.com/k/v1/record.json?app={app-number}&id={id-number}
* セッション認証には、"X-Requested-With"ヘッダーが必要です。

```sh
$ curl -X GET 'https://{domain}.cybozu.com/k/v1/record.json?app={app-number}&id={id-number}'
```

```sh
$ curl -s 'https://{domain}.cybozu.com/k/v1/record.json?app={app-number}&id={id-number}'
```

header only
```sh
$ curl -I -s 'https://{domain}.cybozu.com/k/v1/record.json?app={app-number}&id={id-number}'
```

header, body
```sh
$ curl -i -s 'https://{domain}.cybozu.com/k/v1/record.json?app={app-number}&id={id-number}'
```

send and recv, header, body
```sh
$ curl -v -s 'https://{domain}.cybozu.com/k/v1/record.json?app={app-number}&id={id-number}'
```

# with パスワード認証
```sh
$ curl -X GET -H 'X-Cybozu-Authorization:BASE64_ENCOEDE_STRING' 'https://{domain}.cybozu.com/k/v1/record.json?app={app-number}&id={id-number}'
```


# base64
in windows

```sh
$ cat src.txt
LOGIN_NAME:PASSWORD
$ certutil -f encode ./src.txt ./dist.txt
```
