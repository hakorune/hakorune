---
Status: selected__source_admission_design__2026-09-14
Task: MIR-CALL-STATIC-COMPATIBILITY-CATALOG-TARGET-D0
Date: 2026-09-14
Parent: mir-call-r7-stringbox-lower-structural-membership-i0-2026-09-14.md
NextCard: none__resolve_mixed_cohort_source_authority
Implementation permission: false; user reopened design and taskization only
---

# Static compatibility source admission and handoff

## Six-line brief

```text
Decision: design source-backed admission through existing package/publication owners; Windows is deferred and a catalog-only shortcut is not selected.
Source authority + canonical issuer: parser lineage and transformed source admitted by the normal materializer/package; ScriptDirectStaticCallLookupIssuerV1 issues target/result publication from that package.
Non-authority: AST possession, ScriptRoot location, names/arity, declaration existence, MIR, numeric sites, test results and Windows capability.
Fail-fast boundary: missing source/target/result relations reject before argument descent or MIR effects; compatibility cannot borrow a fabricated publication owner.
Smallest next slice: pin the failing compiler invocation and its source-classification reason, then decide its complete source-window admission in this D0.
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
| 2b — next, D0 | Choose the mixed-cohort authority boundary | Decide whether to generalize the parser source-seal/static-parent issuer for the whole merged invocation or define a complete source-backed sub-cohort with explicit compatibility rows. Name every issuer/consumer, import relation, body/loop/conditional/early-return coverage, transfer/opaque disposition and the old edge that would be deleted. No implementation or caller switch until this Decision is accepted. |
| 3 — conditional I0 | Switch accepted source cohort to existing package | Connect materializer admission to `PreparedNormalDefaultProgramRootV1::from_callable_source`, existing package issuer/collector and Cataloged static handoff. In the same slice retire that cohort's old compatibility classification/raw static-child dispatch. Scope the exact caller and branches after tasks 1–2; no blanket root switch. |
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
