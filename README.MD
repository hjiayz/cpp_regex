# cpp_regexp

cpp std::regex
test only
```rust
    use cpp_regexp::{RegExp,Config};
    assert!(RegExp::new("^hello$",Default::default()).test("hello"));
    let config = Config{
        icase:true,
        ..Default::default()
    };
    let mut regex = RegExp::new("^hello$",config);
    assert!(regex.test("hellO"));
```

License: MIT OR Apache-2.0
