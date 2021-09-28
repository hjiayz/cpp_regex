# cpp_regexp

cpp std::regex
- [x] test
- [x] replace
- [ ] match
- [ ] match_all
```rust
    use cpp_regexp::{RegExp,Config};
    assert!(RegExp::new("^hello$",Default::default()).unwrap().test("hello").unwrap());
    let config = Config{
        icase:true,
        ..Default::default()
    };
    let mut regex = RegExp::new("hello",config).unwrap();
    assert!(regex.test("hellO").unwrap());
    assert!(RegExp::new("^(((hello$",config).is_err());
    assert!(regex.replace("hello world","good").unwrap()=="good world");
```

License: MIT OR Apache-2.0
