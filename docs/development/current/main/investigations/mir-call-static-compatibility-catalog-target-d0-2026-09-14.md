---
Status: selected__publication_tuple_design__2026-09-19
Task: MIR-CALL-STATIC-COMPATIBILITY-CATALOG-TARGET-D0
Date: 2026-09-14
Parent: mir-call-r7-stringbox-lower-structural-membership-i0-2026-09-14.md
NextCard: none__mixed_source_co_seal_design
Implementation permission: false; publication tuple design and taskization only
---

# Static compatibility source admission and handoff

## Six-line brief

```text
Decision: design source-backed admission through existing package/publication owners; Windows is deferred and a catalog-only shortcut is not selected.
Source authority + canonical issuer: parser lineage and transformed source admitted by the normal materializer/package; ScriptDirectStaticCallLookupIssuerV1 issues target/result publication from that package.
Non-authority: AST possession, ScriptRoot location, names/arity, declaration existence, MIR, numeric sites, test results and Windows capability.
Fail-fast boundary: missing source/target/result relations reject before argument descent or MIR effects; compatibility cannot borrow a fabricated publication owner.
Smallest next slice: define A0/A1 for one parser-brand merged invocation: the mixed source-admission contract and finite static-parent co-seal relations.
Non-claims: no implementation permission, restored fallback, source widening, StringBox dynamic acceptance, Windows support or R7 closure.
```

Census boundary: the phase14/16/17 compiler invocations -> normal source
classification -> root package issuance -> static publication ingress -> named
static terminal. Includes the already-inventoried StringUtils calls and their
source/import/transform relations. Excludes unrelated static families, Math,
instance/Extern calls, backend parity and shared-schema deletion. The inventory
below is a bounded direct-call inventory, not whole-R7 coverage.

## Premise correction and source evidence

The user explicitly deferred Windows and requested worker-assisted taskization
on 2026-09-14. This reopens the internal source-admission design in this card.
The previous catalog-only audit still establishes `NoSafeSlice` for an immediate
port swap. It does not require waiting for someone else to implement an issuer.
The missing admission contract makes this design work, not Fast path.

The read-only worker distinguished three boundaries at `b54599f99b`:

| Boundary | Current source behavior | Design implication |
| --- | --- | --- |
| `src/mir/builder/normal_default_root_catalog_lifecycle.rs:251` and `:328` | Compatibility gets no semantic package; static lookup/publication issuance is skipped | Required semantic products are never issued on this route, not simply dropped by a port |
| `src/mir/builder/normal_script_direct_static_lookup.rs:45` | Existing issuer seals package declarations, imports, whole-source targets, result catalog and publication owner | Reuse this source-backed chain after admission is established |
| `src/mir/builder/normal_default_root_catalog_post_install.rs:75` | Publication owner on compatibility is rejected as drift | Attaching an owner to a raw compatibility root is not a valid repair |
| `src/mir/compiler/normal_source_plan/compatibility_origin.rs` | Typed compatibility retains AST, transform reason and parser lineage as transport only | Origin evidence is not a semantic package; AST-only JSON must be distinguished |
| `src/mir/builder/raw_compatibility_child_terminal.rs` and `src/mir/builder/static_result_publication_ingress.rs:151` | Located ScriptRoot context differs from required Cataloged caller/expression-site lineage | Source location alone cannot authorize a static target/result |

The phase14 smoke has two compiler invocations: fixture generation through
`stageb_emit_program_json_v0_fixture`, then normal-file execution of
`lang/src/mir/builder/compat/program_json_v0_entry.hako`. The historical error
alone does not identify which invocation failed. Do not conflate the payload's
Program JSON with the Hako compiler program being compiled to process it.

## Task 1 result — failing ingress and source disposition

The phase14 probe was run once with the existing `target/quick/hakorune`
binary, without Cargo. The first invocation completed and wrote the payload:

```text
target/quick/hakorune --emit-program-json-v0 <program.json> <phase14 fixture>
Program JSON written: /tmp/..._program.json
```

The failing invocation is the second one, which compiles and runs the Hako
MirBuilder entry:

```text
HAKO_PROGRAM_JSON_FILE=<program.json> target/quick/hakorune --backend vm \
  lang/src/mir/builder/compat/program_json_v0_entry.hako
[freeze:contract][static-call/legacy-fallback-retired]
owner=ParserStringUtilsBox method=starts_with arity=3
```

The payload fixture is therefore not the source that enters the failing
compiler route. The failing source is
`lang/src/mir/builder/compat/program_json_v0_entry.hako`; its
`using lang.mir.builder.MirBuilderBox` is expanded by
`prepare_normal_source_with_imports` into a recursively merged source cohort.
That cohort includes the parser sources, including
`lang/src/compiler/parser/program/parser_program_box.hako:102`, whose
`ParserStringUtilsBox.starts_with("" + declaration_row, 0, "[freeze:contract]")`
call is the named failing static site. The helper definition is in
`lang/src/compiler/parser/scan/parser_string_utils_box.hako:32-43`; its
`index_of` self-call at `:70` is a separate inventoried site. The import map
is the selected normal text-merge product; no bundle or external source
artifact is supplied by the Program(JSON v0) file.

The observed import chain is:

