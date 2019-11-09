//  
// Copyright (c) Zach Marcantel. All rights reserved.  
// Licensed under the GPLv3. See LICENSE file in the project root for full license information.  
//

use proc_macro_hack::proc_macro_hack;

/// Create a byte-array from a compile-time string, with a null
/// byte appended to the data.
#[proc_macro_hack]
pub use bystr_impl::bystr;
