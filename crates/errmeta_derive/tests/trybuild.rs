#[test]
fn compile_fail() {
  let t = trybuild::TestCases::new();
  t.compile_fail("tests/fails/*.rs");
}
