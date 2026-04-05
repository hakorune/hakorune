# Phase 113x: kernel vs vm-reference cluster wording correction

- 目的: `kernel` を `nyash_kernel` に限定し、`lang/src/vm` を `VM/reference cluster` として固定する。
- 対象:
  - `crates/nyash_kernel/README.md`
  - `lang/src/vm/README.md`
  - `docs/development/architecture/selfhost_execution_ssot.md`
- success:
  - `nyash_kernel` は native/product runtime kernel と明示される
  - `lang/src/vm` は product kernel ではなく VM/reference cluster と明示される
  - current pointers がこの wording correction lane に揃う
