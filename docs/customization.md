# Customization of `mutagen`

The behavior of `mutagen` and the attribute `#[mutate]` can be customized by adding arguments.

## Examples

```rust
// only mutate int-literals
#[mutate(only(lit_int))]

// only mutate int-literals and statements
#[mutate(only(lit_int, stmt))]

// include all mutations except statement mutations
#[mutate(not(stmt))]
```

## Format

WIP: specify format formally

### Options intended for testing `mutagen`

```rust
// assign mutation ids independently from the rest of the program
#[mutate(conf(local))]
```

## Known Transformers

Each implemented mutator has a corresponding transformer. The following table shows the mapping between names of the transformers and their mutators.

| Transformer name | Mutator |
| `lit_int` | `MutatorLitInt` |
| `lit_bool` | `MutatorLitBool` |
| `stmt` | `MutatorStmt` |

## `#[mutate]` arguments

should be possible:

* mutagen configuration (`local` vs `global`)
* define list of mutators
* configure each mutator

should be easy:

* leave out a single mutator
* take only one mutator
* configure a single mutator but do not change the list of mutators

examples:

```
#[mutate(conf(local))]
#[mutate(only(lit_int))]
#[mutate(not(stmt))]
#[mutate(not(early_return), stmt(keep_return), lit_int(x+1))]
```
