# MIRBUILDER-UNTYPED-OBJECT-STORAGE-D0

Status: ParkedSealed; 2026-09-25 decision retained on 2026-10-19.
Implementation permission: false until this D0 accepts a source-backed contract.
Scope: unchanged `typed-object-untyped-field-min`; no new campaign or implementation card.
Authority: [final pipeline](../design/mirbuilder-final-pipeline-ssot.md),
[RULES](../../../RULES.md), EBNF legacy stored-slot semantics.

## Accepted decision and current evidence

- 2026-09-25: birth-param fixture migrated to explicit fields/result and lifecycle EXE.
  That historical exception does not authorize rewriting the unchanged untyped-field sentinel.
- 2026-10-19 user decision: retain ParkedSealed and advance another acceptance app.
  Recording alternatives below is not acceptance of an implementation or language change.
- Source `init { count, items }` declares untyped stored slots. `count = 7` and
  `items = new ArrayBox()` through a local do not declare fixed field storage.
- Both suite precheck and direct `--backend mir --emit-exe` stop at
  `return_type_strategy.rs` with `[phase84-5] Type inference failed for ValueId(14)`.
  Direct EXE exits 101 before producing an artifact. This is not a link/runtime failure.
- `fields/post_success.rs` deliberately publishes no declared type for an untyped read.
  Canonical layout and destruction reject unavailable field types; MIR observations cannot repair them.
- Handle-field owners now cover declared object/Array fields and source-qualified birth-side
  direct Array acquisition. Main-local alias-to-field writes in this app are outside that profile.
  Their existence alone does not satisfy this sentinel's reopen condition.

## Alternatives to decide

Costs are relative engineering scope, not measured completion dates.

| Option | Semantic effect | Implementation cost / affected owners |
| --- | --- | --- |
| Tagged dynamic slot | Preserve untyped slots, including value-kind changes. Define empty/uninitialized state, checked reads/conversions, replacement ownership, Normal/Fault cleanup and object destruction. A handle and an integer need distinct tags. | High: source slot issuer, layout/tagged ABI, write/read Recipe and verification, runtime/C consumer, cleanup and publication must agree end to end. Reuse existing tagged values where contracts match; no parallel value system. |
| Annotation-required rejection on canonical EXE | Require explicit field types for this lane. Specify an early diagnostic instead of panic. If made a language-wide rule, revise legacy compatibility syntax and migration policy. | Smaller compiler scope, but language/acceptance policy changes are substantial. The unchanged exit-7 fixture remains failing; converting it to an expected refusal requires explicit acceptance changes and is not product success today. |
| Closed source-qualified fixed profile | Admit only fields proven by a complete original writer inventory, exact acquisition/receiver identity and Normal dominance. Define escapes, aliases, conflicting writes and replacement behavior. | Medium to high: may reuse field-writer and Array residence owners, but introduces a new source contract. It is not generic dynamic-slot support and cannot replace that requirement merely because this fixture passes. |
| Keep parked | Preserve current semantics and explicit unsupported boundary; continue other apps. | Selected now. No dynamic-slot implementation, source migration or new inference mechanism. The sentinel remains red and the full goal remains incomplete. |

## Reopen trigger and acceptance

Reopen only after this existing D0 explicitly accepts one option and records:

1. Source authority and slot/result representation; no classification from MIR types,
   receiver names, backend success or C defaults.
2. Construction/initial state, field replacement, alias/escape behavior, handle Home transfer,
   destruction and Normal/Fault cleanup, including failed writes and failed reads.
3. Existing issuer, sole runtime/backend consumer and final verification/publication path;
   unsupported shapes retain a deliberate refusal boundary.
4. Unchanged source/exit-7 acceptance for a support option, or explicitly approved
   language/reference and suite changes for annotation-required rejection.

D0 acceptance authorizes implementation; this option record does not.
Do not suppress the panic by inventing I64, revive phase-293x MIR storage inference,
weaken Text/Array negatives, or rewrite the app to evade this decision.

## Next action

- Keep the fixture and existing safety boundaries unchanged; do not count it as PASS or SKIP.
- Continue `APP-allocator-stress` by the user's 2026-10-19 selection override.
- Historical D0 rationale and birth-param migration details remain in this file's Git history.
