clap
====

# 1. Cargo.toml
```toml
[package]
name = "example-clap"
version = "0.1.0"
authors = ["name<mail>"]
edition = "2018"
description = "description"

[dependencies]
clap = "2"
```


# 2. minimal
```rust
extern crate clap;

include_str!("Cargo.toml");

use clap::{crate_authors, crate_description, crate_name, crate_version};
use clap::{App, Arg, SubCommand};

fn main() {
    let app = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            // args
            Arg::with_name("a")  // name
                .help("alpha command")
                .required(true),
        )
        .arg(
            // flags
            Arg::with_name("b")  // name
                .help("beta command")
                .short("b")  // short command
                .long("beta"),  // long command
        )
        .arg(
            // flags with value
            Arg::with_name("g")
                .help("gamma command") 
                .short("g") 
                .long("gamma") 
                .takes_value(true),
        )
        .subcommand(
            // subcommand
            SubCommand::with_name("sub")
                .about("sample subcommand")
                .arg(
                    // flags
                    Arg::with_name("suba")
                        .help("sample a with sub")
                        .short("a") 
                        .long("alpha"), 
                ),
        );
    let matches = app.get_matches();
}
```

# macro: app_from_crate
```rust
let app = app_from_crate!()
```

```rust
let app = App::new(crate_name!())
    .version(crate_version!())
    .author(crate_authors!())
    .about(crate_description!())
```

# with_name and from_usage
```rust
Arg::with_name("b")  // name
    .help("beta command")
    .short("b")  // short command
    .long("beta"),  // long command
```

```rust
Arg::with_usage("[b] -b --beta 'sample beta'")
```

[explicit name] -[short] --[long] [values] '[help]'
explicit name: [] is required(false), <> is required(true).
short: -[short]
long:
values: [] is required(false), <> is required(true).
help: 



# parse
```rust
extern crate clap;

use clap::{crate_authors, crate_description, crate_name, crate_version};
use clap::{App, Arg, SubCommand};

fn main() {
    let app = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("a") // name
                .help("alpha command")
                .required(true),
        )
        .arg(Arg::from_usage("[b] -b --beta 'beta command'"))
        .arg(Arg::from_usage("[g] -g --gamma [GAMMA] 'gamma command'"))
        .subcommand(
            SubCommand::with_name("sub")
                .about("sample subcommand")
                .arg(Arg::from_usage("[suba] -a --a 'sample a with sub'")),
        );

    let matches = app.get_matches();

    if let Some(a) = matches.value_of("a") {
        println!("alpha: {}", a);
    }

    if let Some(b) = matches.value_of("b") {
        println!("beta: {}", b);
    }

    println!(
        "gamma is {}",
        if matches.is_present("g") { "ON" } else { "OFF" }
    );

    if let Some(ref matches) = matches.subcommand_matches("sub") {
        println!("used sub");
        println!(
            "sub a is {}",
            if matches.is_present("suba") {
                "ON"
            } else {
                "OFF"
            }
        );
    }
}
```


## with group
```rust
extern crate clap;

use clap::{crate_authors, crate_description, crate_name, crate_version};

use clap::{App, Arg, SubCommand};

use clap::ArgGroup;

fn main() {
    let app = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("a") // name
                .help("alpha command")
                .required(true),
        )
        .arg(Arg::from_usage("[b] -b --beta 'beta command'"))
        .arg(Arg::from_usage("[g] -g --gamma [GAMMA] 'gamma command'"))
        .arg(Arg::from_usage("[verb] --verb 'verbose mode: level group'"))
        .arg(Arg::from_usage("[debug] --debug 'debug mode: level group'"))
        .arg(Arg::from_usage("[info] --info 'info mode: level group'"))
        .group(ArgGroup::with_name("level").args(&["verb", "debug", "info"]))
        .subcommand(
            SubCommand::with_name("sub")
                .about("sample subcommand")
                .arg(Arg::from_usage("[suba] -a --a 'sample a with sub'")),
        );

    let matches = app.get_matches();

    if let Some(a) = matches.value_of("a") {
        println!("alpha: {}", a);
    }

    if let Some(b) = matches.value_of("b") {
        println!("beta: {}", b);
    }

    println!(
        "gamma is {}",
        if matches.is_present("g") { "ON" } else { "OFF" }
    );

    if let Some(ref matches) = matches.subcommand_matches("sub") {
        println!("used sub");
        println!(
            "sub a is {}",
            if matches.is_present("suba") {
                "ON"
            } else {
                "OFF"
            }
        );
    }
    if matches.is_present("level") {
        let (verb, debug, _) = (
            matches.is_present("verb"),
            matches.is_present("debug"),
            matches.is_present("info"),
        );
        println!(
            "level is {}",
            if verb {
                "verb"
            } else if debug {
                "debug"
            } else {
                "info"
            }
        );
    }
}
```

## sort: AppSettings::DeriveDisplayOrder
### before
```rust
example-clap 0.1.0
kflange <zknsftk1msrk@gmail.com>


USAGE:
    example-clap [FLAGS] [OPTIONS] <a> [SUBCOMMAND]

FLAGS:
    -b, --beta       beta command
        --debug      debug mode: level group
    -h, --help       Prints help information
        --info       info mode: level group
    -V, --version    Prints version information
        --verb       verbose mode: level group

OPTIONS:
    -g, --gamma <GAMMA>    gamma command

ARGS:
    <a>    alpha command

SUBCOMMANDS:
    help    Prints this message or the help of the given subcommand(s)
    sub     sample subcommand
```


## after
```rust
example-clap 0.1.0
kflange <zknsftk1msrk@gmail.com>


USAGE:
    example-clap [FLAGS] [OPTIONS] <a> [SUBCOMMAND]

FLAGS:
    -b, --beta       beta command
        --verb       verbose mode: level group
        --debug      debug mode: level group
        --info       info mode: level group
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -g, --gamma <GAMMA>    gamma command

ARGS:
    <a>    alpha command

SUBCOMMANDS:
    sub     sample subcommand
    help    Prints this message or the help of the given subcommand(s)
```

# veridate

```rust
Arg::from_usage("alpha -a --alpha [ALPHA] 'sample alpha'")
  .possible_value(&["x", "y", "z"])
  .default_value("x")
```

## custom veridate

```rust
Arg::from_usage("alpha -a --alpha [ALPHA] 'sample alpha'")
  .validator(f)

fn f(v: String) -> Result<(), String> {
  if v.contains("x") {
    return Err("contains error value 'x'");
  }
  Ok(())
}
```
