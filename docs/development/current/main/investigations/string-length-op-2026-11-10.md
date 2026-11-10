# StringLength: frame-free code point read (2026-11-10)

Base: `45a605118e`. User decision: survey §ChatGPT Proへの相談とユーザー決定.
Scope: existing StringLen source laws; source acceptance and Unicode semantics stay unchanged.
Logs: `$L`; baseline `$P`; survey `$B`; instruction/report `$S`.

## Rules

SL1. StringLen/0 uses the exact existing CodePointCount law and returns I64 Unicode code points.
SL2. NoSourceFault comes from the original source loan, exact Text receiver and live read permission, never opcode/effect/name alone.
SL3. The common evaluator evaluates the receiver once in source order and retains acquisition, field/concat/call Fault and temporary End responsibilities.
SL4. `StringLength { dst, value }` is a nonterminator without frame or Normal/Fault edges; Root ABI and other operations keep their frames.
SL5. One Core projection selects normal read versus Invoke for ArrayCount and CodePointCount; consumer purpose supplies only the destination.
SL6. Length acquires, transfers and consumes no ownership; acquisition/transfer/End responsibilities and order remain unchanged on every executable Normal and surviving Fault path. Static End/cleanup rows exclusive to the removed Length Fault edge may disappear and are reported per input (maintainer clarification, 2026-11-10).
SL7. The existing `(block, faulted)` Live solver checks Text readability at each row and Invoke; borrowed origins retain exact marked formal and unique Copy-chain checks.
SL8. Only StaticText, OwnedString, BorrowedText and TextOperand are legal; definitions, types, dominance, same-block order and lifetime checks remain independent.
SL9. Runtime `nyash.string.code_point_count_v1(value, out)` returns 0 with the count or 2 without touching out; it never records a source Fault or returns 1.
SL10. Runtime counts inside the existing borrowed Text callback without full cloning, acquisition/consumption, publication, drop or reentry, and writes out after unlocking.
SL11. Physical program v2 adds exactly `{op:string_length,dst,value}`; C validates each row and sends nonzero status to the existing invalid exit.
SL12. C temporaries/globals/labels are unique by function/block/row; storage is prologue allocated and PHIs use existing edge forwarding.
SL13. All old checked Length producers migrate before deleting its enum arm, JSON/C reader/emitter and checked_length_v1 export/header; real Fault operations retain injection/cleanup evidence.
SL14. Source raw StringLen selection retires; unrelated byte/compat helpers stay only with concrete callers, while b=0, unchanged EXE and nonincreasing MIR are verified against `$P`.

## Route and connection points (base revision)

| Responsibility | Source location | Connection |
|---|---|---|
| Exact law | `src/mir/resolved_semantics/core_method_instance_target.rs:429,450` | ArrayCount and CodePointCount source query |
| Frame demand | `src/mir/resolved_semantics/home_expression_evaluation.rs:47`; `src/mir/normal_callable_semantic_package/ordinary_new_local_commit/root_home.rs:345` | Same law query, retain other Fault demand |
| Core issuer | `src/mir/normal_callable_semantic_package/core_method_source/ordinary.rs:70,168` | Exact original method/site/plan and receiver loan |
| Sole checked Length producer | `src/mir/normal_callable_semantic_package/ordinary_new_local_commit/root_call_entry/expression_source_values.rs:393,426` | Sole normal/Invoke Core projection |
| Writer | `src/mir/builder/ordinary_new_admission/selected/expression_value.rs:522` | One StringLength writer; shared normal-Core answer |
| Correspondence | `expression_source_values.rs:224`; sibling `expression_projection.rs:247`; `expression_effect_order.rs:47` | Consume same projection, remove Array-only bypasses |
| Lifetime entry | `src/mir/verification/invoke.rs:328`; `invoke_string_lifetime.rs:281` | Activate for ordinary Text read as well |
| Borrowed origin | `src/mir/verification/invoke_string.rs:104` | Explicit formal + unique Copy chain |
| Read state | `src/mir/verification/invoke_string_lifetime.rs:534,550,610` | Shared readable query; transitions stay Invoke-owned |
| Publication | `src/mir/compiler/normal_default_pipeline/published_backend_view/physical_program_json.rs:383`; `physical_program_json_operation.rs:97` | Nonterminator v2 row; remove old operation |
| C SSA definitions | same directory `hako_llvmc_ffi_published_lifecycle_physical_v2.inc:129` | Register ordinary result definitions before kind/state validation |
| C schema/state | `lang/c-abi/shims/published_mir/hako_llvmc_ffi_published_lifecycle_instruction_schema.inc:253`; `hako_llvmc_ffi_lifecycle_v4_admission.inc:190` | Exact keys/domain and readable state at row |
| C emission | same directory `hako_llvmc_ffi_lifecycle_v4_string_emit.inc:21,85,214`; `hako_llvmc_ffi_lifecycle_v4_emit.inc:425,720` | Prologue storage, unique names, existing invalid path |
| Runtime | `crates/nyash_kernel/src/exports/fault_checked_string.rs:61,132,234`; `src/runtime/dynamic_v2_lease.rs:92`; `include/nyrt_fault_v1.h:148` | Borrowing count ABI replaces frame-dependent export |
| Raw selection | `src/mir/generic_method_route_plan/collection_read_routes.rs:456` | Remove StringBox length arm only |

