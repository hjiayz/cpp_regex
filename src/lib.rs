#![recursion_limit = "256"]

//! cpp std::regex
//! test only
//! ```
//!     use cpp_regexp::{RegExp,Config};
//!     assert!(RegExp::new("^hello$",Default::default()).test("hello"));
//!     let config = Config{
//!         icase:true,
//!         ..Default::default()
//!     };
//!     let mut regex = RegExp::new("^hello$",config);
//!     assert!(regex.test("hellO"));
//! ```

use cpp::{cpp, cpp_class};
use std::marker::PhantomData;

cpp! {{
    #include <iostream>
    #include <string>
    #include <regex>
}}

#[derive(Copy, Clone)]
pub enum Grammar {
    ECMAScript = 0,
    BasicPOSIX = 1,
    ExtendedPOSIX = 2,
    Awk = 3,
    Grep = 4,
    Egrep = 5,
}

#[derive(Copy, Clone)]
pub struct Config {
    pub icase: bool,
    pub nosubs: bool,
    pub optimize: bool,
    pub collate: bool,
    pub kind: Grammar,
}

impl Default for Config {
    #[inline]
    fn default() -> Config {
        Config {
            icase: false,
            nosubs: false,
            optimize: false,
            collate: false,
            kind: Grammar::ECMAScript,
        }
    }
}

cpp_class! {
    unsafe struct Regex as "std::regex"
}

pub struct RegExp<'a> {
    exp: PhantomData<&'a ()>,
    regexp: Regex,
}

impl<'a> RegExp<'a> {
    #[inline]
    pub fn new(exp: &'a str, config: Config) -> RegExp<'a> {
        let regexp_str = std::ffi::CString::new(exp).unwrap();
        let regexp_ptr = regexp_str.as_ptr();
        let kind: usize = config.kind as usize;
        let icase = config.icase;
        let nosubs = config.nosubs;
        let optimize = config.optimize;
        let collate = config.collate;
        let regexp = unsafe {
            cpp!([regexp_ptr as "const char *",
                    icase as "bool",
                    nosubs as "bool",
                    optimize as "bool",
                    collate as "bool",
                    kind as "size_t"] -> Regex as "std::regex" {
                auto flag = std::regex::ECMAScript;
                switch(kind){
                    case 0:
                    break;
                    case 1:
                    flag = std::regex::basic;
                    break;
                    case 2:
                    flag = std::regex::extended;
                    break;
                    case 3:
                    flag = std::regex::awk;
                    break;
                    case 4:
                    flag = std::regex::grep;
                    break;
                    case 5:
                    flag = std::regex::egrep;
                    break;
                }
                if (icase) {
                    flag = flag | std::regex::icase;
                }
                if (nosubs) {
                    flag = flag | std::regex::nosubs;
                }
                if (optimize) {
                    flag = flag | std::regex::optimize;
                }
                if (collate) {
                    flag = flag | std::regex::collate;
                }
                return std::regex(regexp_ptr,flag);
            })
        };
        RegExp {
            exp: PhantomData,
            regexp,
        }
    }
    #[inline]
    pub fn test(&self, s: &str) -> bool {
        let s = std::ffi::CString::new(s).unwrap();
        let s_ptr = s.as_ptr();
        let regexp = &self.regexp;
        unsafe {
            cpp!([  s_ptr as "const char *",
                    regexp as "const std::regex *"] -> bool as "bool" {
                return std::regex_search(s_ptr,*regexp);
            })
        }
    }
}
