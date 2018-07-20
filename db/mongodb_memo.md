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