```text
program_json_v0_entry.hako
 -> lang.mir.builder.MirBuilderBox
 -> lang.compiler.build.build_box (BuildBox)
 -> lang.compiler.parser.parser_box (ParserBox)
 -> lang.compiler.parser.program.parser_program_box (ParserProgramBox)
 -> ParserStringUtilsBox.starts_with/3
```

The first invocation uses the explicit Stage-1 bridge
`--emit-program-json-v0` and its strict Program(JSON v0) source route. The
second invocation uses the normal VM front door, which materializes the
merged source through
`materialize_normal_callable_program_with_identity_v1(..., filename)`.
The parser postpass classifies a static/mixed merged program as the
`StaticBox`/`MixedProgram` compatibility cohort. The transform reason is
therefore the typed
`NormalCallableTransformCompatibilityV1::Parser(MixedProgram)` branch. It
becomes a `NormalCallableCompatibilityOriginV1` carrying AST, reason and
parser lineage, then `for_mir_mode_compatibility` creates the compatibility
root. Static declaration rows remain `AstOnlyCompatibility`; no semantic
package, catalog, target or result publication owner is issued. This is a
typed compatibility transport around AST-only semantic rows, not a
source-backed callable product.

This single chain closes Task 1's invocation/source question. Task 2 now has
the concrete design boundary: decide whether this entire merged static parser
cohort can be admitted through the existing source-backed package issuer,
including its imports and all static method body relations, or name each
unsupported partition before any I0 switch.

## Task 2 result — whole-cohort admission decision

The whole merged `MixedProgram` is a `NoSafeSlice` for the existing package
issuer. A compatibility origin cannot be relabelled as
`PreparedNormalDefaultProgramRootV1::from_callable_source`: the required
source authority is not issued on that branch.

The existing chain requires one parser invocation to provide all of these
relations before the package is issued:

- ordinary source seals matching final box path, ordinal, declaration kind,
  method inventory, generated delegate and constructor coverage;
- a complete parser-branded parameter catalog;
- a ready normal-source-plan seed and root-execution/source-authority product;
- every callable body tied to a parser-issued callable identity;
- canonical static target/header, expression-site, result-contract and
  publication-owner relations for each direct static call.

The current static-parent issuer is narrower still: it accepts only a pure
`StaticBox` cohort with one parent, direct-method-only members, and exactly one
direct method (`src/parser/callable_parameter_source/static_box_source.rs:
295-410`). The phase14 merged source has ordinary `ParserBox` plus multiple
static parser and builder boxes, so it is `MixedProgram` and cannot use that
seal. `ParserNormalSourcePlanSurfaceIssuerV1` already knows how to consume
multiple static/ordinary rows, but only after the completed postpass is
source-backed with a complete seed; the current compatibility branch never
provides that precondition.

The unresolved partitions are explicit: static/mixed top-level source seals;
static method body identities; loop/conditional/early-return body projections;
merged using/import rows; opaque or transferred subtrees; and non-ordinary
parameter-transfer rows. Body shape alone does not prove any of these
relations. The existing `ParserStaticBoxParentSourceAuthorityIssuerV1` and
normal source-plan surface must be extended or a complete source-backed
sub-cohort must be defined before a caller switch is safe.

The smallest finite observation tuple is retained for the next design task:

```text
caller:     src/mir/builder/method_call_handlers.rs:456
site:       lang/src/compiler/parser/program/parser_program_box.hako:102
target:     ParserStringUtilsBox.starts_with/3
old edge:   method_call_handlers.rs:493-498
            UnissuedStaticCallRetirementV1::GenericCompatibility
```

This tuple is not an I0 authorization. The next D0 must choose one authority
boundary for the whole merged invocation: either a generalized parser
source-seal/static-parent issuer that admits all required mixed rows, or an
explicit source-backed sub-cohort with finite include/exclude relations and a
retained compatibility partition. A name allowlist, AST rewrite, MIR
inference, or generic fallback re-entry cannot fill the gap.

## Task 2b result — one authority boundary selected

The read-only design comparison selected **A: admit the entire merged
invocation as one source-backed product under the same parser brand**. The
source-backed `NormalSourcePlan`, callable catalog, semantic package and
static publication issuers already provide one authority chain for ordinary
and static callable rows. The source-backed catalog can project both static
and instance methods, and the package issuer can co-seal resolver forest,
source-site and result/publication relations without re-inferring body
structure from MIR.

The B alternative (a source-backed sub-cohort with compatibility rows beside
it) is declined. Compatibility is currently an AST/reason/lineage transport
with no semantic package, target or result issuer. Allowing a source-backed
body to call a compatibility row would require a new cross-boundary package
and would create a second semantic authority. The observed
`ParserProgramBox.parse/2 -> ParserStringUtilsBox.starts_with/3` edge crosses
that boundary, so B cannot be made safe by a finite name or callsite list.

This is a design selection, not an implementation authorization. The current
static-parent issuer remains too narrow (`StaticBox`, one parent, one direct
method), and the ordinary source seal does not cover `MixedProgram`. The
selected direction must first be decomposed into these bounded design rows:

1. **A0 — mixed source-admission contract.** Define the same-brand merged
   invocation boundary and the exact ordinary/static parent, callable method,
   body, constructor and import-lineage relations that must be co-sealed.
   Fix the source-backed entry condition and explicit reject/non-claims.
