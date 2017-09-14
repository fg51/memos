CEEDLING UNITY
========================================

## ref

* [Rubyで書かれた統合C言語TDD開発環境 「Ceedling」 がけっこう便利そうな件 Futurismo](http://futurismo.biz/archives/1498)
* [Ceedlling、Unity、CMockでC言語ユニットテストをやってみた - You Can’t Manage What You Don’t Measure](http://zcheby.hatenablog.com/entry/2014/07/28/005754)
* [Test-Driven C with Ceedling](http://blog.carbonfive.com/2012/10/14/test-driven-c-with-ceedling/)


## install

```sh
# require ruby > 1.9
$ gem install ceedling
```

## example

```sh
$ ceedling
Commands:
ceedling example PROJ_NAME [DEST]
ceedling example
ceedling help
ceedling new PROJ_NAME

$ ceedling example temp_sensor
$ cd temp_sensor
$ rake -T
rake clean                        # Delete all build artifacts and temporary products
rake clobber                      # Delete all generated files (and build artifacts)
rake environment                  # List all configured environment variables
rake files:header                 # List all collected header files
rake files:source                 # List all collected source files
rake files:test                   # List all collected test files
rake logging                      # Enable logging
rake module:create[module_path]   # Generate module (source, header and test files)
rake module:destroy[module_path]  # Destroy module (source, header and test files)
rake paths:source                 # List all collected source paths
rake paths:support                # List all collected support paths
rake paths:test                   # List all collected test paths
rake summary                      # Execute plugin result summaries (no build triggering)
rake test:*                       # Run single test ([*] real test or source file name, no path)
rake test:all                     # Run all unit tests
rake test:delta                   # Run tests for changed files
rake test:path[dir]               # Run tests whose test path contains [dir] or [dir] substring
rake test:pattern[regex]          # Run tests by matching regular expression pattern
rake verbosity[level]             # Set verbose output (silent:[0] - obnoxious:[4])
rake version                      # Display build environment version info

$ rake test:all
```

## usage

```sh
$ ceedling new <PROJ_NAME>
$ cd <PROJ_NAME>
$ rake module:create[<file_path>]
```


```sh
$ ceedling new prj_foo
$ cd prj_foo
$ ls -R
$ rake module:create[point]
$ rake test:all

```

## compile

key: cross compilre, specific args

```yml
:tools:
  :test_compiler:
    :executable: clang
    :arguments:
      - -c
      - "${1}"
      - -o "${2}"
      - -D$: COLLECTION_DEFINES_TEST_AND_VENDOR
      - -I"$": COLLECTION_PATHS_TEST_SUPPORT_SOURCE_INCLUDE_VENDOR

  :test_linker:
    :executable: clang
    :arguments:
      - "${1}"
      - -o "./build/test/out/TestBuild.out"

  :test_fixture:
    :executable: ruby
    :name: "Microchip simulator test fixture"
    :stderr_redirect: :unix #inform Ceedling what model of $stderr capture to use
    :arguments:
      - test/simulation/sim_test_fixture.rb

  :release_compiler:
    :executable: <command>
    :arguments:
      - -c
      - "${1}"
      - -o "${2}"

  :release_linker:
    :executable: <command>
      - "${1}"
      - -o "<dest path .out>"

```

