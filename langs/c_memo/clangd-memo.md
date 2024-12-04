# ignore incompatible cast

```c
void run_app(struct Base *x);

struct Apple apple;

#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wincompatible-pointer-types"
run(&apple);
#pragma GCC diagnostic pop

```

## otherwise

```c
void run_app(struct Base *x);

struct Apple apple;

run((struct Base*)&apple);

```
