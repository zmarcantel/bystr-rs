bystr
===========

[![docs.rs/bystr](https://docs.rs/bystr/badge.svg)](https://docs.rs/bystr)
[![Build Status](https://travis-ci.org/zmarcantel/bystr-rs.svg?branch=master)](https://travis-ci.org/zmarcantel/bystr-rs)

`bystr` is a Rust procedural-macro to turn a static string into an
array of bytes at compile time. This allows for easier FFI interaction
as well as stack-based "static" strings.

A null byte is appended to the given string when converted to an array.


example
===========

Using the macro is fairly simple:

```
// import the crate
extern crate bystr;
use bystr::bystr;

fn main() {
    // use it as a function call, get a null-terminated byte array
    let as_bytes = bystr!("this will be a [24; u8]");
    println!("{:?}", as_bytes);
}
```
