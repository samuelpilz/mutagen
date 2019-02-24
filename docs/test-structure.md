# Test Plan

`mutagen`'s logic is tested using unit tests and integration tests. The integration tests need a

## Unit tests

The behavior of all mutators is tested using traditional unit tests in the corresponding modules. The mutators are run with run-time configurations constructed by the test itself.

## Integration tests

The behavior of the `#[mutate]` attribute is tested using integration tests in the `tests` folder.

### Test Isolation

Instead of having a program-wide unique id per mutation, each function can have their own local mutation ids by writing adding `conf(local)` to the arguments of `#[mutate]`. This ensures that each test is independent from others. Moreover, only one transformer is enabled in each test. This is done by adding `only(...)` to the mutagen arguments.

### Exhaustive Testing

Each test sets the global `mutation_id` for the single test run and runs test code. This enables exhaustive testing of all mutations within a single run of the test suite and without dependency on external environment variables.

### Implementation Details

The setting of the `mutation_id` during test is possible via special functions that mutate the global run-time configuration and are only available when enabling the feature `self-test` of `mutagen`. The feature `self-test` is not supposed to be used by users of `mutagen`.

The feature `self-test` can be enabled by calling `cargo test --all-features`. If the feature is not enabled, the compiler will issue errors, giving the error message "Hello! You want: `cargo test --all-features`". Without the feature enabled, the tests would not compile since the functions required for mutating the state are not present.

## Example

Typically, a complete test setup looks like this
```rust
// only enable mutator `xyz`
#[mutate(conf(local), only(xzy))]
pub fn x() {
    // function to mutate
}

#[test]
pub fn x_inactive() {
    test_with_mutation_id(0, || {
        // test and assert on `x()` where no mutations have been performed
    })
}
#[test]
pub fn x_active() {
    test_with_mutation_id(1, || {
        // test and assert that the correct mutation has been performed in `x()`
    })
}

// more tests with other mutation ids, if more than one mutation has been performed
```
