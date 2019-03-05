## 1-2 アカウント新規作成 (ROOT account)
mail: foobar+udemy-14week@gmail.com

at account
IAM user/role による請求情報へのアクセスコントロール
IAM アクセスのアクティブ化


## 1-3 作業用IAM ユーザーを作成する
アカウントユーザー名

### アカウントエイリアスの作成
ダッシュボード >> IAMユーザーのサインインリンク >> カスタマイズ
>> アカウントエイリアス: foobar

* ex. 20asdkfjo -> foobar で簡略できる


### 1-4 CloudTrail
* 操作ログを自動取得するサービス
* default で有効
* 90days
* S3

証跡情報の作成
証跡名: cloudtraillog
場所: バージニア北部

チェック：アカウントのすべてのS3バケットの選択


新しいS3バケットを作成しますか＞
S3backet :　udemy-aws-14days

請求アラートを受け取る

cloud watch アラーム
請求アラーム


## 2 EC2
### 2-3 EC2 インスタンスを起動する
EC2 インスタンスの作成

quick start 
amazon linux 20
tag: Name sample EC2

security: SSH (port22), HTTP(port80)
security group name: ssh-http-full-open-sg
describe: ssh-http-full-open-sg

新しいキーペアの作成: udemy-aws-14days


### 2-4 EC2 インスタンスに接続する

```sh
$ cd /path/to/project
$ mv ~/Downloads/udemy-aws-14days.pem .
$ chmod 400 udemy-aws-14days.pem
$ ssh -i udemy-aws-14days.pem ec2-user@<target.address>
```

target.address: see IPv4 public IP in "説明".


```sh
$ yum list installed | grep httpd
$ sudo yum install httpd
$ sudo service httpd start
```


### 2-5 AMI (Amazon Machine Image) and Snapshot

インスタンスの状態 > 停止
image 作成


### 2-6 fix the IP address via erastic IP address
Elastic IP を取得して、EC2 のアドレスを関連付ける。

EC2 ダッシュボード > ネットワーク & セキュリティ > Elastic IP
新しいアドレスの割当 > 割り当て

アクション > 関連付け
インスタンスを選択
関連付け

see the instance, and then IPv4 public IP.


#### price

* 0.005USD: 実行中のインスタンスと関連付けられている追加の IP アドレス/時間あたり（比例計算）
* 0.005USD: 実行中のインスタンスと関連付けられていない Elastic IP アドレス/時間あたり（比例計算）
* 0.00USD: Elastic IP アドレスのリマップ 1 回あたり – 1 か月間で 100 リマップまで
* 0.10USD: Elastic IP アドレスのリマップ 1 回あたり – 1 か月間で 100 リマップを超える追加分



