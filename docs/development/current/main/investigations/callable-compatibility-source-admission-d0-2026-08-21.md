---
Status: accepted design stop — tooling guard landed; cohort state census remains NoSafeSlice
Date: 2026-08-21
Decision: CALLABLE-COMPATIBILITY-COHORT-STATE-CENSUS-D0
Parent: docs/development/current/main/investigations/mirbuilder-compatibility-seam-final-ratchet-d0-2026-08-21.md
ProductionCaller: existing parser/macro, MIR, LLVM, Wasm, public-AST, JSON, VM, and REPL compatibility entrances; no new caller
ReplacementCell: select one source-backed compatibility cohort only after its issuer and named consumer are proven; otherwise retain the typed compatibility route
Classification: design stop; no new accepted source shape, semantic receipt, or production route change
Execution row: CALLABLE-COMPATIBILITY-COHORT-STATE-CENSUS-D0
---

# ROUTING-CLASSIFICATION-COMPLETENESS-GUARD-P1

## Six-line brief

Decision: Keep every compatibility reason on the explicit AST lane until one
cohort has a source-product issuer, one named semantic consumer, and complete
negative coverage; do not let an AST-only request silently become a semantic
package. The reusable classification-completeness guard is already landed;
the census must now close one cohort or retain `NoSafeSlice`.

Source authority + canonical issuer: the parser postpass and
`NormalCallableTransformOutcomeV1` issue the SourceBacked/Compatibility origin
and its typed reason. Runner/request adapters may transport that origin, but
they may not recreate it from an AST, name, ordinal, or successful raw compile.

Non-authority: `NormalCallableSemanticPackageMode::Compatibility`, dropped
`reason` fields, public AST APIs, Program JSON/VM/REPL inputs, AST spans/names,
`UnlocatedCompatibility`, `is_brand_declared`, raw success, and compatibility
warnings cannot issue source identity, a resolver ledger, or a direct-call
target.

Fail-fast boundary: immediately after parser/macro materialization and before
semantic-package issuance, Builder effects, child argument descent, or
physical publication. Missing/foreign/ambiguous source origin is `Rejected`;
all currently unsupported cohorts remain `TypedCompatibility`, never inferred
SourceBacked.

Smallest next slice: `CALLABLE-COMPATIBILITY-COHORT-STATE-CENSUS-D0`, a
read-only finite census that either names one cohort's source issuer and
production consumer or records the missing boundary as `NoSafeSlice`. It must
not admit a cohort, issue a semantic package, or inspect historical cards as
authority. The transport P0 and classification-completeness tooling slice are
already closed.

Non-claims: no parser grammar change, no new accepted callable shape, no Brand
cutover, no FunctionCall classifier, no raw retirement, no AST reparse, no
resolver/Recipe/Join/physical Call, no ABI/backend change, and no performance
or promotion claim.

## Classification-completeness receipt

Every current entrance must map to exactly one state before Builder effects.
The `Neither` row is intentional: a request that is not a callable-source
candidate must not be disguised as an empty semantic package.

| state | authority / issuer | before effects | allowed terminal | fallback |
|---|---|---|---|---|
| `SourceBacked` | parser postpass → `VerifiedFinalCallableProgramSourceV1` | retain source identity and exact callable source handoff | installed semantic-package owner | no Compatibility downgrade |
| `TypedCompatibility` | parser/macro transform → `NormalCallableTransformCompatibilityV1` | retain cohort/reason; do not issue semantic rows | existing compatibility-only AST owner | no SourceBacked inference |
| `Unavailable` | public AST/JSON/VM/REPL entrance with no parser source product | no source-backed claim or semantic effects | explicit compatibility-only owner | never fabricate a parser reason or package |
| `Neither` (`NoCandidate`) | lane admission proves this request is outside callable-source scope | no callable package or child effects | caller-owned non-callable/no-candidate terminal | never `Complete(empty)` or raw fallback |
| `Rejected` | parser/source-lineage/identity/transform validator | typed freeze before resolver/Builder effects | stable rejection terminal | no retry, AST re-scan, or compatibility recovery |

`Discarded` is a later candidate/session terminal, not an admission state; its
owner must discard the whole isolated candidate and must not re-enter
Compatibility. `Deferred` from the legacy semantic admission card is also not
a Compatibility state; it is parked test/canary-only and any production use is
`NoSafeSlice`.

## Current caller census

The current tree has multiple compatibility entrances, but they do not all
carry the same authority:

