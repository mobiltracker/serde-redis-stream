#[test]
fn test_base() {
    let t = trybuild::TestCases::new();
    t.pass("tests/it_compiles.rs");
}

#[test]
fn test_option() {
    let t = trybuild::TestCases::new();
    t.pass("tests/it_compiles_option.rs");
}
