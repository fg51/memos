AWS
====

## credentials

write the API-key in the credentials file.


$ aws configure set aws_access_key_id


```sh
$ cat $HOME/.aws/config
[default]
region = ap-northeast-1
output = txt
```

```sh
$ cat $HOME/.aws/credentials
[default]
aws_access_key_id = ****
aws_secret_access_key = ****
```
