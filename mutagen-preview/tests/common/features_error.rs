#![cfg(not(feature = "self-test"))]
macro_rules! hide_from_rustfmt {
    ($mod:item) => {
        $mod
    };
}

hide_from_rustfmt! {
    "Hello! You want: `cargo test --all-features`"
}
// this ensures that a useful compiler message is printed in case the self-test feature is not enabled when running the tests.
// If the features are not enabled, the test cases do not compile.
//
// Without wrapped in a macro, `cargo fmt` would complain about invalid syntax
//
// this trick has been found in the test suite of the crate `syn`: https://github.com/dtolnay/syn/tree/master/tests/features
