#![allow(clippy::assign_op_pattern, unused)]
#[test]
fn compile_errors() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile_errors/**/*.rs");
}
