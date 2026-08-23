# Normal root execution pre-cutover split T0

Status: selected — fast BoxShape only
Date: 2026-08-23
Decision: NORMAL-ROOT-EXECUTION-PRE-CUTOVER-SPLIT-T0
Parent: NORMAL-ROOT-EXECUTION-ATOMIC-CUTOVER-MANIFEST-D0

## Six-line brief

Decision:
  Extract one inline test module without changing behavior or authority.
Source authority + canonical issuer:
  Unchanged; this row issues and consumes no semantic product.
Non-authority:
  File placement, `#[path]`, test names, hashes, and line counts.
Fail-fast boundary:
  Stop if production bytes, test-body bytes, callers, visibility, or behavior
  differ after the move.
Smallest next slice:
  Move only `main_expansion.rs` inline tests to `main_expansion_tests.rs`.
Non-claims:
  No root/source-plan design, classifier, lifecycle, Builder effect, fallback,
  production switch, or adjacent file cleanup.

## Change

```text
src/mir/builder/main_expansion.rs
  inline #[cfg(test)] mod tests
    -> src/mir/builder/main_expansion_tests.rs
```

Keep `with_test_main_static_children` in the production module. Replace only
the inline wrapper with:

```rust
#[cfg(test)]
#[path = "main_expansion_tests.rs"]
mod tests;
```

## Contract

- lines 1–456 of `main_expansion.rs` remain byte-identical;
- their SHA-256 remains
  `00ca453d9284261a08d5569b0c1781b2511d6df5a76a0748112fa811e25095c2`;
- the de-indented inner test-body SHA-256 remains
  `b7f3e1f5aa3244458af9bfe4754bede30b5faf130c742800ef38562b437ff3dd`;
- all eight test names, imports, visibility, and logical module identity remain;
- no other Rust file changes;
- both Rust files remain below 760 lines.

## Done

```text
test-name census before/after = 8 / 8
production-prefix hash = exact
test-body hash = exact
cargo test --profile quick --lib mir::builder::main_expansion::tests -- --test-threads=1
cargo check --profile quick
git diff --check
```

Commit and push this BoxShape row independently. Then set `work_mode` back to
`design_stop` unless the atomic C0 is explicitly selected from its frozen
manifest.

## Stop

- any semantic body edit or assertion update is needed;
- a caller/import/visibility change appears outside the module attachment;
- a baseline test fails at the parent commit;
- another over-limit file must be edited;
- C0 semantic work would enter the same commit.
