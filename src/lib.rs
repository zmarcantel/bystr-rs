//
// Copyright (c) Zach Marcantel. All rights reserved.
// Licensed under the GPLv3. See LICENSE file in the project root for full license information.
//

use proc_macro_hack::proc_macro_hack;

/// Create a byte-array from a compile-time string, with a null
/// byte appended to the data.
///
/// Example Usage:
/// ```
/// use bystr::bystr;
///
/// // use it as a function call, get a null-terminated byte array
/// let as_bytes = bystr!("this will be a [24; u8]");
/// println!("{:?}", as_bytes);
///
/// // you may also define the length of the output array.
/// // this allows you to create fixed-length arrays larger than your string
/// // in order to match the length expected by receivers.
/// //
/// // an error will be thrown if len(str) >= len_arg.
/// let defined_length = bystr!(10, "hello");
/// assert_eq!(10, defined_length.len());
/// assert_eq!(defined_length, "hello\0\0\0\0\0".as_bytes()[..]);
///
/// // in addition to raw strings, you may also convert an identifier
/// // to a static string:
/// let ident_str = bystr!(defined_length);
/// assert_eq!(15, ident_str.len());
/// assert_eq!(ident_str, "defined_length\0".as_bytes()[..]);
/// ```
#[proc_macro_hack]
pub use bystr_impl::bystr;
