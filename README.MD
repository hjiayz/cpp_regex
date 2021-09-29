# cpp_regexp

cpp std::regex
- [x] test
- [x] replace
- [ ] match
- [ ] match_all
```rust
    use cpp_regexp::{RegExp,Config};
    assert!(RegExp::new("^hello$",Default::default()).unwrap().test("hello").unwrap());
    let match_results = RegExp::new("^(he)(ll)(o)$",Default::default()).unwrap().regex_match("hello").unwrap();
    assert!(match_results==["hello","he","ll","o"]);
    let config = Config{
        icase:true,
        //UTF-8 only
        locale:"es_US.UTF-8",
        ..Default::default()
    };
    let mut regex = RegExp::new("hello♥",config).unwrap();
    assert!(regex.test("hellO♥").unwrap());
    assert!(RegExp::new("^(((hello$",config).is_err());
    assert!(regex.replace("hello♥ world","good").unwrap()=="good world");
    let config_es = Config{
        icase:true,
        locale:"es_ES.UTF-8",
        ..Default::default()
    };
    let mut regex = RegExp::new("ñ",config_es).unwrap();
    //not single character
    assert!(regex.test("Ñ").unwrap()==false);
```

License: MIT OR Apache-2.0
