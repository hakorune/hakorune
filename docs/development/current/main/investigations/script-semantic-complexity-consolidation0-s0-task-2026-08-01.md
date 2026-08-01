# SCRIPT-SEMANTIC-COMPLEXITY-CONSOLIDATION0-S0

Decision: Accept — one live, behavior-neutral refactor.

## Purpose

Before activating another Script family, reduce receipt-layer growth without
changing Complete/Deferred eligibility, lowering, diagnostics, or raw/reference
routes.

## One production path

```text
ModuleBuilderInvocationSessionV1::
  complete_normal_default_program_root_catalog_lifecycle
-> resolve_script_forest_with_declaration_views
-> VerifiedScriptSemanticSourceV1::seal_with_forest
```

This existing caller consumes every affected product; this is not a
caller-zero scaffold.

## Ordered work

1. Replace sibling receipt storage with private products:
   - `ScriptSemanticSourceCoreV1`: source, forest, projection, runtime coverage
   - `ScriptBoundaryReceiptPackV1`: outbox, StaticConst, Using, diagnostics
   - `ScriptOperationalDemandReceiptPackV1`: Record, enum variant, QMark, Match
   Existing narrow lowering getters remain unchanged. A generic AST-keyed bag is
   forbidden.
2. Split Script shadow outcomes before owner issue:
   - source deferral: unresolved/redeclaration/unsupported/invalid user shape
   - invariant rejection: duplicate receipts, duplicate observations, coverage
     mismatch, malformed Script root input
   Invariants must become `ScriptSemanticSeal` rejection with `RootLower = 0`;
   source deferrals keep the existing RootLower diagnostic authority.
3. Replace demand-window issuer-plus-validator handoff with a private sealed
   issued-entry constructor. It performs the exact AST/disposition proof once;
   ordinal storage only records that issued product and seals totality.
4. Split `normal_script_semantic_source_tests.rs` (currently 1031 lines) by
   existing responsibility, preserving test bodies and manifest anchors. Every
   touched source/check file must be below 800 lines.
5. Reduce current-pointer duplication to the active row/card; move closed
   consultation pointers to the workstream ledger.
6. Make the manifest/guard enforce the full pre-S0 Complete fixture identity
   floor and exact Deferred reason floor after test-anchor relocation.

## Acceptance

```text
Complete fixture identities/reasons = unchanged
Deferred fixture identities/reasons = unchanged
raw/reference route = unchanged
new semantic admission/lowering route = 0
catch-all Script Err(_) -> Deferred = 0
invariant corruption -> ScriptSemanticSeal, RootLower = 0
all source/check files touched < 800 lines
existing shared guard + pointer guard green
```

## Atomic old-edge deletion

```text
VerifiedScriptSemanticSourceV1 sibling receipt arrays/validation loops
catch-all Script shadow-error deferral
issuer -> independent validator handoff
monolithic semantic-source test file
stale active-row mirrors
```

## Hard stops

Do not land if it changes any admission identity, user diagnostic stage/order,
raw/reference behavior, or compatibility reachability; introduces a second
classifier, generic receipt map, new family, new guard, or a >=800-line touched
source/check file. Land one `refactor(mir)` commit with the compact card update;
do not split select/close docs ceremony.

## Closeout

Closed by the implementation-coupled S0 commit. The next action is a fresh
named-family D0; no historical family is preselected.
