---
Status: accepted design — transport-only P0 candidate identified; package admission remains closed
Date: 2026-08-21
Decision: CALLABLE-COMPATIBILITY-SOURCE-ADMISSION-D0
Parent: docs/development/current/main/investigations/mirbuilder-compatibility-seam-final-ratchet-d0-2026-08-21.md
ProductionCaller: existing parser/macro, MIR, LLVM, Wasm, public-AST, JSON, VM, and REPL compatibility entrances; no new caller
ReplacementCell: preserve one typed compatibility origin without pretending it is a semantic package
Classification: design stop; source-reason transport only, no new accepted callable shape
Execution row: CALLABLE-COMPATIBILITY-SOURCE-ADMISSION-D0
---

# CALLABLE-COMPATIBILITY-SOURCE-ADMISSION-D0

## Six-line brief

Decision: Enumerate every callable source admission at the compatibility
boundary and preserve the distinction between parser-issued SourceBacked,
typed parser/macro Compatibility, source-free Unavailable, explicit Neither,
and Rejected; do not let an AST-only request silently become a semantic package.

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
source-free inputs remain `Unavailable` or explicit `Neither`, never inferred
SourceBacked.

Smallest next slice: `CALLABLE-COMPATIBILITY-SOURCE-TRANSPORT-P0`, a
transport-only BoxShape candidate. It may carry the already-issued macro
reason and parser lineage together to the existing compatibility owner, but it
must not admit a new cohort, issue semantic facts, or switch a production
caller. A later cohort admission would be a separate BoxCount D0.

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

The transport-only P0 remains `NoSafeSlice` until its single carrier and full
move path are proven. Remain `NoSafeSlice` if retaining the reason requires AST re-scan, if a
source-free entrance cannot be distinguished from `Neither`, if multiple
cohorts would share one guessed package, or if compatibility success is the
only evidence for source identity. The existing general classification-
completeness rule in `agent-current-entry-contract-ssot.md` is the governing
review checklist; this card supplies the row-specific finite table.
