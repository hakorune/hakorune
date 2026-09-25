# MIR-TOPOLOGY-REBASE0-P0 — src/mir root module inventory census

Parent: `mir-root-facade-contract-ssot.md` parked post-Loop order step 2
(`MIR-TOPOLOGY-REBASE0-P0` -> `MIR-ROOT-MODULE-SURFACE0-G0`), reached via
`repo-physical-structure-cleanup-ssot.md` integrated order after Loop
authority convergence (M12 + C0 landed).

## Scope boundary

```text
起点: src/mir/mod.rs at HEAD (post-C0, f8f76d5ade..)
終点: one machine-readable root-module inventory
includes: every `mod` / `pub use` declaration in src/mir/mod.rs, the
          inline `compile_target_capability` module, cfg(test)/cfg(feature)
          gated declarations
excludes: nested module contents (owned by their own module trees),
          docs-only references, non-mir crates
```

## Task (from facade SSOT)

Change: reproduce the local file/module/export census and emit one
machine-readable root-module inventory. Delete or move nothing.

Contract: classify declarations by visibility, owner family, active
caller class, and lifecycle (`durable / temporary / bootstrap-compat /
internal / retire-candidate`). A name is never deletion authority.

Done: every declaration in the rebase snapshot has one owner and one
lifecycle class; `pub use` and `pub mod` are reported separately. The
report is evidence, not the lasting surface authority.

Stop: any unresolved owner, generated declaration, or active production
caller returns the row to classification. No folder design begins here.

## Inventory

`docs/development/current/main/design/fixtures/mir-root-module-inventory-p0-v1.tsv`

Columns: `decl_kind` (mod|inline_mod|use|fn), `name`, `visibility`
(pub|pub(crate)|private|cfg-test|cfg-feature), `owner_family`,
`caller_class` (production|test-only|facade-reexport|none),
`lifecycle`, `evidence`.

## P0 landing — inventory complete

285 declaration rows in `mir-root-module-inventory-p0-v1.tsv`:

- decl_kind: 214 file/dir `mod`, 1 `inline_mod`
  (`compile_target_capability`), 69 `use` re-export statements
  (56 `pub`, 13 `pub(crate)`), 1 `cfg(test)` fn (`test_global_target`).
- visibility (mod decls): 130 `pub`, 77 `pub(crate)`, 3 `private`,
  4 `cfg-test`, 1 `cfg-feature` (`aot_plan_import`).
- caller_class: 202 production, 77 facade-reexport, 2 brace-import,
  3 internal-test (the cfg(test) harness modules themselves), 1 `none`.
- lifecycle: 260 durable, 15 temporary (`*_micro_seed_plan` /
  `*_seed_plan` / `*_pilot` temporary seed bridges per owner comments),
  5 internal-test, 3 internal (`compiler`, `printer_helpers`,
  `spanned_instruction`), 2 bootstrap-compat (`string_corridor_compat`,
  `string_corridor_names`).

Findings:

- `dynamic_operator_contract` is the only `caller_class=none` durable
  module — a sealed prepared contract (`bea8a285fe`) owning the Dynamic
  Add/Less execution envelope with no production consumer wired yet.
  Recorded as evidence, not deletion authority (facade SSOT: a name is
  never deletion authority).
- Caller detection required three passes: `crate::mir::name` /
  `mir::name::` paths, `crate::mir::{...}` brace imports, and
  `use super::{...}` sibling imports from `src/mir/*.rs` files
  (balanced-brace parse — nested `::{...}` groups broke naive regex).
- No file moved or deleted; `cargo check --profile quick --lib`
  unaffected (inventory + docs only).
