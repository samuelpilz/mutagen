mod common;

use common::*;
use mutagen_preview::mutate;

// constant true
#[mutate]
fn simple_true() -> bool {
    true
}
#[test]
fn simple_true_inactive() {
    test_with_mutation_id(0, || {
        assert_eq!(simple_true(), true);
    })
}
#[test]
fn simple_true_active() {
    test_with_mutation_id(1, || {
        assert_eq!(simple_true(), false);
    })
}

// constant false
#[mutate]
fn simple_false() -> bool {
    false
}
#[test]
fn simple_false_inactive() {
    test_with_mutation_id(0, || {
        assert_eq!(simple_false(), false);
    })
}
#[test]
fn simple_false_active() {
    test_with_mutation_id(2, || {
        assert_eq!(simple_false(), true);
    })
}
