# Implemented Mutators

`mutagen` provides several mutators out of the box. The table below gives a rough overview of the implemented mutators.

| Mutator | when activated | example |
| -- | -- | -- |
| `MutatorLitBool` | inverts bool literals | `false` -> `true` |
| `MutatorLitInt` | mutates an integer literal | `1u8` -> `2u8`  |
| `MutatorStmt` | removes a statement | `x += 1` -> no-op |
