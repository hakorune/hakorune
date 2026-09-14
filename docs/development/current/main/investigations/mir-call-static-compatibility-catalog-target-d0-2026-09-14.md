---
Status: selected__source_admission_design__2026-09-14
Task: MIR-CALL-STATIC-COMPATIBILITY-CATALOG-TARGET-D0
Date: 2026-09-14
Parent: mir-call-r7-stringbox-lower-structural-membership-i0-2026-09-14.md
NextCard: none__mixed_source_co_seal_design
Implementation permission: false; user reopened design and taskization only
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

The next selectable slice is **A0-1 design**. A1 must consume A0's accepted
window; neither row grants implementation permission yet.

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

This is design-only. The A0-2-D design condition is now fixed above. The next
row is **A0-3**, which must define the source-backed root admission boundary
before any transport implementation or focused guard is selected.

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
| 2f — next, D0 | A0-3 source-backed root admission | Define how the allowed same-brand window enters `Initial`/`NormalSourcePlan` and how every compatibility/source sibling is consumed on reject. Do not switch production callers in this row. |
| 3 — conditional I0 | Switch accepted source cohort to existing package | Connect materializer admission to `PreparedNormalDefaultProgramRootV1::from_callable_source`, existing package issuer/collector and Cataloged static handoff. In the same slice retire that cohort's old compatibility classification/raw static-child dispatch. Scope the exact caller and branches after A0/A1 and A2/A3; no blanket root switch. |
| 4 — I0 acceptance | Prove publication, rejection and retirement | Real selected source reaches static publication; missing/foreign site, brand mismatch, missing/ambiguous target and unsupported source/result reject before argument effects. Existing owner guards prove selected old-edge absence and residual handling. |
| 5 — return to StringBox I0 | Close original owner acceptance | Run existing phase14/16/17 and its malformed/wrong-class/extra-argument/embedded/empty cases only after upstream reach is established. Record exact owner-to-terminal results; an earlier stop leaves this acceptance open. |

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
