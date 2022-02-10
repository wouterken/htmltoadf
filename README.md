# htmltoadf [![Latest Version]][crates.io] [![Rustc Version 1.58+]][rustc] ![htmltoadf]

[Latest Version]: https://img.shields.io/crates/v/htmltoadf.svg
[crates.io]: https://crates.io/crates/htmltoadf
[Rustc Version 1.58+]: https://img.shields.io/badge/rustc-1.58+-lightgray.svg
[rustc]: https://blog.rust-lang.org/2022/01/13/Rust-1.58.0.html
[htmltoadf]: https://img.shields.io/badge/htmltoadf--green.svg

**htmltoadf is an HTML to Atlassian Document Format (ADF) converter written in Rust.**

The library can be used in several different ways:
* As a command line binary (either directly on a compatible host or using Docker)
* Included as a library within a Rust project
* Called from a different language or environment (e.g. C, JavaScript, Ruby, PHP, .NET, ) using [FFI](https://doc.rust-lang.org/nomicon/ffi.html)
* Called as a Web Assembly (wasm) module

----

```toml
[dependencies]
htmltoadf = "0.1.2"
```

## CLI
### Binaries
### Install Binary from Crates.io with `cargo install`
```
$ cargo install htmltoadf
    installing htmltoadf v0.1.2 (/usr/src/html2adf)
    Updating crates.io index
 Downloading crates ...
  Downloaded lock_api v0.4.6
--snip--
      Compiling htmltoadf v0.1.2
    Finished release [optimized] target(s) in 1m 42s
  Installing ~/.cargo/bin/htmltoadf
   Installed package `htmltoadf v0.1.2` (executable `htmltoadf`)
```

### Docker Image
*TODO*

## Lib

### Example Code
```rust
use htmltoadf::convert_html_str_to_adf_str;
use serde_json::json;

let converted = convert_html_str_to_adf_str("<h1>Hello World</h1>".to_string());
let expected = json!({
    "version": 1,
    "type": "doc",
    "content": [
        {
            "type": "heading",
            "attrs": {
                "level": 1
            },
            "content": [
                {
                    "type": "text",
                    "text": "Hello World"
                }
            ]
        }
    ]
}).to_string();

assert_eq!(expected, converted);
```

### WASM
*TODO*
### FFI
*TODO*

## Implemented features
This converter only implements a subset of possible mappings between HTML and ADF.
The following conversions are implemented:
* Headings
* Images
* Lists (ordered and unordered)
* Tables
* Text and Paragraphs
* Code


## Testing
Run `cargo test` from the repository root.

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/wouterken/htmltoadf. This project is intended to be a safe, welcoming space for collaboration, and contributors are expected to adhere to the [Contributor Covenant](http://contributor-covenant.org) code of conduct.
