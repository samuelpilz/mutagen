mod common;

use common::*;
use mutagen_preview_transform::mutate;

#[mutate]
fn lit_u32() -> u32 {
    1 + 2
}

#[mutate]
fn lit_u8() -> u8 {
    1u8 + 2
}

#[mutate]
fn lit_usize_suffixed() -> usize {
    444usize
}

#[test]
fn test1() {
    test_with_mutation_id(0, || {
        assert_eq!(lit_u32(), 3);
    })
}
