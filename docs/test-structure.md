# Test Plan

`mutagen`'s logic is tested using unit tests and integration tests. The integration tests need a

## Unit tests

The behavior of all mutators is tested using traditional unit tests in the corresponding modules. The mutators are run with run-time configurations constructed by the test itself.

## Integration tests

The behavior of the `#[mutate]` attribute is tested using integration tests in the `tests` folder.

Each test sets the global `mutation_id` for the single test run and runs test code. This is possible via special functions that mutate the global run-time configuration and are only available when enabling the feature `self-test` of `mutagen`. The feature `self-test` is not supposed to be used by users of `mutagen`.

Setting the `mutation_id` in every test-case enables exhaustive testing of the `#[mutate]` macro within a single run of the test suite. The behavior of all mutations inserted into the code can be tested within a single run of the test suite. A global lock for the integration tests ensures that all integration tests are run sequentially and do not interfere with each other.

The feature `self-test` can be enabled by calling `cargo test --all-features`. If the feature is not enabled, the compiler will issue errors, giving the error message "Hello! You want: `cargo test --all-features`". Without the feature enabled, the tests would not compile since the functions required for mutating the state are not present.

Unfortunately, the integration tests as currently implemented are quite brittle, as each added or removed mutation requires the `mutation_id`s of each test to be adapted as well.

WIP: transformer-level test isolation in integration tests
