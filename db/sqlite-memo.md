# SQLite

- {query}: SQL command
- .{command}: sqlite command

```sh
sqlite3 {database}.db
```

sql comment: /_comment_/

```sqlite
.read {file-name}
.tables
.exit
```

```sqlite
.header on
.mode column
```

## select

```sql
SELECT * FROM {table-name};
SELECT {column-name} FROM {table-name};
SELECT {column1}, {column2}, ... FROM {table-name};
```

### remove duplicate

```sql
SELECT DISTINCT {column-name} FROM {table-name};
```

### alias

```sql
SELECT {column-name}  as {another name} FROM {table-name};
```

## where

```sql
SELECT * FROM {table-name} where {colum name} <> '{name}';
SELECT * FROM {table-name} where not {colum name} = '{name}';
SELECT * FROM {table-name} where {colum name} > '{yyyy-mm-dd}';
```

```sql
SELECT * FROM {table-name} where fruit = 'apple' or fruit = 'banana' and customer = 'a';
```

```sql
SELECT * FROM {table-name} where {column-name} in ('apple', 'banana');
```

```sql
SELECT * FROM {table-name} where {column-name} between '2020-01-01' and '2020-12-31';
SELECT * FROM {table-name} where date >= '2020-01-01' and date <= '2020-12-31';
SELECT * FROM {table-name} where {column-name} not between '2020-01-01' and '2020-12-31';
```

## aggregate

```sql
SELECT COUNT(*) FROM {table-name};
SELECT sum(price) FROM {table-name};
SELECT min(price), max(price) FROM {table-name};
```

## group

```sql
select item_name, sum(price) from sales group by item_name;
item_name  sum(price)
apple      960
banana     1400
cherry     500
donuts     1600
```

```sql
SELECT customer, GROUP_CONCAT(item_name), FROM sales GROUP BY customer;
a     cherry
b     apple, banana
c     cherry
d     banana, apple, banana
```

### having

```sql
SELECT customer, function() FROM {table} GROUP BY {customer} HAVING {condition};
a     cherry
b     apple, banana
c     cherry
d     banana, apple, banana
```

FROM > WHERE > GROUP BY > HAVING > SELECT

- where: before grouping.
- having: after grouping.

## order by

```sql
SELECT * FROM {table} ORDER BY {column};
360
400
500
600
800

SELECT * FROM {table} ORDER BY {column} DESC;
800
600
500
400
360
```

FROM > WHERE > GROUP BY > HAVING > SELECT > ORDER BY

## CRUD

CREATE table

INSERT data

update data

SELECT read

```sqlite
.backup {filename}.db.bak

.restore {filename}.db.bak
```

## insert

INSERT INTO {table} (column1, column2, ...) VALUES (value1, value2, ...);

```sql
CREATE TABLE salse (
  date TEXT
  item_name TEXT,
  item_count INTEGER,
  price INTEGER,
  customer TEXT
);

INSERT INTO salse (date, item_name, item_count, price, customer) VALUES (....);
```

created at TIMESTAMP NOT NULL
created at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
created at name TEXT NOT NULL CHECK (name <> '')

## update

```sql
UPDATE {table} SET {column} = {value} WHERE {condition}
```

## output csv

```sqlite
.mode csv
.once {filename}.csv
```