2. **A1 — static-parent source co-seal.** Decide how the finite static-parent
   set and every direct method relation extend
   `ParserStaticBoxParentSourceAuthorityIssuerV1`. Foreign/stale sites,
   unsupported members and coverage mismatches remain explicit rejects.
3. **A2 — import/body coverage (follow-up).** Co-seal each imported source
   path, callable identity, member ordinal, body/source-site inventory,
   constructor and import lineage. Loop, conditional and early-return shapes
   are body coverage; opaque or non-ordinary transfer stays rejected.
4. **A3 — existing package admission (follow-up).** Connect the admitted
   product to the existing normal source plan, callable catalog, semantic
   package and static lookup/publication issuers. The first finite tuple is
   `MirBuilder::handle_static_method_call_with_descent` at
   `parser_program_box.hako:102` targeting
   `ParserStringUtilsBox.starts_with/3`; target identity must be co-sealed,
   never inferred from name/arity.
5. **A4 — cutover and deletion (follow-up).** After every direct-call
   observation in the merged cohort is cataloged and published, delete only
   that cohort's compatibility classification/raw static-child edge. Do not
   remove the generic retirement terminal before its other callers migrate.

The safe next slice is therefore **A0/A1 design only**. No source widening,
caller switch, production fallback or old-edge deletion is authorized until
A2 body/import coverage and A3 package/publication admission are closed.

## Task 2c result — A0/A1 are separate design rows

The second read-only audit confirms that A0 and A1 must remain ordered design
rows. A0 decides whether the merged invocation is one same-brand source
window; A1 issues the finite static-parent/member/method relations inside that
window. Generalizing the static seal before the source window, imports,
ordinary parents and body coverage are fixed would let a static row look
source-backed without a complete package authority.

### A0 — mixed source-window admission contract

The admitted window is deliberately narrower than the broad
`MixedProgram` label:

```text
one parser invocation brand
+ ordinary Box declarations
+ static Box declarations
+ their direct callable methods
+ no other top-level declaration family
```

Interface/record/build-gate/enum/brand/type-alias/global/static-constant and
nested-Program rows remain explicit rejects. `using`/`import` directives are
not semantic rows after text expansion; their source identity, alias, path and
lineage must instead be carried as typed relations in the same parser product.
The existing normal source-plan row projection already recognizes ordinary
and static boxes and marks other top-level kinds as `Unsupported`
(`src/parser/callable_parameter_source/normal_source_plan_surface.rs:319-476`).
The ordinary-only postpass source finalizer still rejects non-ordinary cohorts
(`src/parser/source_seal/finalize.rs:163-186`), so A0 must provide a new
source-backed admission boundary rather than relabeling compatibility.

A0's issuer consumes one `CompletedParserPostpassV1`, one
`ParserInvocationBrandV1`, projected slots, ordinary source seals, A1 static
seals, a complete parameter catalog, all direct callable rows and typed import
lineage. It must verify slot count/position, declaration path and brand,
callable identity/kind, method/member coordinates, ordinary inventory and
constructor/generated-delegate coverage. The current merged-source helper
returns code plus an alias map; that is insufficient as authority, so missing,
ambiguous or duplicate import lineage is a reject rather than an inferred
source relation.

On success, A0 hands one `ParserBackedNormalSourcePlanBoundV1` and the
source-backed initial/root execution product to the existing normal source
plan, callable catalog, semantic package and publication consumers. On
failure it preserves the existing named terminals: postpass/compatibility
unavailable, foreign brand/relation, slot mismatch, missing/orphan ordinary or
static parent, missing callable syntax/source, relation mismatch, unsupported
top-level row, missing/ambiguous import lineage, and non-direct/nested or
nonordinary parameter transfer. No empty catalog or AST-only substitute is
issued.

### A1 — finite static-parent source co-seal contract

A1 takes the A0 brand and final slot set plus a finite set of prepared static
parent rows. For each parent it co-seals:

```text
brand
box declaration source path and syntax
member count and member ordinal -> kind rows
direct method source site
parser-issued callable identity
complete parameter-catalog row
```

The current `ParserStaticBoxParentSourceAuthorityIssuerV1` is intentionally
too narrow: it accepts only a pure `StaticBox` cohort, one parent, one path
segment, all-direct-method members and exactly one direct method
(`src/parser/callable_parameter_source/static_box_source.rs:301-360`). A1
extends the design to multiple same-brand parents and multiple direct methods,
while `Field`, `InitBlock` and `StaticInitializer` remain an explicit
unsupported partition until their own owner is named.

Every method relation must be direct, point to the parent path and current
member ordinal, have an empty gate path, match exactly one callable identity
and parameter row, and be free of duplicate coordinates or identities. Each
parent maps to one final slot, and no prepared parent may remain unconsumed.
Member-count mismatch, foreign/stale site, missing source, duplicate identity,
or coverage mismatch rejects before source-backed admission. These checks
extend the existing coordinate/identity checks at
`static_box_source.rs:322-422` and the static-row consumer at
`normal_source_plan_surface.rs:378-433`.

