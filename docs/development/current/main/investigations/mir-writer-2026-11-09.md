# MIR writer consolidation (2026-11-09)

Scope: main `aee4cdebc8`, branch `mir-writer`; acceptance, refusal text, EXE output and exit are invariant. This is implementation organization, not a language change. Source owners retain admission/ownership, and the final verifier remains independent. Read-only workers traced the following paths before implementation.

## Rules

W1. Each operation has one physical shape owner; callers supply operands, existing source/effect evidence, destination and Normal/Fault continuations rather than choosing another physical shape.
W2. Exact nonfaulting i64 Add/Sub/Mul and trivial comparisons remain raw; ownership-bearing or fallible operations retain their existing checked operation and cleanup evidence, and raw alone is never evidence of fault freedom.
W3. A checked expression's existing normal-result/operator/envelope evidence selects its representation once; moving this decision must preserve all source membership, consumer receipt and ordered-producer checks.
W4. Existing `ssot/binop_lower`, `ssot/cf_common`, builder emission and operation owners are extended; no parallel instruction family, opcode, ABI, descriptor or C change is introduced.
W5. Mechanical append policy remains with its existing prepared transaction: destination/block allocation order, span, type publication timing, ledger receipts, missing-block behavior and diagnostic wording must not change.
W6. Return itself adds no Fault edge; all pre-existing deferred cleanup, ownership transfer, Fault prefix and return_env obligations must finish through their existing owners before the same terminal is written.
W7. Raw StringBox/ArrayBox/Map calls and checked payload operations are different evidence domains; neither is silently reinterpreted as the other, and a common factory alone is not counted as unifying their semantic shape decision.
W8. Mutating passes, rematerializers, generated thunks and correspondence snapshots are listed separately from source entrance writers and never counted as removed entrance duplication.
W9. Each replaced writer/decision is removed in the same series, while source validation, strict prepared transactions and final independent verification remain intact.
W10. Acceptance/refusal wording and every measured EXE output/exit stay equal; changed MIR is enumerated with rule, writer and before/after instruction counts, and an unexplained heavier form is rejected.
W11. An operation needing changed acceptance/refusal, changed EXE behavior, a new public MIR/ABI or weakened verification is stopped individually with concrete evidence; estimates alone do not stop work.

## Entrance inventory at aee4cdebc8

Paths below are relative to `src/mir/`. A family is counted once even if several source callers reach it. Entries marked core/descriptor/pass/reader are not additional source entrances. Existing preflight/permission is retained during consolidation.