- `src/runner/modes/mir.rs` materializes the parser/macro outcome, then drops
  `reason` while converting `Compatibility { ast, .. }` to
  `NormalCompileRequestV1::for_mir_mode`.
- `src/runner/product/llvm/mir_compiler.rs` performs the same conversion for
  LLVM and `compile(ast, ...)` accepts a public AST directly through
  `for_llvm_source`.
- `src/runner/product/wasm.rs` accepts a parsed AST directly through
  `for_wasm_source`.
- `src/mir/compiler/mod.rs::compile_with_source` and
  `compile_with_source_and_imports` accept public AST directly through
  `for_mir_mode`.
- Program JSON, VM, REPL, and AST-only helpers use compatibility requests with
  no parser-issued source handoff.
- The normal root lifecycle chooses
  `NormalCallableSemanticPackageMode::Installed(package)` only when the
  source-backed package was issued; otherwise it explicitly chooses
  `Compatibility`. That mode is a route selector, not a source authority.

The exact reason variants currently issued by the parser/macro boundary include
parser compatibility cohorts plus `DefaultDeriveWouldGenerateCallable` and
`RegisteredMacroBox`. They must remain typed and source-bound. A future package
admission may select one cohort only after a separate BoxCount/BoxShape
decision; this D0 does not select one.

## Transport decision for the next P0

The parser handoff and macro transform each own a different fact and must not
be made responsible for the other:

- `NormalParserCallableSourceHandoffV1` owns parser disposition and
  `NormalParserSourceLineageV1`.
- `NormalCallableTransformOutcomeV1` owns the macro/parser compatibility reason.
- `NormalCallableMaterializationErrorV1` owns parse/lineage/transform failure,
  not successful compatibility identity.

The next P0 may therefore introduce one private, non-semantic transport
aggregate (candidate name `NormalCallableCompatibilityOriginV1`) containing
the existing typed reason plus the existing parser lineage. Its issuer is the
materialization seam only after both owners have issued their facts. It is moved
with the compatibility AST through the request/root boundary and is consumed
only as a compatibility diagnostic/provenance input. It must be non-Clone,
non-reconstructible from AST/name/ordinal, and incapable of constructing a
resolver ledger, Recipe, Join, target, or physical Call.

The P0 must not put the aggregate into the parser handoff (which would make the
parser a macro-reason issuer), nor into `NormalCallableSemanticPackageMode`
(which is only a route selector). Public AST/JSON/VM/REPL requests remain
`Unavailable` without this carrier.

## D0 census result

The current source shows the exact loss points:

- `src/runner/modes/common_util/normal_callable.rs:69-73` drops parser
  lineage on the Compatibility arm;
- `src/runner/modes/mir.rs:69-79` and
  `src/runner/product/llvm/mir_compiler.rs:54-58` bind the reason as
  `reason: _reason`;
- `NormalCompileRequestV1` has no compatibility-origin field;
- `PreparedNormalDefaultProgramRootV1::Compatibility(ASTNode)` has no
  reason/lineage carrier;
- the live root chooses `NormalCallableSemanticPackageMode::Compatibility`
  when no installed package exists, but that mode cannot repair the lost
  source facts.

No source/name/ordinal re-pairing or second semantic issuer is acceptable at
any of these seams.

## Design questions

1. Which runner/request boundary will retain the typed compatibility reason and
   parser lineage without cloning or reparsing the AST?
2. Which public AST/JSON/VM/REPL entrances are intentionally `Unavailable`,
   and which are explicit `Neither` because they are outside the callable lane?
3. Can the existing compatibility owner consume the preserved reason without
   changing accepted syntax or creating a second semantic authority?
4. Is any cohort actually ready for a later semantic package, or must all
   cohorts remain compatibility-only until a source product is issued?
5. Which source/identity drift cases reject before resolver/Builder effects,
   and how will the guard prove that no reason is silently dropped?

## Acceptance and stop line

This D0 is accepted with the transport-only boundary above:

- every current entrance is mapped to one finite state above, including
  `Neither`, `Unavailable`, and source-loss `Rejected`;
- parser/macro reason and source lineage are named as the sole source authority;
- runner/request adapters are recorded as transport-only and their current
  reason-dropping holes are listed as the next `...SOURCE-TRANSPORT-P0`
  implementation boundary;
- no compatibility state can become SourceBacked, an installed package,
  `Complete(empty)`, or raw fallback by default;
- source drift, foreign lineage, duplicate/ambiguous cohort, and missing reason
  stop before resolver/Builder/child effects;