A1 does not interpret method bodies. Loop, conditional and early-return
structure stays attached to the parser-issued callable source and is later
checked by the resolver/source projection. Ordinary constructor and generated
delegate coverage stays with the ordinary source seal. Every direct-call
observation still needs a canonical target/header/source-site relation; the
existing semantic-package co-seal closes missing target/header/inventory or
duplicate ownership as `UnissuedDirectCallObservation`
(`src/mir/normal_callable_semantic_package/issuer.rs:97-229`). Opaque subtrees,
unissued targets and nonordinary transfers remain rejects.

The consumer chain after both design rows close is:

```text
parser source-window issuer
 -> ParserNormalSourcePlanSurfaceIssuerV1
 -> source-backed normal root
 -> ConsumedNormalRootCallableSourceV1
 -> source-backed callable catalog
 -> resolved semantic batch / result contract / physical signature
 -> ScriptDirectStaticCallLookupIssuerV1
 -> static result publication owner
```

No deletion occurs in A0/A1. The later cutover may remove the selected
cohort's compatibility classification in
`src/parser/postpass_envelope/normal_callable_program.rs:107-115` and the
observed generic terminal at
`src/mir/builder/method_call_handlers.rs:493-498`, but only after all source
rows, body/direct-call observations and publication owners are co-sealed and
compatibility re-entry is impossible.

### A0/A1 task order

| Order | Bounded row | Completion condition |
| --- | --- | --- |
| A0-1 | Mixed source-window predicate | Ordinary/static declarations and direct methods are the only admitted rows; every excluded cohort has a named reject terminal. |
| A0-2 | Import-lineage co-seal | Every merged source segment has parser-brand identity/path/alias/lineage; missing, ambiguous and duplicate relations reject. |
| A0-3 | Source-backed root admission | The allowed mixed window reaches existing `NormalSourcePlan` as `Ready`; it cannot be relabeled from AST-only compatibility. |
| A1-1 | Finite static-parent issuer | Multiple same-brand static parents and direct methods are co-sealed by path/member/identity with no orphan or duplicate rows. |
| A1-2 | Body/call/constructor handoff | Callable bodies, ordinary constructor/generated rows and every direct-call source-site are available to downstream package issuers. |
| A1-3 | Selected caller/site proof | `ParserProgramBox.parse/2 -> ParserStringUtilsBox.starts_with/3` has a canonical catalog/publication route, with target identity co-sealed rather than inferred from name/arity. |

The A0-1 predicate is now fixed as design evidence. A1 must consume A0's
accepted window; neither row grants implementation permission yet.

## Task A0-2 result — typed import lineage is still a design stop

The read-only import audit confirms that A0-2 cannot be closed by the current
merge outputs. `PreparedSourceWithImports` returns only merged `code` and a
`HashMap<String, String>` alias-to-box binding
(`src/runner/modes/common_util/source_hint.rs:4-7,67-112`). The text merger
does create DFS order and `LineSpan { file, start_line, line_count }`
(`src/runner/modes/common_util/resolve/strip/merge.rs:123-227`), but the
thread-local context is a diagnostic observer, not parser authority
(`src/runner/modes/common_util/resolve/context.rs:4-57`). Duplicate canonical
paths, alias rebinding and alias-to-box conflicts are rejected by the strip
owner (`src/runner/modes/common_util/resolve/strip/using.rs:197-233,582-613`,
`merge.rs:263-281`), yet their result is still a string error/map rather than
a branded source product. The current normal parser lineage represents the
merged root as one identity and loses per-import provenance.

### A0-2-D typed lineage contract

The next design row must define one finite lineage product emitted by the merge
owner together with the merged text. Each segment carries:

```text
source identity (root/imported canonical path and, when available, bytes digest)
import edge (origin source, using line, requested target, resolved path)
alias (explicit/default alias and concrete static-box binding)
merged range and origin-local range
DFS merge ordinal and parent import relation
parser-brand co-seal witness
exactly-once segment coverage (no gap, overlap or orphan)
```

The handoff remains one-way: `prepare_normal_source_with_imports` returns
merged code, the existing runtime alias map and typed lineage; the normal MIR
entry passes all three into the parser/materializer; the parser co-seals
lineage with `ParserInvocationBrandV1`, projected slots, ordinary seals and
A1 static seals; only a successful co-seal may issue the existing
`ParserBackedNormalSourcePlanBoundV1` and downstream source-backed catalog /
package. `using_import_boxes` remains runtime alias lowering and is not a
replacement authority. Builder/MIR code must not reconstruct import lineage.

Reject terminals must include unresolved target, segment gap/overlap, source
read or digest mismatch, foreign brand, unregistered segment and missing
lineage, in addition to existing duplicate path, alias rebinding and alias
binding conflict. `NormalParserSourceLineageV1` remains the merged-root
product; `LineSpan` and the alias map stay non-authoritative.

This is design-only. The A0-2-D design condition is fixed above. The source
backed root admission remains the next A0 design boundary.

## Task A0-3 result — source-backed root needs an admission witness

The existing postpass has a partial route: when its broad
`semantic_candidate` is true, `source_seal/finalize.rs:179-243` selects
`from_initial_compatibility`, and `postpass_envelope.rs:264-294` stores an
`Initial` program while retaining the `MixedProgram` cohort label. This is not
itself a complete A0 admission contract. `CompletedParserPostpassV1::is_source_backed()`
only checks the `Initial` enum arm (`postpass_envelope.rs:175-178`), while
`compatibility_program_can_enter_initial_callable_lane_v1` accepts unrelated
top-level families beyond its explicit build-gate/interface/record/delegate
checks (`initial_callable_program_source/issue.rs:144-172`). The normal source
plan can also leave unsupported top-level rows as `Unsupported` while still
returning `Ready` (`normal_source_plan_surface.rs:319-476`).