| Operation | Entrance writer file:line | Form | Selection evidence and callers |
|---|---|---|---|
| i64 arithmetic | builder/ops/arithmetic.rs:85,97 | raw BinOp | ops/mod.rs:157; ordinary source/LocalSSA, checked scalar and borrowed Mul completion |
| Loop Add/Sub | builder/emission/loop_operation.rs:40,115 | raw BinOp | exact Integer operands, static/dynamic selected Loop and existing S6C callers |
| unary Sub | builder/ops/unary.rs:175 | raw zero minus operand | existing core13 unary policy |
| Script Add/Sub/Mul | builder/script_physical_exit/direct_static_entry_kernel.rs:103,128 | raw BinOp | original ScalarOperandRecipeNode::Binary |
| terminal Add | builder/ordinary_new_admission/selected.rs:504,535 | raw BinOp | prepared two-field Integer terminal and original source receipts |
| resolved trivial arithmetic | builder/resolved_lowering/trivial_ssa/operation.rs:12,51 | raw BinOp | source representation and prepared result type, called by trivial lowerer:336 |
| CorePlan arithmetic | builder/control_flow/plan/lowerer/effect_emission.rs:328 | raw BinOp | CoreEffectPlan::BinOp, existing LocalSSA operand finalization |
| JSONv0 increment | ssot/loop_common.rs:7,41 | raw Add | lowering/stmts.rs:69 continue hint |
| arithmetic core (not extra entrance) | ssot/binop_lower.rs:21,53 | raw BinOp append | JSONv0 expr/binary_ops.rs:23 and ordinary arithmetic |
| checked Binary shape decision | builder/ordinary_new_admission/selected/expression_value.rs:579 | raw Integer / String Invoke / Dynamic operation | checked operation normal_result/operator/envelope; integer source receipt, expression cleanup |
| Compare | builder/emission/compare.rs:49 | raw Compare | ordinary comparison, Loop, unary Not, checked source conditions; prepared Bool type |
| strict Compare | builder/builder_emit_core.rs:75 | raw Compare | canonical_compare_writer.rs:25, strict CFG/SSA/unique-destination transaction |
| resolved trivial Compare | builder/resolved_lowering/trivial_ssa/operation.rs:55 | raw Compare | source representation and prepared result type |
| CorePlan Compare | builder/control_flow/plan/lowerer/effect_emission.rs:359 | raw Compare | finalize_compare, no additional Bool publication |
| JSONv0 Compare | ssot/cf_common.rs:25 | raw Compare | JSONv0 binary/match/sum/loop_range; absent-block no-op preserved |
| checked condition selection | normal_callable_semantic_package/ordinary_new_local_commit/root_call_entry/scalar_condition_operation.rs:126 | i64 raw or checked conversion plus Compare | source exact i64 vs carrier/Text/Map; expression_value.rs:714 consumer |
| exact field | builder/fields.rs:168,182 | raw ObjectFieldGet | source exact claim, field identity/type and destination |
| terminal Add field operands | builder/ordinary_new_admission/selected.rs:517 | raw ObjectFieldGet | prepared original field reads and ledger receipt |
| terminal field | builder/ordinary_new_admission/selected.rs:584 | raw ObjectFieldGet | prepared.checked false |
| argument field | builder/ordinary_new_admission/selected/arguments.rs:114 | raw ObjectFieldGet | I64Field source and receiver binding |
| legacy field | builder/fields.rs:444 | raw FieldGet or existing fastmem | source field origin/type; raw expression/ordinary field owner |
| CorePlan field | builder/control_flow/plan/lowerer/effect_emission.rs:304 | raw FieldGet | CoreEffectPlan field and normalized local base |
| checked field | builder/fields/checked_read.rs:71 | Invoke plus NormalResult | LocalFieldReadSource.requires_checked_field_read; distinct landings and cleanup |
| checked terminal field | builder/ordinary_new_admission/selected.rs:376 | root graph FieldGet Invoke | prepared checked source and root exit/Fault suffix |
| String length raw | builder/calls/unified_emitter/physical_terminal.rs:97 | Call(StringBox.length) | method_call_terminal.rs:455; CommonV2 length_call.rs:129 checks PureRead/non-suspending/StringBox/I64 |
| String length checked | builder/ordinary_new_admission/selected/expression_value.rs:696,755 | String Length Invoke plus NormalResult | exact core source projection expression_source_values.rs:395, owned temporaries/Home cleanup |
| Array raw read/count | builder/array_element_write.rs:121,135,145,154 | ArrayRead | original field recipe or known ArrayBox get/1; unified_emitter.rs:443, indexing.rs:196, boxcall facade |
| Array dynamic get | builder/ordinary_new_admission/selected/expression_value.rs:505 | DynamicArrayGet Invoke | original core contract, Integer index, DynamicCarrier cleanup |
| Array owned get | builder/ordinary_new_admission/selected/owned_array.rs:92 | OwnedArray Get Invoke | source holder descriptor, index receipt and Fault plan |
| Array raw append/set | builder/array_element_write.rs:56 | ArrayElementWrite | exact known method/arity or collection literal; mir/array_element_write.rs:59 instruction owner |
| Array field append | builder/ordinary_new_admission/selected/field_array_append.rs:45 | ArrayElementWrite Push Invoke | original append loan, pinned receiver and argument/unwind proof |
| Array owned append | builder/ordinary_new_admission/selected/owned_array.rs:199 | OwnedArray or DynamicCarrierArray Append Invoke | holder descriptor/candidate and cancellation cleanup |
| Array script literal | builder/script_array_control_emission.rs:99,152 | LiteralAppend Invoke | sealed recipe, ArrayInvokeSite and acquired prefix |
| Map ordinary set/get | builder/calls/unified_emitter/physical_terminal.rs:77 | Call(Method) | ordinary method source; ssot/method_call.rs:65; write metadata only after success |
| Map literal install | builder/ordinary_new_admission/selected/map.rs:52 | Map Invoke variants | sealed Map flow, key/value classes and installed-prefix cleanup |
| Map field set | builder/ordinary_new_admission/selected/map_field_install.rs:9 | Map FieldInstallIndexed Invoke | PreparedMapFieldInstall operation and End outcome |
| Map stored get | builder/ordinary_new_admission/selected/terminal_map_get.rs:10,74 | Map Invoke / root graph ingress | same original receiver/key proof, local vs Return continuations |
| Map bounded get | builder/ordinary_new_admission/selected.rs:616 | CheckedGetI64 Invoke | prepared terminal, Map Home End on Normal/Fault |
| Map nested read | builder/map_read_physical_consumer.rs:302 | Map ArrayIndexMap/ArrayLength/MapGetText Invoke | MapReadFacts row, original receiver/key; duplicates Invoke+Normal writer |
| Map keys count | builder/ordinary_new_admission/selected/terminal_map_get.rs:97 | Map FieldKeys Invoke then raw Array Count | original keys-count recipe and snapshot End |
| Return ordinary | builder/stmts/return_stmt.rs:163,201 | Return or defer Copy/Jump | raw value descent, protected-region defer; emit_instruction→canonical CFG append |
| Return canonical | builder/resolved_lowering/canonical_cfg/session.rs:165 | Return | strict prepared source preflight, distinct from selected Body append |
| Return root ownership | builder/ordinary_new_admission/selected/root_graph.rs:359 | cleanup Invokes then Return | finalized original root exit/Fault graph and result |
| Return Loop/CorePlan | builder/control_flow/plan/lowerer/exit_lowering.rs:39; effect_emission.rs:455 | Return | original loop exit values and canonical control session |
| Return wire | builder/control_flow/edgecfg/api/emit.rs:112 | Return with return_env | wired edge args, existing source checks and canonical commit |
| Return Script completion | builder/script_physical_exit/exit.rs:286; direct_static_entry_kernel.rs:211 | Return | prepared Script result/exit relation |
| Return raw root | builder/raw_root_body_exit.rs:277 | Const Void then Return | brand-bound root completion, signature and witness |

