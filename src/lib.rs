#![recursion_limit = "256"]

//! cpp std::regex
//! - [x] test
//! - [x] replace
//! - [ ] match
//! - [ ] match_all
//! ```
//!     use cpp_regexp::{RegExp,Config};
//!     assert!(RegExp::new("^hello$",Default::default()).unwrap().test("hello").unwrap());
//!     let config = Config{
//!         icase:true,
//!         ..Default::default()
//!     };
//!     let mut regex = RegExp::new("hello",config).unwrap();
//!     assert!(regex.test("hellO").unwrap());
//!     assert!(RegExp::new("^(((hello$",config).is_err());
//!     assert!(regex.replace("hello world","good").unwrap()=="good world");
//! ```

use cxx::{let_cxx_string, Exception, UniquePtr};
use std::marker::PhantomData;
#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cpp_regexp/include/regex.hpp");
        type Regex;
        fn new_regex(
            s: &CxxString,
            icase: bool,
            nosubs: bool,
            optimize: bool,
            collate: bool,
            grammar: usize,
            format_no_copy: bool,
            format_first_only: bool,
            format_sed: bool,
            match_not_bol: bool,
            match_not_eol: bool,
            match_not_bow: bool,
            match_not_eow: bool,
            match_any: bool,
            match_not_null: bool,
            match_continuous: bool,
            match_prev_avail: bool,
        ) -> Result<UniquePtr<Regex>>;
        fn test(&self, s: &CxxString) -> Result<bool>;
        fn replace(&self, s: &CxxString, replacement: &CxxString) -> Result<UniquePtr<CxxString>>;
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum Grammar {
    ECMAScript = 0,
    BasicPOSIX = 1,
    ExtendedPOSIX = 2,
    Awk = 3,
    Grep = 4,
    Egrep = 5,
}

#[derive(Copy, Clone, PartialEq)]
pub enum ReplaceRule {
    ECMAScript = 0,
    Sed = 1,
}

#[derive(Copy, Clone)]
pub struct Config {
    pub icase: bool,
    pub nosubs: bool,
    pub optimize: bool,
    pub collate: bool,
    pub grammar: Grammar,
    pub format_no_copy: bool,
    pub format_first_only: bool,
    pub match_not_bol: bool,
    pub match_not_eol: bool,
    pub match_not_bow: bool,
    pub match_not_eow: bool,
    pub match_any: bool,
    pub match_not_null: bool,
    pub match_continuous: bool,
    pub match_prev_avail: bool,
    pub replace_rule: ReplaceRule,
}

impl Default for Config {
    #[inline]
    fn default() -> Config {
        Config {
            icase: false,
            nosubs: false,
            optimize: false,
            collate: false,
            grammar: Grammar::ECMAScript,
            format_no_copy: false,
            format_first_only: false,
            match_not_bol: false,
            match_not_eol: false,
            match_not_bow: false,
            match_not_eow: false,
            match_any: false,
            match_not_null: false,
            match_continuous: false,
            match_prev_avail: false,
            replace_rule: ReplaceRule::ECMAScript,
        }
    }
}

pub struct RegExp<'a> {
    exp: PhantomData<&'a ()>,
    regexp: UniquePtr<ffi::Regex>,
}

impl<'a> RegExp<'a> {
    #[inline]
    pub fn new(exp: &'a str, config: Config) -> Result<RegExp<'a>, Exception> {
        let_cxx_string!(scxx = exp);
        let c = config;
        Ok(RegExp {
            exp: PhantomData,
            regexp: ffi::new_regex(
                &scxx,
                c.icase,
                c.nosubs,
                c.optimize,
                c.collate,
                c.grammar as usize,
                c.format_no_copy,
                c.format_first_only,
                c.replace_rule == ReplaceRule::Sed,
                c.match_not_bol,
                c.match_not_eol,
                c.match_not_bow,
                c.match_not_eow,
                c.match_any,
                c.match_not_null,
                c.match_continuous,
                c.match_prev_avail,
            )?,
        })
    }
    #[inline]
    pub fn test(&self, s: &str) -> Result<bool, Exception> {
        let_cxx_string!(scxx = s);
        ffi::Regex::test(&self.regexp, &scxx)
    }
    #[inline]
    pub fn replace(&self, s: &str, replacement: &str) -> Result<String, Exception> {
        let_cxx_string!(scxx = s);
        let_cxx_string!(replacementcxx = replacement);
        Ok(ffi::Regex::replace(&self.regexp, &scxx, &replacementcxx)?.to_string())
    }
}
