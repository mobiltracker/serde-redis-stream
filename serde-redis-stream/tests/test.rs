#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/it_compiles.rs");
}