## Passes, descriptors and readers (not entrance writers)

- ssot::cf_common::append_instruction_core and canonical_cfg::commit_instruction are mutation cores; a shared mutation core does not by itself eliminate semantic writer duplication.
- builder/ssa/local/materialize.rs:537 and ssa/phi_input_materializer/edge_rematerialization.rs:192 re-materialize an existing definition; passes/inline_soft_leaf.rs:296,375 remaps already-built arithmetic/field instructions.
- passes/string_corridor_sink/concat_corridor_apply.rs:317 rewrites String corridors; passes/callsite_canonicalize/array_read.rs:33 rewrites a proved existing Call.
- ordinary_new_scalar_operations.rs validates originals/ordered producers; ordinary_new_field_reads.rs:632,655 snapshots existing reads; expression_value.rs:494 records an already-emitted ArrayRead.
- scalar_condition_operation.rs:118 creates a correspondence descriptor; physical_program_scalar, verification/invoke_map.rs:198 and verification/invoke_string_lifetime.rs:291 independently inspect output.
- Generated physical thunks and pinned-text finish/normalization adapters must retain their own completion proof; they are not additional source-expression admission owners.

## Implementation order

1. Unify exact arithmetic/Compare emission on existing SSOT, retaining all append/receipt/type transactions, then centralize checked arithmetic shape selection.
2. Reuse field append owners without changing early metadata publication; keep source-selected checked cleanup distinct until the same evidence reaches its common owner.
3. Remove duplicate Map Invoke writer, preserve existing Array single raw read/write owners, and investigate checked/raw evidence boundaries.
4. Unify Return at the existing canonical CFG boundary while preserving placement/span and strict vs selected preflight; never replace all Returns with the strict emitter blindly.
5. Verify acceptance/refusals and outputs against the supplied base, enumerate all MIR changes and remaining evidence boundaries in the final report.

## Consolidated boundaries

- Arithmetic raw construction is in `ssot/binop_lower::emit_binop_with`; checked Binary shape selection is in existing `builder/ops/arithmetic::emit_checked_binary_v1`. The expression descent no longer independently chooses Integer/String/Dynamic emission. Ordinary unqualified Add is not promoted into a forged Integer proof.
- Compare construction is in `ssot/cf_common::emit_compare_with`; strict prepared append and ordinary Bool publication retain their original transactions.
- Field construction/dispatch is in existing `builder/fields`: raw exact reads 4→1, legacy raw field reads 2→1, checked Scalar operation projection 2→1. Terminal raw/checked dispatch moved from selected into fields/checked_read; its existing prepared source/root cleanup is retained. Local/Terminal/shared source proofs remain distinct.
- Known Array write/read call fronts share the existing Array owner. The CorePlan write-only Array-like proof is preserved, and all three call fronts share successful-write observation/receipt publication. The old `try_emit_known_array_method_write` wrapper is removed. Raw Array read and write primitives were already single owners.
- The Map read consumer no longer hand-writes Invoke/NormalResult. Selected operations, expression/field checked reads, script Array control and root packets share the existing Invoke layout/control writer; result projection and cleanup keep their prepared transactions.
- Return construction is in `cf_common::return_instruction` and installation in `append_return_to_block_core`; canonical `commit_return` also preserves predecessor/edge truth. Strict vs selected preflight, append vs replacement, current vs fallback span and return_env remain unchanged. Source completion/cleanup owners are not claimed to have merged.

## Stopped semantic representation boundaries (W7/W11)

- String length retains raw Call and checked Invoke rather than forcing one form: raw target proof is discarded at source_call_publication.rs:154/expression_port/source_method.rs:8, while checked expressions retain their original Core source loan. More decisively, compiler/normal_default_pipeline/published_backend_view/physical_program_json.rs:434 rejects non-Birth raw Call as `call-not-birth`. Lowering a source-safe checked length to raw Method Call would require extending the lifecycle serializer/C acceptance, prohibited in this series; converting every raw length to Invoke would add unproved Fault/cleanup overhead. No fabricated expression loan or C change is used.
- Ordinary raw Map set/get retains its ABI. generated/core_method_contract_rows.rs:88–106 has Unprojected semantic laws, and the raw terminal keeps MirCall/effects/post-success observations, not key-domain, nullable-result, ownership or Fault-free evidence. These cannot impersonate a checked Map field/slot source, typed descriptor or Dynamic carrier. Map's source-qualified operations are shared physically, but replacing the ordinary runtime result protocol needs source/representation work outside this no-ABI-change series. Effects/receiver names are not substituted for proofs.
- Owned Array and raw Array remain distinct operand/ownership contracts; no raw access is declared Fault-free merely because its opcode is raw. Their existing source-selected operations keep the same shared physical writer and independent checks.
