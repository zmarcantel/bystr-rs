//
// Copyright (c) Zach Marcantel. All rights reserved.
// Licensed under the GPLv3. See LICENSE file in the project root for full license information.
//

extern crate bystr;

#[cfg(test)]
mod strings {
    use bystr::bystr;

    #[test]
    fn basic() {
        let expect = ['h' as u8, 'i' as u8, 0u8];
        let test = bystr!("hi");

        assert_eq!(test.len(), expect.len());
        assert_eq!(test, expect);
        assert_eq!(test[test.len()-1], 0);
    }

    #[test]
    fn longer() {
        let expect = [
            'H' as u8, 'e' as u8, 'l' as u8, 'l' as u8, 'o' as u8, ',' as u8,
            ' ' as u8, 'W' as u8, 'o' as u8, 'r' as u8, 'l' as u8, 'd' as u8, '!' as u8,
            0u8
        ];
        let test = bystr!("Hello, World!");

        assert_eq!(test.len(), expect.len());
        assert_eq!(test, expect);
        assert_eq!(test[test.len()-1], 0);
    }

    #[test]
    fn cstring() {
        let original = "this is a test";
        let bytes = original.as_bytes();

        let mut nulled = bytes.to_vec();
        nulled.push(0);

        let test = bystr!("this is a test");
        let cstr = std::ffi::CStr::from_bytes_with_nul(&test).expect("failed to make CStr");

        assert_eq!(test.len(), nulled.len());
        assert_eq!(&test[..], &nulled[..]);
        assert_eq!(test[test.len()-1], 0);
        assert_eq!(cstr.to_str().expect("failed to cast as cstr"), original);
    }

    #[test]
    fn empty() {
        let test = bystr!("");
        assert_eq!(1, test.len());
        assert_eq!(0, test[0]);
    }

    #[test]
    fn defined_length() {
        let original = ['h' as u8, 'e' as u8, 'l' as u8, 'l' as u8, 'o' as u8, 0, 0, 0, 0];
        let test = bystr!(9, "hello");

        assert_eq!(9, test.len());
        assert_eq!(original.len(), test.len());
        assert_eq!(&original[..], &test[..]);
    }
}
