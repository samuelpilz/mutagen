mod common;

use common::*;
use mutagen_preview_transform::mutate;

#[mutate(conf(local), only(stmt))]
fn add_one() -> u32 {
    let mut x = 1;
    x += 1;
    x
}

#[test]
fn add_one_inactive() {
    test_with_mutation_id(0, || {
        assert_eq!(add_one(), 2);
    })
}

#[test]
fn add_one_active() {
    test_with_mutation_id(1, || {
        assert_eq!(add_one(), 1);
    })
}
