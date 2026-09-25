# MIR-ROOT-MODULE-SURFACE0-G0 — manifest-backed root module surface guard

Parent: `mir-root-facade-contract-ssot.md` step 2
(`MIR-TOPOLOGY-REBASE0-P0` landed `646f363ff1` -> this row).

## Task (from facade SSOT)

Change: extend the existing root-facade guard family with a
manifest-backed no-unreviewed-growth check for public module
declarations.

Contract: preserve module visibility and behavior; the guard owns
surface drift only, never semantic acceptance or retirement policy.
Its checked manifest is the sole root public-module surface authority
after this row.

Done: root symbol exports and public module declarations each have one
stable, index-listed guard entry; current inventory is reproduced
exactly.

Stop: no another per-row shell guard; no suffix-based keep/retire
inference.

## Landing

- `tools/checks/mir_root_module_manifest.txt` created from the P0
  inventory: 215 module declarations (`name <TAB> visibility <TAB>
  gate`; gate ∈ none|test|feature|path-test). This is now the sole
  checked root module-declaration surface authority.
- `tools/checks/mir_root_facade_guard.sh` extended (same guard family,
  no new per-row script): second phase compares every `mod`
  declaration in `src/mir/mod.rs` — name, visibility, cfg gate —
  against the manifest; extra/missing/visibility-mismatch = drift error.
- `tools/checks/mir_root_facade_allowlist.txt`: `ConstructionTarget`
  added — it was already exported at HEAD (`pub use instruction::{…}`)
  but missing from the allowlist; reproducing the current inventory
  exactly is this row's Done condition.
- `docs/tools/check-scripts-index.md` index row added.

Verify: `bash tools/checks/mir_root_facade_guard.sh` →
`ok exports=129`, `ok modules=215`. Behavior unchanged — guard and
manifest only.
