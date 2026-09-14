---
Status: closed__fast__2026-09-14
Task: MIR-CALL-STATIC-COMPATIBILITY-I0-A3-PACKAGE-ADMISSION-BOUNDARY
Date: 2026-09-14
Priority: route phase14 acceptance through the source-backed MIR path and keep VM compatibility-only
Parent: mir-call-static-compatibility-a3-package-admission-d0-2026-09-14.md
NextCard: MIR-CALL-NORMAL-PIPELINE-RED-RECOVERY-D0
Implementation permission: bounded package-admission evidence only; no VM promotion, If lowering, or caller cutover
---

# A3 package-admission boundary and phase14 route

## Six-line brief

```text
Decision: A3 admits the selected same-brand MixedProgram through the source-backed MIR package chain; the observed VM keep/compiler-request range remains Compatibility-only, and resolver UnsupportedExpression(If) is a downstream named terminal.
Source authority + canonical issuer: CompletedParserPostpassV1 plus the A0-3 source-admission witness feed ParserNormalSourcePlanSurfaceIssuerV1, PreparedNormalDefaultProgramRootV1::from_callable_source, issue_normal_callable_semantic_package_with_brand_catalog_v1, and the existing resolver/publication owners.
Non-authority: VM compatibility roots, stage-specific compatibility overrides, AST/name/arity guesses, MIR, fallback, fixture-only success, and ParserStringUtilsBox name allowlists.
Fail-fast boundary: missing/foreign brand or lineage, coverage/slot/body mismatch, package rejection, and resolver deferred batches stop explicitly; no Compatibility downgrade or retry hides the terminal.
Smallest next slice: record one phase14 source-backed MIR invocation through package/resolver entry, preserve the first named resolver terminal, and freeze the finite five-callable If inventory as a separate expressivity row.
Non-claims: VM parity or promotion, resolver If lowering, publication/caller cutover, old-edge deletion, whole-artifact success, StringBox fixes, Windows evidence, or R7 closure.
```

## Current boundary

The phase14 second invocation is not blocked by a missing publication owner.
The route matters first. In the observed normal compiler-request range,
`--backend vm` enters `for_vm_keep_post_macro` in `src/runner/keep/vm.rs`
and seals `PreparedNormalDefaultProgramRootV1::seal(ast)`, which produces a
`PreparedNormalDefaultProgramSourceV1::Compatibility` root. The lifecycle
therefore has no callable source to consume, so it does not issue the semantic
package, brand catalog, static lookup, or publication owner. The resulting
unavailable ingress and retired fallback are the designed VM compatibility
boundary. Stage1 direct routes are outside this statement.

The acceptance route for this A3 slice is the source-backed `--backend mir`
path (or the default mainline route that selects it). That path reaches
`issue_normal_callable_semantic_package_with_brand_catalog_v1` and then stops
at the named `SelectedCallableResolverDeferredBatchV1` terminal when an `If`
appears in a resolver expression Value/Rhs/Initializer position.

The concrete phase14 static-call site to preserve in the evidence is
`lang/src/compiler/parser/program/parser_program_box.hako:102`, where
`ParserProgramBox` calls `ParserStringUtilsBox.starts_with/3`. `ParserBox` is
also an instance box containing similar static calls, but the owner name in a
resolver error does not prove that it was the failing caller. The two sites
must remain distinct until a source-site receipt identifies the caller.

## Evidence and finite blocker inventory

The observed command is recorded in the phase14 repro artifacts under
`/tmp/phase14_repro.sh` and `/tmp/hako_run17.err`. Its error is a resolver
deferred batch, not a publication-owner absence. Those files are local
observation, not yet a durable receipt: the acceptance run must pin its
commit, timestamp, command, and first terminal before it is counted. The
finite deferred callable inventory is:

| Owner | Deferred expression position | Terminal |
| --- | --- | --- |
| `PatternUtilBox.find_local_bool_before` | `If` in expression position | `ResolverDeferred` |
| `JsonNumberCanonicalBox.canonicalize_f64` | `If` in expression position | `ResolverDeferred` |
| `JsonFragNormalizerBox._normalize_instructions_array` | `If` in expression position | `ResolverDeferred` |
| `JsonFragNormalizerBox._canonicalize_f64_str` | `If` in expression position | `ResolverDeferred` |
| `LowerMethodArrayGetSetBox.try_lower` | `If` in expression position | `ResolverDeferred` |

This table is an inventory for the next expressivity D0. It is not an
authorization to add a fallback, infer a type from names, or widen the A3
source-admission witness.

## Implementation receipt

The existing source-backed package handoff is now guarded by
`src/mir/compiler/normal_default_pipeline_a3_tests.rs`:

* `mir::compiler::normal_default_pipeline::tests::a3_package_admission_tests::mixed_source_reaches_semantic_package_without_compatibility_retry` — PASS with `CARGO_BUILD_JOBS=4 cargo test --profile quick --lib ... -- --exact`
* the guard keeps a same-brand ordinary+static source as `SourceBacked`, runs
  the normal semantic package path, and observes the canonical `helper/1`
  definition; it does not claim a published backend route.
* the parser/source-admission and mixed source-plan guards remain PASS.

The compile emitted the existing 536-warning baseline. Four unrelated
normal-pipeline red tests remain explicitly classified in the next card:
two documented parent-baseline candidates and two route/view candidates that
still require parent replay. No compatibility expectation was loosened.

## Ordered bounded tasks

| Order | Task | Completion condition |
| --- | --- | --- |
| 1 | Route classification | Phase14 acceptance invokes the source-backed MIR/mainline route; VM keep evidence is labeled compatibility-only for the observed compiler-request range. |
| 2 | Package handoff evidence | One same-brand MixedProgram reaches the existing source plan, callable-source root, semantic package issuer, and resolver batch without a compatibility retry. |
| 3 | Terminal recording | The first `SelectedCallableResolverDeferredBatchV1` is recorded with command, commit, source site, callable owner, and the five-row inventory above. |
| 4 | Expressivity handoff | Create `MIR-CALL-RESOLVER-IF-EXPRESSION-EXPRESSIVITY-D0`; its owner is the existing resolver/body lowering path, not A3 admission or VM keep. |
| 5 | Publication acceptance | After the expressivity row closes, co-seal the finite `parser_program_box.hako:102 -> ParserStringUtilsBox.starts_with/3` target/header/result/publication tuple. |
| 6 | Cutover decision | Only after the tuple and acceptance matrix pass may the parser cohort's compatibility/static-child edge be retired. |

## Explicit terminals

| State | Meaning | Action |
| --- | --- | --- |
| `SourceBackedReady` | A0-3 witness and same-brand coverage are accepted | Continue through existing package owner |
| `CompatibilityOnly` | VM keep/compiler-request compatibility root or unsupported special route | Record as non-claim; do not promote |
| `AdmissionRejected` | Missing/foreign lineage, brand, slot, body, or coverage | Fail before package effects |
| `PackageRejected` | Existing source/package owner rejects the admitted product | Keep named package terminal |
| `ResolverDeferred` | Existing resolver cannot represent one of the finite `If` cases | Hand off to expressivity D0; never downgrade |
| `PublicationReady` | Target/header/result/publication tuple is co-sealed | Eligible for later cutover review |

## Non-claims and reopening triggers

This card does not implement VM source-backed materialization, resolver `If`
lowering, publication/caller switching, old-edge deletion, or full phase14
artifact execution. Reopen A3 if a same-brand admitted product reaches a
different terminal, if the source-site receipt identifies a caller other than
the recorded `ParserProgramBox:102` site, or if a normal MIR invocation
silently returns to Compatibility after package admission.
