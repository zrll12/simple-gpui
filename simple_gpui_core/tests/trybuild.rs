#[test]
fn component_macro_compile_guards() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/fail/*.rs");
    t.pass("tests/success/*.rs");
}