Therefore A0-3 cannot be closed by flipping the existing predicate or by
calling `from_initial_compatibility` for every `MixedProgram`. The source-backed
root needs a parser-issued **admission witness** that co-seals:

```text
MixedProgram cohort (label retained)
same ParserInvocationBrandV1
ordinary source seals
A1 static parent/method rows
projected program slots
complete parameter catalog
A0-2 typed import lineage
ordinary/static direct callable coverage
```

The witness must reject interface/record/build-gate/enum/brand/type-alias/
global/static-constant/nested/using/import rows, foreign brands, slot or
parent coverage mismatch, missing callable/source rows and unsupported
transfer. `NormalSourcePlanSurfaceIssuerV1` consumes this witness and returns
`Ready` only for the allowed window. `from_compatibility` and
`CompatibilityOutside` remain the AST-only route; they must not be used as a
source-backed shortcut.

The existing handoff remains one execution path:

```text
CompletedParserPostpass
 -> ParserNormalSourcePlanSurfaceIssuerV1
 -> ParserNormalRootExecutionIssuerV1
 -> ParsedProgramWithCallableParameterSourceV1::new
 -> ParserNormalRootSourcePlanConsumerV1
 -> into_normal_callable_program_with_root_execution
 -> PreparedNormalCallableProgramSourceV1
```

The named terminals stay affine. Surface failures use
`SourceAuthorityUnavailable`/`Incomplete`/`IntegrityInvalid`; root failures
include missing or duplicate Main coverage; consumer failures distinguish
compatibility/source-authority/incomplete/integrity; the final transform keeps
foreign parser, callable/syntax/constructor coverage,
`CompositeSourceCompatibilityLoss` and `MainAppEntryCompatibilityLoss`.
Every reject consumes the product, source authority, catalog and root siblings
at its existing named terminal. No AST-only fallback is emitted after a
failed source-backed admission.

The next bounded row is **A0-3-D**: fix the witness issuer/consumer boundary
and the complete reject mapping. A1 static co-seal, A0-2 lineage transport,
production switch and old compatibility-edge deletion remain downstream.

### A0-3-D design boundary

The witness belongs between the parser finalizer and the existing normal
source-plan issuer. It must be issued from the same `CompletedParserPostpass`
transaction that owns the AST, projected slots, callable rows, ordinary
seals, A1 static seals, parameter catalog and A0-2 lineage; it must be consumed
by `ParserNormalSourcePlanSurfaceIssuerV1::issue_once` before that issuer can
return `Ready`. The downstream handoff stays:

```text
CompletedParserPostpass
 -> ParserNormalSourcePlanSurfaceIssuerV1
 -> ParserNormalRootExecutionIssuerV1
 -> ParsedProgramWithCallableParameterSourceV1::new
 -> ParserNormalRootSourcePlanConsumerV1
 -> into_normal_callable_program_with_root_execution
 -> PreparedNormalCallableProgramSourceV1
```

The witness is an admission relation, not a second semantic package or a new
`Verified*` meaning. Its issuer must prove same-brand identity and exact
coverage for the allowed ordinary/static/direct-call window. It must retain
the `MixedProgram` cohort label, so `Initial` does not silently relabel the
program as ordinary. The existing `semantic_candidate` predicate may be used
as an input observation, but it cannot be the admission authority because it
currently accepts unrelated top-level rows and `is_source_backed()` only
checks the enum arm.

Failure mapping remains affine: surface
`SourceAuthorityUnavailable`/`Incomplete`/`IntegrityInvalid`; root
`MainMethodMissing`/`MainMemberCoverage`/`DuplicateMain`/
`DuplicateMainMethod`; consumer
`CompatibilitySourceUnavailable`/`SourceAuthorityUnavailable`/`Incomplete`/
`IntegrityInvalid`; and final transform foreign-parser, callable/syntax/
constructor coverage, `CompositeSourceCompatibilityLoss` or
`MainAppEntryCompatibilityLoss`. On every failure the parser product,
parameter catalog, source authority, root disposition and lineage are
consumed at their existing named terminal; no AST-only fallback is emitted.

The exact placement is now bounded: the witness is issued once while
`from_initial_compatibility` builds the `CompletedParserPostpassV1` owner and
is move-consumed by `ParserNormalSourcePlanSurfaceIssuerV1::issue_once`
(`src/parser/postpass_envelope.rs:264-294`,
`src/parser/callable_parameter_source/normal_source_plan_surface.rs:254-301`).
`ParsedProgramWithCallableParameterSourceV1::new` transports the resulting
surface/root state only; it must not infer admission from AST or cohort later.

The witness carries only the parser brand, retained `MixedProgram` label,
typed import-lineage co-seal, static-parent co-seal and projected/callable
coverage witness plus an affine seal. It does not duplicate slot sets,
ordinary seals, callable rows or static rows owned by the existing seed,
postpass coverage, initial callable source and A1 products. The
`from_compatibility` path issues no witness and stays `CompatibilityOutside`;
the compatibility closure continues to reject Ready/incomplete/integrity
states instead of downgrading them.

