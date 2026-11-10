# StringIndexOf and shared Text borrowing

Base: `a58caad795`. Branch: `string-index-of`. Evidence `$L`; immutable baselines `$FR`, `$SL`, `$ME`, `$NF`.
Read-only workers traced source, verification, C and runtime before implementation.

## Rules

I1. Only the exact original CodePointSearchFromZero law with two live Text operands permits a frame-free IndexOf; frame_free_read_law_v1 is the sole source query, never opcode names or PureRead.
I2. The existing Core projection returns one private read plan retaining its law, receiver and optional needle; writer and source correspondence consume that plan rather than deriving a second decision.
I3. Receiver and needle are evaluated once each in source postorder, receiver first; existing values, formal/Copy origin, lease, source site and use position are retained.
I4. Independent Text-read verification is implemented before a production writer selects the new instruction; single and double operand reads share domain, availability, liveness and I64-result-origin checks.
I5. Only StaticText, OwnedString, BorrowedText and TextOperand are legal for both operands; End, pending/Fault state, wrong provenance/type, duplicate definition and use-before-definition remain errors.
I6. Repeated operands are legal and validated twice; SSA use occurrence counts retain both positions rather than deduplicating them.
I7. StringIndexOf is a nonterminator with dst/value/needle and I64 CP result, no frame/site/Normal/Fault edge and no ownership acquisition, transfer or consumption.
I8. The v2 JSON row has exactly op/string_index_of,dst,value,needle; extra edge/frame keys and non-Text operand kinds are rejected.
I9. Runtime returns 0 with the CP index or 2 with out unchanged; empty needle is 0 and absent needle is -1, without source Fault recording.
I10. A shared private borrow helper validates one or two operands under one lease-table lock then one registry read lock, checking every token/handle/generation/exact Text before invoking a non-reentrant callback.
I11. The callback borrows strings without whole-String cloning, publication or drop; result storage is written only after both guards have been released.
I12. C validates both Text operands at their actual row, keeps prologue-only allocas, names operands/status/labels by function/block/row/role, and routes nonzero status to the existing invalid exit.
I13. Existing edge forwarding remains the PHI owner when one MIR row expands into multiple LLVM blocks.
I14. Each executable Normal and surviving Fault path preserves acquisition/transfer/End responsibilities and order; static End counts are evidence rather than an equality gate.
I15. Operand concatenation, calls, field materialization and End keep their own Fault/cleanup owners; IndexOf adds no private cleanup stack.
I16. Old checked IndexOf retires only after all producers/consumers/tests/headers are migrated and exact old-operation callers are zero; raw and two-argument compatibility routes remain for 3b.
I17. Source acceptance and existing EXE output/exit stay fixed; incidental newly executable sources are listed individually, and no heavier MIR is accepted.
I18. Functionality and cleanup normalized production/test LOC are separate; cleanup includes all shared-code destinations and must be net negative without credit for movement/renaming/formatting.

## Base route and responsibilities

|Stage|Base location|Responsibility|
|---|---|---|
|Source order|resolved_value_profile/expression_children.rs:146; expression_plan.rs:204|Receiver then arguments, one ordered postorder|
|Core source|normal_callable_semantic_package/core_method_source/ordinary.rs:70|Original method pointer, manifest, target and exact law|
|Source law/frame|resolved_semantics/core_method_instance_target.rs:436,455; home_expression_evaluation.rs:33|One exact-law query, retain other frame reasons|
|Core projection|ordinary_new_local_commit/root_call_entry/expression_source_values.rs:405|Existing sole projection; checked IndexOf producer at454|
|Writer/correspondence|builder/ordinary_new_admission/selected/expression_value.rs:522; expression_source_values.rs:219|Same plan, actual instruction and retained evaluation|
|Other consumers|expression_projection.rs:247; expression_effect_order.rs:40|Array bindings and ordered effects use the same projection|
|Independent verifier|verification/invoke_string_read.rs; invoke_string_lifetime.rs:285,415,440,477,531; invoke.rs:343|Shared domains/liveness/order/I64 origin and activation|
|General MIR|instruction/methods.rs; query.rs; value_consumer.rs; joinir remappers; SSA/dom|Both operand occurrences, definitions, types and visitors|
|Publication|compiler/normal_default_pipeline/published_backend_view/physical_program_json.rs:386|Strict normal v2 row; existing readonly borrowed uses|
|C|published_mir instruction_schema, admission, native_admission, indexed_rows, string_emit, emit|Strict row/live checks, shared row machinery, forwarding|
|Runtime|runtime/dynamic_v2_lease.rs:95; host_handles/lease_identity.rs:111,210; kernel exports/fault_checked_string.rs|lease then registry, exact generation/Text, borrow-only leaf|

Unqualified Rust paths are below src/mir; expression files are in the root-call-entry directory above. C files are under lang/c-abi/shims/published_mir.

## Loop and cleanup

Loop TextIndexOf is a portable recipe node, not another MIR producer:
source_loop_bool.rs:125 -> source_loop_v2_projector.rs:185 -> source_loop_schedule.rs:349 -> source_loop_frames.rs:472 -> lower_checked_bool_condition_v1 -> selected comparison inputs -> the same expression writer.
The recipe's conservative effect summary must not become a second source-law authority. Existing unsupported exact-I64 comparison scopes are not opened to manufacture a loop example.
For concatenation, expression_value/invoke.rs retains its acquisition and fault cleanup; fault_cleanup_plan.rs:35 retains reverse temporary order. Only IndexOf's own edge disappears.

## Forms and evidence

|Form|Rules|
|---|---|
|Static/Owned/Borrowed/formal/Copy/loop Text operands|I1-6, I10-13|
|Same Owned twice|I6, I10-11; both positions checked under one guard|
|Unicode/empty/absent/longer needle|I7, I9; no byte-profile fallback|
|Multiple rows, loop, PHI|I3-6, I12-13; actual current source acceptance is measured first|
|Concatenated receiver or needle|I3, I14-15; their own Fault and End remain|
|Nonroot frame removal|I1-2, I14; Root ABI and other fault reasons remain|
|Malformed descriptors, stale tokens, ended/foreign Text, forged JSON/SSA|I4-13; independently rejected|

## Stages

1. Shared verification and negative evidence before source selection.
2. Shared one/two-Text borrowing, runtime leaf, instruction plumbing, C/JSON output.
3. One Core plan/writer and sole law query; migrate all checked producers.
4. Caller-zero retirement and generated manifest; retain raw/compat callers with locations.
5. Update ABI/String reference, then freeze source and run final verification once.

Final evidence includes all1,788 dumps and changed-input metrics, new/old EXE pairs, existing229/apps100/384/suite/Opus/unseen18/String groups, runtime checked/helper tests with timeout, C physical tests, full lib failure-name comparison, Python21 (including the four baseline missing captures), guards and formatting.
Baseline FFI/runtime are copied before any build and their expected hashes match. Final reports record actual Normal/Fault responsibility changes, feature/cleanup LOC, all retired or moved test names, and three timings per old/new loop EXE. No speed claim precedes measurement.
