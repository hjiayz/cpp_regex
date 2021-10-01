#![recursion_limit = "256"]

//! cpp std::regex
//! - [x] test
//! - [x] replace
//! - [x] match
//! - [x] match_all
//! ```
//!     use cpp_regexp::{RegExp,Config};
//!     assert!(RegExp::new("^hello$",Default::default()).unwrap().test("hello").unwrap());
//!     let match_results = RegExp::new("^(he)(ll)(o)$",Default::default()).unwrap().regex_match("hello").unwrap();
//!     assert!(match_results==["hello","he","ll","o"]);
//!     let hello2 = "hello".repeat(2);
//!     let match_all_results = RegExp::new("(he)(ll)(o)",Default::default()).unwrap().match_all(&hello2).unwrap();
//!     assert!(match_all_results[0]==["hello","he","ll","o"]);
//!     assert!(match_all_results.len()==2);
//!     assert!(match_all_results.iter().collect::<Vec<_>>()==[["hello","he","ll","o"],["hello","he","ll","o"]]);
//!     let config = Config{
//!         icase:true,
//!         //UTF-8 only
//!         locale:"es_US.UTF-8",
//!         ..Default::default()
//!     };
//!     let mut regex = RegExp::new("hello♥",config).unwrap();
//!     assert!(regex.test("hellO♥").unwrap());
//!     assert!(RegExp::new("^(((hello$",config).is_err());
//!     assert!(regex.replace("hello♥ world","good").unwrap()=="good world");
//!     let config_es = Config{
//!         icase:true,
//!         locale:"es_ES.UTF-8",
//!         ..Default::default()
//!     };
//!     let mut regex = RegExp::new("ñ",config_es).unwrap();
//!     //not single character
//!     assert!(regex.test("Ñ").unwrap()==false);
//!     //but you can
//!     assert!(regex.test(&"Ñ".to_lowercase()).unwrap());
//! ```

use cxx::{let_cxx_string, Exception, UniquePtr};
use std::marker::PhantomData;

#[cxx::bridge]
mod ffi {
    struct MatchGroup<'a> {
        pub items: Vec<&'a str>
    }
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
            locale: &CxxString,
        ) -> Result<UniquePtr<Regex>>;
        fn test(self: &Regex, s: &CxxString) -> Result<bool>;
        fn replace(self: &Regex, s: &CxxString, replacement: &CxxString) -> Result<String>;
        fn regex_match<'b>(self: &Regex, s: &'b str) -> Result<MatchGroup<'b>>;
        fn match_all<'b>(self: &Regex, s: &'b str) -> Result<Vec<MatchGroup<'b>>>;
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
pub struct Config<'a> {
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
    pub locale: &'a str,
}

impl Default for Config<'static> {
    #[inline]
    fn default() -> Config<'static> {
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
            locale: "en_US.UTF-8",
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
        let_cxx_string!(locale = c.locale);
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
                &locale,
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
        ffi::Regex::replace(&self.regexp, &scxx, &replacementcxx)
    }
    #[inline]
    pub fn regex_match<'b>(&self, s: &'b str) -> Result<MatchGroup<'b>, Exception> {
        ffi::Regex::regex_match(&self.regexp, &s)
    }
    #[inline]
    pub fn match_all<'b>(&self, s: &'b str) -> Result<Vec<MatchGroup<'b>>, Exception> {
        ffi::Regex::match_all(&self.regexp, s)
    }
}

pub use ffi::{MatchGroup};

impl<'a> MatchGroup<'a> {
    #[inline]
    pub fn len(&self) -> usize {
        self.items.len()
    }
    #[inline]
    pub fn into_iter(self) -> std::vec::IntoIter<&'a str> {
        self.items.into_iter()
    }
    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_,&'a str> {
        self.items.iter()
    }
}

impl<'a> std::ops::Index<usize> for MatchGroup<'a> {
    type Output = str;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        self.items[index]
    }
}

impl<S: AsRef<str>> PartialEq<&[S]> for MatchGroup<'_> {
    #[inline]
    fn eq(&self, other: &&[S]) -> bool {
        self.iter().zip(other.iter()).all(|(&a, b)| a == b.as_ref())
    }
}

impl<S: AsRef<str>> PartialEq<[S]> for MatchGroup<'_> {
    #[inline]
    fn eq(&self, other: &[S]) -> bool {
        self.iter().zip(other.iter()).all(|(&a, b)| a == b.as_ref())
    }
}

impl<S: AsRef<str>> PartialEq<[S]> for &MatchGroup<'_> {
    #[inline]
    fn eq(&self, other: &[S]) -> bool {
        self.iter().zip(other.iter()).all(|(&a, b)| a == b.as_ref())
    }
}

impl<S: AsRef<str>, const N: usize> PartialEq<[S; N]> for MatchGroup<'_> {
    #[inline]
    fn eq(&self, other: &[S; N]) -> bool {
        self.iter().zip(other.iter()).all(|(&a, b)| a == b.as_ref())
    }
}

impl<S: AsRef<str>, const N: usize> PartialEq<[S; N]> for &MatchGroup<'_> {
    #[inline]
    fn eq(&self, other: &[S; N]) -> bool {
        self.iter().zip(other.iter()).all(|(&a, b)| a == b.as_ref())
    }
}
