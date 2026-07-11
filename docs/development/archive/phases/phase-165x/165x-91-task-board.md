# 165x-91: escape barrier vocabulary task board

## Board

- [x] `165xA` docs lock
  - phase README
  - SSOT
  - root/current pointers
- [x] `165xB` API cut
  - add `src/mir/escape_barrier.rs`
  - export the vocabulary from `src/mir/mod.rs`
- [x] `165xC` consumer switch
  - make `src/mir/passes/escape.rs` consume the new classifier
- [x] `165xD` proofs
  - classifier unit tests
  - escape integration tests for method receiver and `FieldSet.value`
- [x] `165xE` verification
  - targeted `cargo test`
  - `git diff --check`

## Notes

- keep this phase narrow
- do not mix cross-block escape reasoning into this task series
