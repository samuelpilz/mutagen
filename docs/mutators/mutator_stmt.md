# `MutatorStmt`

## Target Code

* statements ending with a semicolon
    * containing `return`
    * not a `let`-binding

## Mutations

1. remove the statement

## Limitations

Removing statements containing `return` may result in compiler errors since it is required that all computation paths. However, when the `return` is not necessary for the termination of the function, removing the statement would be a correct mutation. These are not considered by this mutator.
