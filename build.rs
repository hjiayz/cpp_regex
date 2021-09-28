fn main() {
    cxx_build::bridge("src/lib.rs") // returns a cc::Build
        .file("src/regex.cc")
        .flag_if_supported("-std=c++11")
        .compile("cpp_regexp");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/regex.cc");
    println!("cargo:rerun-if-changed=include/regex.hpp");
}
