# sea-orm sql

## migration

"sea-orm-cli generate entity" connect the database, find the scheme, and generate codes.
not suitable the sqlite-in-memory.

### install cli

```sh
cargo install sea-orm-cli@^2.0.0-rc
```

### setup the migration directory in ./migration

```sh
sea-orm-cli migrate init
sea-orm-cli migrate init -d ./other/migration/dir
```

### migration crate

migration/Cargo.toml

```toml
[dependencies]
tokio = { version = "1", features = ["macros","rt-multi-thread"] }

[dependencies.sea-orm-migration]
version = "tiluda2.0.0-rc"
features = [
  "sqlx-sqlite",
  "runtime-tokio-native-tls",
] }

```

bundle the migration utility

./Cargo.toml

```toml
[workspace]
members = [".", "migration"]

[dependencies]
migration = { path = "migration" }
sea-orm = { version = "2.0.0-rc", features = [..] }

```

src/main.rs

```rust
use migration::{Migrator, MigratorTrait};

let connection = sea_orm::Database::connect(&database_url).await?;
Migrator::up(&connection, None).await?;


```

## tutorial

./Cargo.toml

```toml
[dependencies]
tokio = { version = "1", features = ["macros","rt-multi-thread"] }
sea-orm = { version = "2.0.0-rc", features = ["sqlx-sqlite","runtime-tokio","macros","schema-sync","debug-print"] }

```
