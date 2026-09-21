---
Status: closed__2026-09-21__WarningSelectedDynamicEmitterTestFacade
Task: MIRBUILDER-WARNING-SELECTED-DYNAMIC-EMITTER-TEST-FACADE-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i45-2026-09-21.md
Implementation permission: true for selected dynamic emitter test facades only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I46
---

# Warning cleanup: selected dynamic emitter test facade

## Six-line brief

```text
Decision: gate the selected dynamic emitter's representation/view/reject test
  imports and short assembly helper; retain the production ledger and _from_parts
  assembly path.
Source authority + canonical issuer: selected_dynamic_physical_emitter/mod.rs;
  assembly and value-ledger child modules remain the owners.
Non-authority: cargo-fix, wildcard imports, physical emitter redesign, ledger
  semantics, or warning guesses.
Fail-fast boundary: any production consumer of a gated name, compile error,
  changed test warning, or warning-count mismatch rejects the slice.
Smallest next slice: add cfg(test) to the three facade groups and run fixed gates.
Non-claims: no dynamic physical behavior change, suppression, or cutover.
```

## Preconditions and acceptance

I45 records three lib-only unused-import diagnostics in
`src/mir/builder/resolved_lowering/selected_dynamic_physical_emitter/mod.rs`:
representation, value view/reject, and the short assembly helper. Production
retains the ledger owner and `assemble_unpublished_selected_dynamic_w6_from_parts`.

Acceptance requires lib warnings to drop from **1,785 to 1,782**, lib-test
warnings to remain **561**, and both fixed commands to exit 0:

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
```

Only the three cfg annotations/splits may change.

## Closeout evidence

The sole permitted source edit gated the selected dynamic emitter's test-only
representation/view/reject imports and `assemble_unpublished_selected_dynamic_w6`
re-export. The production ledger and `_from_parts` export remain unconditional.
The fixed gates completed sequentially with exit 0:

* lib: 1,782 warnings — `/tmp/hakorune-warning-i0-selected-dynamic-emitter-test-facade-lib-20260921.log`
* lib test: 561 warnings — `/tmp/hakorune-warning-i0-selected-dynamic-emitter-test-facade-lib-test-20260921.log`

No dynamic physical behavior or production assembly route changed.