## Accepted forms and governing rules

| Form already source-proved | Rules |
|---|---|
| Static/local String receiver; local/return/print/condition/argument destination | SL1–5, SL7–8 |
| Multiple reads in a block or result through PHI | SL5–8, SL11–12 |
| Borrowed formal and Copy aliases | SL7–8; same marked-formal origin |
| Owned/field-loaded/concat result receiver | SL3, SL6–8; preserve receiver acquisition and End |
| Typed TextOperand selected by existing loop PHI | SL7–8; same descriptor-live law |
| Dynamic or unrelated same-name method | SL2, SL8, SL14; preserve existing refusal |
| Malformed JSON, ended Text or forged borrowed origin | SL7–12; reject independently, never source Fault |

## Stages and verification

1. Shared Text-read verification, retaining Invoke transitions.
2. Instruction, general MIR plumbing, JSON, C and borrowing runtime ABI.
3. Sole Core projection/writer and shared frame demand; retire old checked/raw selection in this series.
4. ABI/String reference update; final verification uses final commit binary only.

Final evidence: all 1,788 dumps with b=0 and complete lighter-MIR census; prior 45 checked EXEs,
36 matrix probes, Unicode five, all changed-MIR compilable inputs, 229/apps100/384/suite8/Opus/unseen18;
full lib failure-name comparison, Python v4, naming/pointer/rustfmt/diff checks.
Feature and cleanup normalized production/test LOC are separate; cleanup must be net negative.
The earlier normalization exception remains bounded by dyn_locals instructions83/blocks61/Invokes22;
it does not permit a heavier StringLength transformation.

## Retained compatibility contracts

The source StringBox arm in `match_generic_len_route` is removed. The shared
`GenericMethodRouteKind::StringLen` descriptor remains for actual C compatibility
consumers: `lang/c-abi/shims/hako_llvmc_ffi_same_module_generic_method_string_emit.inc:28,83`
and `hako_llvmc_ffi_same_module_generic_method_collection_emit.inc:607` use the
byte helper (including substring omitted-end and indexOf bounds).
`hako_llvmc_ffi_generic_method_len_policy.inc:196,200` also retains compatibility
length. These callers keep `nyash.string.len_fast_h`; they do not authorize a
canonical source fallback. The `.hako` CoreMethodContractBox StringLen row is
retained because the generated manifest supplies the new operation's exact
CodePointCount source law, independently of that compatibility helper symbol.

C verification exposed pre-existing fixture/consumer mismatches: indexed Copy
is a readonly descriptor alias, not End authority; Substring indices use the
existing operand descriptor ABI. Their rejection/cleanup evidence is preserved.
The native Array release emitter now guards its indexed-only Map query before
using the unchanged native release path; native Substring indices require I64.

## Opus の確認（2026-11-10）

Codex の実装（`06b56b30c7`〜`50471fd1cc`）と仕上げ（`acebaf066b`、test・README・生成manifestだけ）を、main `45a605118e` から監査して fast-forward した。

- 行数（rustfmt後）: 機能の追加は production +468・test +254、整理は production −43・test −6。Opus の数え方（`src/` だけ、file単位）でも食い違いはない。差はinlineのtest moduleの扱い。800行を超える変更productionは無い。
- 構造: 枠なしで書けるかの法則は `frame_free_read_law_v1`（`core_method_instance_target.rs`）1つで、Arrayのcountと共有する。frameの要否（`home_expression_evaluation.rs`）、唯一のCore projection（`expression_source_values.rs`）、writer（`expression_value.rs`）、sourceの対応検査が同じ答えを使い、旧`array_count_receiver_v1`は消えた。
- 独立検証: 新しい`invoke_string_read.rs`が、通常行とInvokeのTextの読みの検査を共有する。End後の読み、定義より前の読み、Bool・型の偽装、terminatorとしての行を拒否する。
- C・runtime: Opus とは別の reviewer に読ませた。約束からのずれと検査の弱化は無かった。受理が広がったのはreadonlyの許可リストの1か所で、readonlyの本体はstaticか借りた仮引数のTextしか持てないので安全。借りたTextとCopy carrierの正の例、static textの値、不正なdescriptorでstatus 2と出力不変の確認は、仕上げで足した。
- 検証（Codex のlogを読んだ）: 1,788本で新規受理0・後退0・文言変化0・軽量化38。全libの赤228件は基準と名前が一致。Opus の未見の18本（`$C/sl-audit/in/s01〜s18`、`len()`の別名、自分の`length`を持つuser box、連結の結果を2回数える、絵文字と修飾子など）は、前と後で受理と出力がすべて同じ（通る6本、止まる12本）。
- 残り: 13本はまだ raw の`call_method StringBox.length`を出す。`return s.length()`のようにReturnのrootがMethodCallの形を、共通の評価器が選ばない（`home_terminal_scalar_evaluation.rs:35–79`）ため。正規のEXEの道では全部拒否され、byteの数は答えにならない。互換のCの道（`hako_llvmc_ffi_mir_call_route_policy.inc:204`から`len_fast_h`）も残る。前者はbasic-exprの残り、後者は3bの旧経路の退役で消す。
