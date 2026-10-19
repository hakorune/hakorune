# MIRBUILDER-UNTYPED-OBJECT-STORAGE-D0

Status: ReopenedDesign on 2026-10-19; common acceptance critical path.
Implementation permission: false until this D0 accepts a source-backed contract.
Scope: unchanged untyped-field sentinel and the three remaining real apps; one common D0.
Authority: [final pipeline](../design/mirbuilder-final-pipeline-ssot.md),
[RULES](../../../RULES.md), [EBNF stored-slot semantics](../../../../reference/language/EBNF.md),
[Home field/empty-slot contract](../../../../reference/language/ownership.md).

## Accepted decision and current evidence

- 2026-09-25: birth-param fixture migrated to explicit fields/result and lifecycle EXE.
  That historical exception does not authorize rewriting the unchanged untyped-field sentinel.
- Earlier 2026-10-19 decision retained ParkedSealed while other apps advanced.
  Latest user feedback reopens this common design decision before further app expansion.
- Direction selected: preserve the existing dynamic/untyped language with tagged dynamic
  storage. Annotation-required rejection is not the unchanged-source acceptance goal;
  source-qualified fixed storage alone does not implement dynamic slots.
  Representation direction is decided; the complete Home/storage contract is not accepted.
- EBNF stored semantics and legacy init-list semantics already declare these slots untyped.
  This does not decide empty state, Home ownership or checked-read behavior by itself.
- Shared dependency is present in binary-trees (TreeNode), boxtorrent-mini (Chunk/Store/
  Torrent), JSON (UserStats/Aggregator), and the sentinel. Their current first-stops differ;
  this is a necessary common dependency, not proof that storage alone completes all four.
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
| Keep parked | Preserve current unsupported boundary while other work advances. | Former selection, superseded by the common design review. Implementation remains closed until the full contract is accepted. |

## Contract acceptance and implementation gate

The design review is reopened. Implementation opens only when this same D0 records:

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

## Priority and concrete remaining decisions

This is the next semantic decision, not a follow-up after every app finishes.
Resolve these in this D0 and the owning reference contracts before storage code:

| Decision | Required source-backed resolution |
| --- | --- |
| Initial state | Distinguish a never-written field from language null and local UninitializedSlot. Decide read-before-write and partial construction without copying a VM default. |
| Store ownership / aliases | Select each allowed owning or anchored-borrow destination from source. Ordinary handles do not become owners; payload kind alone never selects transfer/share. Cover constructor formals and local-alias-to-field writes used by unchanged apps. |
| Replacement / failure | Define prepare/commit order, old-value end, failed RHS/write, and exactly-once Normal/Fault cleanup across scalar/handle kind changes. |
| Reads / teardown | Decide borrowed versus owning result, checked I64/Bool/Text/object conversion and anchor validity; destruction uses the same accepted slot relation. |
| Physical closure | Map source issuer → layout → read/write → cleanup → both final consumers → selected C/runtime ABI. A call transport tag or legacy VMValue is not a field contract. |

- Keep all four sources/expected outputs and existing safety boundaries unchanged.
- Resume implementation only after full acceptance above; direction alone grants no ABI.
- JSON Loop Call work is independent but does not remove this dependency. Its unverified
  source-only edits are retained in Git stash with message
  `JSON Loop Call source connection pending common storage decision`; no build/test ran.
- No new campaign, checker, test file or app rewrite is selected. Historical rationale
  and the earlier parked decision remain in this file's Git history.
