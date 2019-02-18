mod common;

use common::*;
use mutagen_preview_transform::mutate;

#[mutate]
fn main_fn() -> u32 {
    1 + 2
}

#[test]
fn test1() {
    test_with_mutation_id(0, || {
        assert_eq!(main_fn(), 3);
    })
}
