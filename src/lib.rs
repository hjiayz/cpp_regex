#![recursion_limit="256"]

//! cpp std::regex
//! test only
//! ```
//!     use cpp_regexp::RegExp;
//!     assert!(RegExp::new("^hello$").test("hello"));
//!     let mut regex = RegExp::new("^hello$");
//!     regex.icase = true;
//!     assert!(regex.test("hellO"));
//! ```

use cpp::cpp;

cpp!{{
    #include <iostream>
    #include <string>
    #include <regex>
}}

#[derive(Copy,Clone)]
pub enum Grammar{
    ECMAScript = 0,
    BasicPOSIX = 1,
    ExtendedPOSIX = 2,
    Awk = 3,
    Grep = 4,
    Egrep = 5,
}

pub struct RegExp<'a>{
    exp:&'a str,
    pub icase:bool,
    pub nosubs:bool,
    pub optimize:bool,
    pub collate:bool,
    pub kind:Grammar,
}

impl<'a> RegExp<'a>{
    pub fn new(exp:&'a str)->RegExp<'a>{
        RegExp{
            exp,
            icase:false,
            nosubs:false,
            optimize:false,
            collate:false,
            kind:Grammar::ECMAScript,
        }
    }
    pub fn test(&self,s:&str)->bool{
        let regexp = std::ffi::CString::new(self.exp).unwrap();
        let regexp_ptr = regexp.as_ptr();
        let s = std::ffi::CString::new(s).unwrap();
        let s_ptr = s.as_ptr();
        let kind:usize = self.kind as usize;
        let icase = self.icase;
        let nosubs = self.nosubs;
        let optimize = self.optimize;
        let collate = self.collate;
        unsafe {
            cpp!([regexp_ptr as "const char *",
                    s_ptr as "const char *",
                    icase as "bool",
                    nosubs as "bool", 
                    optimize as "bool", 
                    collate as "bool", 
                    kind as "size_t"] -> bool as "bool" {
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
                std::regex regex(regexp_ptr,flag);
                return std::regex_search(s_ptr,regex);
            })
        }
    }
}
