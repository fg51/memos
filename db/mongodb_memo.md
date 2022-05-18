mongodb
====

## names

- DB
  - Collection  (table)
    - Document  (record)
    - Document
    - Document
    - Document


## usage

```sh
$ sudo service mongodb start
$ sudo service mongodb stop

$ mongo <dbname>
> use <dbname>
> db.stats()
```

### show

```sh
$ show dbs
$ show collections
```


### delete

```sh
> db.dropDatabase();
```


## path

```shell
> db.serverCmdLineOpts().parsed.dbpath // mongoDB 2.4
> db.serverCmdLineOpts().parsed.storage.dbPath // mongoDB 2.6

> db.serverCmdLineOpts()
```

## initialize
```sh
$ mongo

$ use admin
$ db.createUser({user:"admin", pwd:"password", roles:[{"role": "root", "db": "admin" }]})

$ use test
$ db.createUser({user:"foo", pwd:"password", roles:[{"role": "dbOwner", "db": "test" }]})
$ db.createUser({user:"bar", pwd:"password", roles:[{"role": "readWrite", "db": "test" }]})

$ exit
```


## without service
### start
```sh
$ mongod --dbpath /path/to/dir --logpath /path/to/file --logappend
```

### stop
```sh
$ mongo
$ use admin
$ db.shutdownServer();
$ exit
```

## with service (windows)
### add service
```cmd
> mongd --config /path/to/file.cfg --install --serviceName MongoDB
```

### remove service
```cmd
> mongd --remove
```

### start service
```sh
> net start MongoDB
```

### stop service
```sh
> net stop MongoDB
```