The disposition design is recorded below as A0-3-D1. This remains design-only;
no code, fixture, production switch or old-edge deletion is authorized.

### A0-3-D1 disposition ownership result

The logical admission disposition has exactly three arms and is stored once in
`CompletedParserPostpassV1`:

```text
ParserNormalSourceAdmissionDispositionV1
  Ordinary
  Mixed(ParserMixedSourceAdmissionV1)
  CompatibilityOutside
```

`from_source_product` issues `Ordinary` with the existing `Initial`, ordinary
coverage and ready seed. `from_initial_compatibility` issues `Mixed(witness)`
only after source-seal, final ordinal, seed, A0-2 lineage and A1 static
co-seal checks pass, retaining the `MixedProgram` label. `from_compatibility`
issues `CompatibilityOutside` with the AST-only program and compatibility seed.
The new disposition duplicates no slots, static rows, ordinary seals, callable
identities, constructor source or lineage; those stay in the existing seed,
postpass coverage, initial callable source, A1 product and A0-2 product.

The affine transition is:

```text
CompletedParserPostpass --consume admission + seed-->
ParsedProgramWithCallableParameterSourceV1::new
  --pass witness-->
ParserNormalSourcePlanSurfaceIssuerV1::issue_once(move witness)
  -> ParserBackedNormalSourcePlanBoundV1
  -> root execution -> normal callable source
```

Only `Ordinary` and `Mixed` enter the source-backed root transform;
`CompatibilityOutside` alone enters the existing compatibility closure. A true
broad candidate followed by witness failure is a named source-admission reject,
never a silent return to compatibility. Existing coverage, seed, surface,
root, consumer and final-transform terminals consume the witness and sibling
products together. This closes the A0 design; the next decision selects the
first physical producer slice, A0-2 typed lineage transport or A1 static
co-seal.

## Reused finite caller inventory

The source owner is
`lang/src/compiler/parser/scan/parser_string_utils_box.hako::starts_with`.
The prior inventory records 16 direct sites under the parser tree:

- `program/parser_program_box.hako`: 102, 125, 164;
- `parser_box.hako`: 163, 226, 229;
- `stmt/parser_control_box.hako`: 82, 146, 238, 281, 363, 492;
- `stmt/parser_guard_box.hako`: 26, 92;
- `stmt/parser_stmt_box/core.hako`: 347;
- `scan/parser_string_utils_box.hako`: 70 (`index_of` self-call).

They use the ordinary StaticReceiver route. The observed rejection is
`[freeze:contract][static-call/legacy-fallback-retired]` for
`ParserStringUtilsBox.starts_with/3`; this is dependency evidence, not proof
that each callsite executed or that the StringBox lowerer was reached.
No repeated repository-wide census is needed.

## Ordered task pack

| Order | Task | Concrete output / completion condition |
| --- | --- | --- |
| 1 — complete, D0 | Pin failing ingress and source disposition | The one phase14 run proves fixture emission succeeds and the second `program_json_v0_entry.hako` normal VM invocation fails at `ParserStringUtilsBox.starts_with/3`; the merged import cohort and typed compatibility/AST-only branch are recorded above. |
| 2 — complete design audit, NoSafeSlice | Decide source admission for that whole cohort | The merged `MixedProgram` cannot enter the current package issuer: static parent/source-seal, parameter, root/source-plan, callable identity, target/header/result/publication relations are not co-issued. The finite `ParserProgramBox.parse/2 -> ParserStringUtilsBox.starts_with/3` tuple and every unresolved partition are recorded above. |
| 2b — complete, D0 | Choose the mixed-cohort authority boundary | A is selected: the whole same-brand merged invocation becomes one source-backed product. B is declined because compatibility has no semantic package/publication issuer and would create a second authority. The next rows are A0/A1 design; no implementation or caller switch. |
| 2c — complete design split, D0 | Separate A0/A1 source co-seal rows | A0 owns the same-brand source-window/import contract; A1 consumes that window and owns finite static-parent/member/method co-seal. The ordered A0-1…A1-3 outputs and reject partitions are recorded above; implementation remains unauthorized. |
| 2d — complete design, D0 | A0-1 mixed source-window predicate | Ordinary/static declarations and direct methods are the only admitted rows; excluded top-level families and reject terminals are fixed above. |
| 2e — complete design, D0 | A0-2-D typed import-lineage schema | The merge-owner lineage product, parser-brand co-seal boundary, exact coverage/reject rules and parser handoff are fixed above. Transport implementation remains unauthorized. |
| 2f — complete design, D0 | A0-3-D source-backed admission witness | The witness is issued once in the completed postpass and move-consumed by the normal source-plan surface; it carries no duplicated semantic rows and preserves the compatibility closure. |
| 2g — complete design, D0 | A0-3-D1 admission disposition ownership | `CompletedParserPostpassV1` owns `Ordinary`/`Mixed(witness)`/`CompatibilityOutside`; constructors, surface consumer and affine reject mapping are fixed above. |
| 2h — complete design, D0 | Choose first physical producer slice | A0-2 typed import-lineage transport precedes A1 because A1 needs one parser-branded merged invocation, import identity, segment coverage and alias-edge authority. The first I0 is selected below; no production switch yet. |
| 2i — complete, landed | A0-2 typed import-lineage transport | `MergedSourceLineageV1` is issued by the merge DFS, carried beside runtime aliases, and co-sealed to one parser invocation. Diamond-edge and nested-parent/source-range corrections are landed; missing/foreign-lineage production consumption remains a non-claim. |
| 2j — complete, bounded | A0-3/A3 source-backed package boundary | The Mixed admission witness reaches the existing normal source-plan and semantic-package owners on the MIR route; VM keep remains compatibility-only and the resolver/physical-result terminal stays explicit. No publication or caller cutover claim. |
| 3 — active design | Static publication tuple | Define the exact Cataloged source-site, target/header, result contract and publication-owner handoff for the finite parser cohort. Reuse the existing ingress; do not add a second issuer or switch a compatibility root. |
| 4 — conditional I0 | Switch the accepted source cohort | Connect the fully admitted tuple to the existing publication consumer and retire only that cohort's compatibility classification/raw static-child edge. Scope the exact caller and branches; no blanket root switch. |
| 5 — I0 acceptance | Prove publication, rejection and retirement | Real selected source reaches static publication; missing/foreign site, brand mismatch, missing/ambiguous target and unsupported source/result reject before argument effects. Existing owner guards prove selected old-edge absence and residual handling. |
| 6 — return to StringBox I0 | Close original owner acceptance | Run existing phase14/16/17 and malformed/wrong-class/extra-argument/embedded/empty cases only after upstream reach is established. Record exact owner-to-terminal results; an earlier stop leaves this acceptance open. |

