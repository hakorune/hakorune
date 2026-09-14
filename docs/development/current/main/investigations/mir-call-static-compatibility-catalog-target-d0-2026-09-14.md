---
Status: selected__source_admission_design__2026-09-14
Task: MIR-CALL-STATIC-COMPATIBILITY-CATALOG-TARGET-D0
Date: 2026-09-14
Parent: mir-call-r7-stringbox-lower-structural-membership-i0-2026-09-14.md
NextCard: none__await_source_admission_decision
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
| 1 — next, D0 | Pin failing ingress and source disposition | Trace the existing phase14 smoke, `tools/lib/program_json_v0_compat.sh`, fixture helper and normal materializer. Identify the exact compiler invocation, input source, parser cohort, transform reason, bundle/import lineage and typed-origin versus AST-only branch. Record one entry-to-static-terminal chain; if static evidence is insufficient, name a focused diagnostic probe for separate execution selection. |
| 2 — D0 | Decide source admission for that whole cohort | Name declaration/caller/site, import, post-transform, argument/type/result issuers and consumers. Cover method bodies, loops, conditionals, early returns and transferred/opaque subtrees. Use source types/contracts for `length`/`substring` demands; do not infer from integer returns or MIR. Resolve each unsupported partition explicitly. |
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
