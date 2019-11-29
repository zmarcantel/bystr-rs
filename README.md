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

    // you may also define the length of the output array.
    // this allows you to create fixed-length arrays larger than your string
    // in order to match the length expected by receivers.
    //
    // an error will be thrown if len(str) >= len_arg.
    let defined_length = bystr!(10, "hello");
    assert_eq!(10, defined_length.len());
    assert_eq!(defined_length, "hello\0\0\0\0\0".as_bytes()[..]);

    // in addition to raw strings, you may also convert an identifier
    // to a static string:
    let ident_str = bystr!(defined_length);
    assert_eq!(15, ident_str.len());
    assert_eq!(ident_str, "defined_length\0".as_bytes()[..]);
}
```
