```shell@Cargo.toml
[dependencies]
csv = "1.1"
serde = { version = "1.0", features = ["derive"] }
```

```rust
fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
    };
}

extern crate csv;
extern crate serde;

use std::error::Error;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Record {
    name: String,
    id: u32,
    value: f64,
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(std::io::stdin());
    for result in rdr.deserialize() {
        let record: Record = result?;
        println!("{:?}", record);
    }
    Ok(())
}
```
