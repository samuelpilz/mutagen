mod common;

use common::*;
use mutagen_preview_transform::mutate;

#[mutate]
fn main_fn() -> u32 {
    let mut x = 1;
    x += 1;
    x
}

#[test]
fn test1() {
    test_with_mutation_id(0, || {
        assert_eq!(main_fn(), 2);
    })
}
