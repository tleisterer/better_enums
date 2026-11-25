#![allow(clippy::all, warnings)]

#[test]
fn compile_errors() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile_errors/**/*.rs");
}