Task 2 prefers source-backed admission because the canonical issuer exists.
A catalog-only alternative would need a complete independent semantic design
for target and result issuance; declaration lookup is insufficient and that
alternative is not selected here. Task 1 is the first concrete unresolved
question. The task pack is accepted for design; tasks 3–5 are dependency-bound,
not an assertion that implementation is ready.

## Implementation entry and finish line

Before selecting I0, record one exact source cohort, every required issuer,
consumer and source identity, the finite positive/negative mapping, and the
specific old branch to delete. The old generic fallback is already retired;
its absence is not a new deletion credit. Shared compatibility helpers and
the generic retirement terminal stay until their own callers migrate.

Use existing `Facts`/Recipe/JoinSig/publication ownership. Do not add a second
resolver, guessed receipt, named-helper allowlist, AST rewrite or test-only
semantic constructor. Keep semantic admission changes separate from any
required source split. Current audit sizes: lifecycle owner 682 lines,
post-install 160, lookup issuer 104; design a responsibility split at 760 and
keep each changed source below 800.

I0 must update affected module README and reference contracts with code/tests.
Run one Cargo process, quick profile, at most four jobs and focused nonzero
positive/negative tests plus existing owner guards. No whole-library or CI run
is required to accept this taskization. Windows is not a gate in this ladder.

## Scheduler handoff — 2026-09-19

The normal-pipeline red recovery inventory is now stable: the two
`published_consumer_*` failures are immutable parent/current baseline debt and
the two former route candidates pass at the current head. The route-observer
I0 is already landed, so no assertion rewrite or owner fix is selected here.

This card is the next non-parked design boundary. A0/A1/A2/A3 child receipts
are retained as prerequisites; the remaining decision is the exact
source-backed publication target/header/result tuple and its later cohort-local
compatibility-edge cutover/delete set. Implementation remains prohibited until
that authority chain and terminal inventory are accepted. The OwnedText T3
family stays `ParkedSealed__NoSelectedOwnedTextCaller`.

## Publication ingress source audit — 2026-09-19

The remaining boundary is not an unconnected Builder call site. The existing
publication chain is already wired at three distinct owners:

```text
normal_default_root_catalog_lifecycle
  -> ScriptDirectStaticCallLookupIssuerV1
  -> static_result_publication_owner
  -> ModuleDraftCollectorV1::install_static_result_publication_owner

StaticReceiver/member route
  -> StaticResultPublicationIngressPortV1
  -> Selected/TargetOnly physical lowerer
  -> Unavailable-only legacy owner policy
```

`src/mir/builder/method_call_handlers/static_current_owner_policy.rs` takes
the publication ingress before the legacy owner policy for `me.method(...)`;
`src/mir/builder/calls/member_route.rs` does the same for a static receiver.
Both routes fail on a typed ingress error or an exact-target miss and only
enter the retained owner policy for `Unavailable`. The owner is installed by
`normal_default_root_catalog_lifecycle.rs` and
`program_root_lowering.rs`; no second publication issuer is needed.

Therefore the unresolved tuple is upstream reachability and exact source
identity, not a missing ingress connector. The phase14 source-backed MIR
route must first survive the finite resolver expression-`If`/physical-result
boundary and issue a `Cataloged` source site. Only then can the selected
`ParserProgramBox.parse/2 -> ParserStringUtilsBox.starts_with/3` observation
consume the existing owner. A compatibility root cannot borrow this owner.

This audit closes the question "where should publication be connected?" as an
existing-owner reuse. It does not close publication acceptance: target/header/
result co-seal, real source-site consumption, cohort-local compatibility-edge
deletion, and phase14 source-to-exe evidence remain open. No code, fallback,
resolver widening, or new semantic receipt is authorized by this note.

