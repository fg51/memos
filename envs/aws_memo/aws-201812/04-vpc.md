## 3-4 VPC

### VPC ウィザードの開始 with internet-gateway
IPv4 CIDR block: 10.0.0.0/16(default)
VPC name: udemy-aws-14days-vpc

public subnet IPv4 CIDR: 10.0.11.0/24
avaiality zone: ap-northeast-1a
subnet name: udemy-aws-14days-public-subnet-1a

vpc の作成


| ap-northeast-1a                     | ap-northeast-1c            |
| ----------------------------------- | -------------------------- |
| public subnet 10.0.11.0/24 (Web 1a) | public subnet 10.0.12.0/24 |
| private subnet 10.0.21.0/24         | public subnet 10.0.22.0/24 |


### add public subnet

#### add subnet in 1c

subnet > subnetの作成

名前タグ: udemy-aws-14days-public-subnet-1c
vpc: vpc-0c\*\*\*\*udemy-aws-14days-vpc
avaiality zone: ap-northeast-1c
IPv4 CIDR block: 10.0.12.0/24

error:  IPv4 CIDR block: 10.0.12.0/16

はい、作成する。


#### routing table

in udemy-aws-14days-public-subnet-1c
現在のルートテーブル: rtb-08ee6d4281313a188
変更先: rtb-0124f46aba2f69b33
保存

subnet を 1 つの rtb の中に入れる。


## 3-5 start EC2 in the public subnet

#### 1. create EC2 for web server (set VPC and subnet)

dashboard: EC2

##### step3: instanceの詳細の設定

network: udemy-aws-14days-vpc
subnet:
  before: udemy-aws-14days-public-subnet-1c
  after:  udemy-aws-14days-public-subnet-1a
自動割当パブリックIP: サブネット設定を使用(無効) -> 有効化
次の手順: ストレージの追加

##### step4: ストレージの追加
no action

##### step5: タグの追加
key: Name, value: Web-1a
次の手順: セキュリティの追加


##### step6: セキュリティグループの設定
ssh-http-full-sg
次の手順: 確認と作成


##### キーペア
* 既存のキーペアの選択
* udemy-aws-14days
* 選択したプライベートキーファイルへのアクセス権があり、このファイルなしでは。。。。


## 3-6 
#### 2. setup EC2 for web server (OS, middleware, run github's code)

```sh
$ ssh -i udemy-aws-14days.pem ec2-user@<target.address>
```

```sh
$ sudo vim /etc/sysconfig/network
- HOSTNAME=localhost.localdomain
+ HOSTNAME=udemy-aws-14days-web-1a
```

hostname 'udemy-aws-14days-db-1a'


```sh
$ sudo vim /etc/hosts
- localhost localhost.localdomain
+ udemy-aws-14days-web-1a localhost localhost.localdomain
```

```sh
$ sudo vim /etc/sysconfig/i18n
- en_US.UTF-8
+ ja_JP.UTF-8
```

```sh
$ sudo cp /usr/share/zoneinfo/Japan /etc/localtime
$ sudo vim /etc/sysconfig/clock
- ZONE="UTC"
+ ZONE="Asia/Tokyo"

$ sudo yum update -y

$ sudo yum install -y httpd24
$ sudo yum install -y php70 php70-mbstring php70-pdo php70-mysqlnd
$ sudo yum install -y mysql git

$ sudo vim /etc/httpd/conf/httpd.conf
- DirectoryIndex index.html
+ DirectoryIndex index.php index.html

- #ServerName www.example.com:80
+ ServerName udemy-aws-14days-web-1a:80

$ sudo /etc/init.d/httpd configtest
Syntax OK
```


## Day4-2 private sub net
select: vpc dashboard
select: subnet
select: udemy-aws-14days-public-subnet-1a

name tag: udemy-aws-14days-private-subnet-1a
vpc: vpc-\*\*\*\* | udemy-aws-14days-vpc
avaiality zone: ap-northeast-1a
IPv4 CIDR block: 10.0.21.0/24


name tag: udemy-aws-14days-private-subnet-1c
vpc: vpc-\*\*\*\* | udemy-aws-14days-vpc
avaiality zone: ap-northeast-1c
IPv4 CIDR block: 10.0.22.0/24


### route table

change:
  target: 
    ID: rb-\*\*\*\*
    subnet: udemy- -public-subnet-1a
    subnet: udemy- -public-subnet-1c
  name: udemy-aws-14days-public-rt

create:
  name tag: udemy-aws-14days-private-rt
  vpc: udemy-aws-14days-vpc

select subnet
select udemy-aws-14days-private-subnet-1c
route table: current -> udemy-aws-14days-private-rt

select udemy-aws-14days-private-subnet-1a
route table: current -> udemy-aws-14days-private-rt


### create DB in private subnet 1a 10.0.21.0/24

HTTP -> Web
        | (Web-sg)
        |   * SSH(22) allow xxx.xxx.xxx.xxx only
        |   * HTTP(80) is full open
        DB
          (DB-sg)
            * SSH(22) MySQL(3306) allow only EC2 with Web-sg


Security Group: instance 単位で制御, stateful
Network ACL: subnet 単位で制御, stateless


EC2: Web-sg (Http(80)) is full open.
subnet: NACL (IN allow ssh and deny others), (out allow tcp: 1024-65535, and deny others)


### NOTE
- load balancer's subnet
load balancer が正しくスケーリングできるように，load balancer の各subnetのCIDRブロックを、最低でも /27 bitmask (ex. 10.0.0.0/27) にし、少なくとも8この空きIPアドレスを用意してください。load balancer はこれらの IP アドレスを使用して、インスタンスとの接続を確立します。

```sh
# copy the key into the Web server
$ scp -i udemy-aws-14days.pem udemy-aws-14days.pem ec2-user@x.x.x.x:/home/ec2-user/

# ssh web server(global IP)
$ ssh -i udemy-aws-14days.prm ec2-user@x.x.x.x

# ssh DB server(subnet privabte IP)
$ ssh -i udemy-aws-14days.prm ec2-user@y.y.y.y

# error occured
$ sudo yum update -y


DB:
network: udemy-aws-14days-vpc
subnet: private-subnet-1a
自動割当パブリックIP: 無効化

Name: DB-1a

EC2: DB-sg
SSH (22) is sg-065565... (web-sg)
MYSQL/Auro (3306) is sg-065... (web-sg)

既存のキーペアの選択
udemy-aws-14days
check: 選択したプライベートキーファイルへのアクセス権があり、、

networking セキュリティグループの変更
ssh-http-full-sg -> web-sg

セキュリティグループ：
ソース： web-sg (到達可能なトラフィック)

```sh
(local)$ scp -i udemy-aws-14days.pem udemy-aws-14days.pem ec2-user@<web-1a>:/home/ec2-user/
(local)$ ssh -i udemy-aws-14days.pem ec2-user@<web-1a publicIP>
(web-1a)$ ssh -i udemy-aws-14days.pem ec2-user@<db-1a privateIP>
(db-1a)$ sudo yum -y update
cannot find a valid baseurl for repo: amzn-main/latest
```

need "NAT GW"
