# Static CurrentOwner closed-cohort packet D0

Status: selected design
Date: 2026-10-10
Scope: MIRBUILDER-STATIC-CURRENTOWNER-CLOSED-COHORT-D0
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/investigations/mirbuilder-gate1-object-guarded-outgoing-transport-d0-2026-10-10.md
  - src/mir/normal_callable_semantic_package/README.md

## Production dependency

The unchanged mimalloc-lite EXE still fails at
`ordinary-new/borrowed-entry/source-only-object-actuals` on
`Heap.allocate -> small_page.allocate(size)`. That exact Page call is a
downstream symptom: `LayoutBox.class_id`, `SizeClassBox.good_size`,
`size_to_bin`, `normalize_size`, and `bin_size` all have original borrowed
source drafts, but none has executable Static transport in the current
closed-App EXE package. `Page.allocate` does have an executable borrowed
definition in the imported-source diagnostic. No source or runtime change
has landed for this D0.

The Static seed currently requires a Qualified incoming call, no
CurrentOwner caller to the same callee, and no unsupported Static context.
`normalize_size` has only the selected CurrentOwner local initializer from
`size_to_bin`; `size_to_bin` also has a selected condition-expression caller,
and `bin_size` has a selected condition-expression caller. The existing
`UnresolvedArgument` fixed point then removes `good_size`, `class_id`, and
`Heap.allocate`. Relaxing only the seed predicate would admit incomplete
physical obligations.

The real imported-source test `real_mimalloc_incoming_domain_keeps_all_callers_without_false_stored_veto`
was run three times with temporary, removed diagnostics under `--profile
quick -- --nocapture`; each selected 1/1 test, with 9078 others filtered.
The current closed-App EXE observation was:

| Original call | Home local observation | Complete selected callee cohort |
| --- | --- | --- |
| `size_to_bin -> normalize_size`, Body(0) local initializer | yes | 1 |
| `good_size -> size_to_bin`, Body(0) local initializer | yes | 2 |
| `SizeClassBox.accepts -> size_to_bin`, Body(0) IfCondition Lhs | no | same 2 |

These cohorts come from `PreparedBorrowedFormalIngressV1::static_incoming_cohort_v1`
over the original selected-caller incoming inventory. The method retains
the original `Rc`, target, site, arity, and vetoes without changing the seed.
Temporary traces were removed; the tracked source is clean. The observations
establish source eligibility, not an executable actual or physical packet.

## Integrated Decision

Source authority + canonical issuer: keep
`QualifiedStaticCallClaimIndexV1::issue` and its original
`StaticIncomingSourceV1` for each exact Static target, with the existing
borrowed incoming scan as sole selected-caller cohort issuer. Home's
`LocalCallObservationV1` owns the selected local initializer and ordered
arguments. The existing `CallPacketSourceV1::Static` path and its publication
handoff are the physical packet owner to extend only after the exact source,
entry carrier, result Completion, and Normal/Fault coordinates agree.

Non-authority: `SourceStatic` pending arguments have no executable opaque
actual; candidate Integer agreement, an I64 result claim, a same-named
call, and the Loop-only selected packet cannot supply one. The
`class_id(size)` result does not classify its original `size` carrier.

Fail-fast boundary: preserve the current `source-only-static-actuals` and
`source-only-object-actuals` rejections until the selected original call and
**whole callee cohort** have one checked entry/actual/packet handoff. Wrong
target/site/ordinal/brand, missing or duplicate cohort rows, or a non-I64
tag at the first numeric payload use must reject before a payload read. No
source AST rewrite, fallback, or name-specific admission.

Smallest next design cell: `size_to_bin -> normalize_size` at its original
Body(0) local initializer, whose selected `normalize_size` cohort has one
caller. Freeze the exact `SourceStatic` to executable-actual mapping,
tagged forwarding/entry check, I64 result publication and Completion, and
Fault edge using existing packet and physical owner. Then select an S0 card
for that one cell. The two-caller `size_to_bin` cohort cannot be promoted by
the `good_size` initializer alone: its condition-expression caller lacks a
Home observation and needs a separate selected condition-call design (or a
separately proved closed-App omission of that caller). `bin_size` condition
calls and the later Heap-to-Page object outgoing actual remain separate.

The selected mapping must preserve `SourceStatic` staging until the physical
signature and callee Completion are available. Adding CurrentOwner directly
to `prepared.incoming` would make `prepare_borrowed_call_actuals_v1` call the
generic constructor too early; that constructor requires Qualified source.
The post-signature finisher may turn only the original, one-caller
`normalize_size` row into an executable checked actual. It must corroborate
the original `Rc`, Home observation and ordered argument, target/site/ordinal,
formal contract and checked I64 input, exact-I64 result, physical signature,
and Completion before joining the existing borrowed entry. The resulting
packet must use the existing `CallPacketSourceV1::Static` and its original
publication handoff, with tagged entry validation before numeric payload use
and the existing Normal/Fault result edges. This is a narrow completed-row
exception at the generic actual constructor, borrowed-entry target loan, and
Static packet validation; merely widening `require_qualified()` or the
publication predicate would admit an unproved CurrentOwner call. The
zero-input Static finisher is an identity-checking precedent, not an argument
or carrier proof for this one-input row.

Acceptance for the first S0: the unchanged imported source retains the exact
one-caller `normalize_size` cohort and original source facts; a selected
CurrentOwner local initializer gets a checked tagged actual and sole packet;
wrong/foreign source, changed ordinal/target, and non-I64 payload reject;
unselected callers stay source-only. Run the existing Static source/packet
positive and negative families, physical JSON/ABI validation required by the
chosen owner, then the unchanged app first-stop probe. Do not count source
receipt or fixture success as the app advancing.

Non-claims: this D0 does not authorize Static condition lowering, Page
outgoing transport, generic reachability pruning, or completion of the
MirBuilder goal. The object-outgoing D0 remains open as a dependent row.