## Finite tuple shape audit — 2026-09-19

The first tuple has a narrower caller boundary than the whole parser corpus:

```text
caller:  ParserProgramBox.parse/2
         (static box; StaticBoxMethod namespace)
site:    parser_program_box.hako:102
target:  ParserStringUtilsBox.starts_with/3
args:    String, I64, String
result:  ExactI64 (source returns only 0 or 1)
required_i64_arguments: [1]
```

The caller namespace is significant. `ParserProgramBox` is a static box, so
its source context can become `Cataloged(StaticBoxMethod)` and is eligible for
the existing publication ingress. The similarly named calls in instance
`ParserBox` are not part of this tuple: their `InstanceBoxMethod` lineage is a
separate lane and must not be admitted by widening the static ingress or by
matching the owner name.

The result and argument facts are already represented by the existing
`VerifiedStaticCallResultPublicationHandoffV1`: the target/header and exact
source `SourceExprSiteV1` are keyed by the parser-issued caller/site, the
result representation is `ExactI64`, and only argument ordinal 1 requires the
i64 proof. The Hako source line is diagnostic evidence only; the publication
consumer must receive the parser/resolver-issued `SourceExprSiteV1`.

This closes the tuple's semantic shape, but not its live acceptance. The
remaining design evidence is the resolver/physical-result route reaching that
site and consuming the handoff exactly once; then the cohort-local old edge
can be selected for deletion. No instance-box expansion or generic fallback
is authorized.

## Resolver/physical-result dependency reconciliation — 2026-09-19

The upstream resolver boundary is now landed and must not be reopened as a
second parser task. `ResolvedExpressionSourceInventoryV1` publishes the exact
expression-`If` relation (condition, both empty-prelude `BlockExpr` tails,
consumer role, and nested call observations), and the source-result issuer
consumes that sealed relation through the callable ledger. Declared
`ArrayBox`/`MapBox` parameters and `return null` value classification have
also crossed their named terminals. These receipts prove source admission
facts only; they do not issue a physical value or publication handoff.

The remaining blocker is the physical result consumer. Existing
`IfRecipeV1`/`IfJoinSigV1` and `resolved_lowering/if_recipe_adapter.rs` own
statement-`If` control and `I64`/`Bool` trivial values. They cannot consume the
expression product's `String` branch class, and widening them would mix
statement and expression authority. `canonical_ssa` remains the sole mutable
CFG/SSA/PHI owner, so the next design row must add a route-specific result
port over that owner, carrying the source-issued owner, exact expression-`If`
site, branch exits, and the parametric `I64 | String` class. The existing
`VerifiedResolvedIfCfgReadyJoinRowsV1` is a useful CFG witness, but its rows
are `BindingRefV1` joins and therefore cannot be used by manufacturing a
synthetic binding for an expression result. The physical consumer may project
`I64 -> MirType::Integer` or `String -> MirType::String` only after consuming
the co-sealed product; it must never infer the class from MIR values or rescan
the AST.

The exact exclusion is visible in the current owner code: `IfValueClassV1`
contains only `I64` and `Bool`, `CanonicalIfPhysicalCorrespondenceV1` is
statement-site/assignment oriented, and `class_for_representation` rejects
non-trivial representations. These are valid statement-If invariants, not a
missing switch to widen in place.

The physical design can still reuse the canonical mechanics without creating a
second CFG/SSA owner. `IfCfgSessionV1` remains responsible for block layout,
branch closure, and merge-predecessor revalidation. A new expression-result
witness should carry those verified predecessors, the two source-issued branch
values, the exact outer consumer, and the `I64 | String` class without a
`BindingRefV1`. Its adapter can then call the existing
`PhiDraftV1::prepare_cfg_ready` and
`phi_lifecycle::define_final_from_prepared_completion`; the source class is
the only type hint, and the returned `ValueId` is handed to the existing
Return/initializer/RHS consumer. This reuses `phi_type_publication` and input
materialization while keeping source admission, CFG topology, and physical
commit in their current owners.

### Next bounded design slice

1. **Result-port contract:** name the existing canonical CFG/SSA entry and
   the one result-carrying JoinSig/port for `Return.Value`, initializer, and
   assignment-RHS consumers. Keep the statement-`If` owner unchanged.
2. **Static tuple co-seal:** map the parser-issued
   `ParserProgramBox.parse/2 -> ParserStringUtilsBox.starts_with/3` row to
   that port with its exact `SourceExprSiteV1`, target/header, `ExactI64`
   result, and required i64 argument ordinal `[1]`.
3. **Parametric guards:** prove the same port rejects missing/foreign
   conditional rows, mixed or unknown branch classes, duplicate consumers,
   non-empty preludes, and unconsumed products before any MIR effect. The
   finite parser cohort remains whole-source admitted; an I64-only shortcut is
   not selected.
4. **Exit:** open implementation only after the port, source-result product,
   publication handoff, and exact delete-set are co-sealed. Until then, no
   caller switch, fallback re-entry, or legacy-edge deletion is authorized.

This dependency audit supersedes the older wording that described the resolver
expression-`If` relation itself as pending. The relation and source-result
issuer are landed; physical expression-result lowering and live Cataloged-site
consumption remain open.
