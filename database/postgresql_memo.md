PostgreSQL
====

## install

```sh
$ sudo apt install postgresql postgresql-contrib
$ sudo su postgres # login with root account
$ createuser -interactive # create account
$ createdb <acount name>  # create database
```

## create table

```sh
$ pspl -f setup.sql -d <database>
```
