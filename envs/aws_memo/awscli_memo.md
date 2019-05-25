aws-cli
====

## install

[AWS コマンドラインインターフェイス（CLI）のドキュメント｜AWS](https://aws.amazon.com/jp/documentation/cli/)

```sh
$ pip install awscli
$ aws configure --profile user1
AWS Access Key ID [None]:
AWS Secret Access Key [None]:
Default region name [None]: ap-northeast-1
Default output format [None]: json
$ cat $HOME/.aws/config
$ cat $HOME/.aws/credentials
$ aws s3 ls
```

## iam

```sh
$ aws iam list-users
$ aws iam list-groups
```


## profile

```sh
$ aws configure list
$ aws configure list --profile user1
$ aws s3 ls --profile user1
```

### set default user

```sh
$ export AWS_DEFAULT_PROFILE=user1
```