- no code, fixture, semantic receipt, production switch, or physical consumer
  was added in this design stop;
- any future implementation row names one cohort, one owner, one production
  caller, positive/negative evidence, a focused gate, a reusable guard, and
  the old compatibility edge it retires.

## Cohort admission audit — NoSafeSlice

The transport P0 is closed, but no current compatibility cohort is eligible
for semantic-package admission:

The parked admission row remains
`CALLABLE-COMPATIBILITY-COHORT-STATE-CENSUS-D0`; this tooling slice does not
advance or replace that decision.

| cohort family | current issuer/evidence | admission result | missing boundary |
|---|---|---|---|
| `TypedCompatibility(Parser(...))` | parser postpass compatibility rows; no source seal and parameter disposition is `SelectedBuildGateUnsupported` | remain AST compatibility | parser-backed callable source product and one consumer |
| `TypedCompatibility(DefaultDeriveWouldGenerateCallable)` | macro decision after `initial.into_ast()` | remain AST compatibility | macro-after source-anchor issuer; registered/default policy co-seal |
| `TypedCompatibility(RegisteredMacroBox)` | environment/registry-dependent macro decision after AST move | remain AST compatibility | stable source membership and deterministic semantic issuer |
| `SourceBacked` | parser final source plus normal transform | existing installed package | none; not a compatibility cohort |
| `Unavailable` / `Neither` | source-free or outside callable lane | no package claim | no source authority by contract |
| `Rejected` / `Discarded` | validator or isolated candidate/session owner | typed freeze or whole-candidate discard | no retry, recovery, or raw fallback |

The parser compatibility families (`InterfaceBox`, `RecordBox`, `MixedProgram`,
`TopLevelBuildGate`, `NoBoxDeclarations`, `NonProgram`, and
`UnsupportedCallableSource`) are not interchangeable. The existing parser
admission rejects source-seal issuance for the unsupported families, and the
macro reasons do not reissue callable anchors after the AST is moved. A
successful compatibility compile, AST/name/ordinal pairing, parser lineage,
or `NormalCallableCompatibilityOriginV1` is not a semantic issuer.

Therefore the current development state is `NoSafeSlice`, not a hidden
implementation row. The current task is the finite state/caller census only;
it must close the missing issuer/consumer boundary before any `Verified*`,
`Prepared*`, semantic package, fallback retirement, or production switch is
opened. The general classification-completeness rule in
`agent-current-entry-contract-ssot.md` remains the review authority.

## Accepted tooling slice — ROUTING-CLASSIFICATION-COMPLETENESS-GUARD-P1

Decision: Add one reusable, read-only guard that resolves only
`CURRENT_STATE.toml.latest_card_path` and checks the active card's finite table.

Source authority + canonical issuer: `CURRENT_STATE.toml` selects the one
active card; the card author owns state vocabulary and its issuer prose. The
guard only validates the table contract and never issues a compiler state.

Non-authority: historical cards, state-name guesses, AST/MIR facts, compiler
success, empty/default rows, and a guard pass cannot select a cohort or create
a semantic package.

Fail-fast boundary: before the guard reports success. Missing table headers,
missing data rows, missing neutral (`Unavailable`/`Absent`/`Unresolved`/
`Neither`/`NoCandidate`) state, or missing `NoSafeSlice` stop with zero compiler
effects. The guard must not scan or rewrite historical cards.

Smallest next slice: `tools/checks/routing_classification_completeness_guard.sh`
plus its stable index entry and one focused positive/negative invocation. Keep
the script below 760 lines and leave all compiler source, fixtures, semantic
receipts, fallback, and production selection untouched.

Non-claims: no cohort admission, parser/source change, semantic package,
resolver/Recipe/Join/physical Call, raw retirement, ABI/backend change, or
performance claim. A guard pass is review evidence only.

## P1 receipt

- `bash tools/checks/routing_classification_completeness_guard.sh` passes for
  the card selected by `CURRENT_STATE.toml.latest_card_path`.
- A temporary card with the neutral rows removed fails before success with the
  explicit-neutral-state diagnostic; a temporary card with every
  `NoSafeSlice` line removed fails with the stop-line diagnostic.
- `bash tools/checks/current_state_pointer_guard.sh` and `git diff --check`
  pass. No compiler, fixture, semantic receipt, fallback, or production route
  was changed by this tooling slice.

The next semantic row remains
`CALLABLE-COMPATIBILITY-COHORT-STATE-CENSUS-D0`; its missing issuer/consumer
boundary is still `NoSafeSlice`.
