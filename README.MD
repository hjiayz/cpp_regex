# cpp_regexp

cpp std::regex
test only
```rust
    use cpp_regexp::RegExp;
    assert!(RegExp::new("^hello$").test("hello"));
    let mut regex = RegExp::new("^hello$");
    regex.icase = true;
    assert!(regex.test("hellO"));
```

License: MIT OR Apache-2.0
