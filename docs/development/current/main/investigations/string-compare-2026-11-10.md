# String comparison: source contract and shared normal Text reads

Date: 2026-11-10. Base: `57bde328c4`. Scope: exact String/String only; mixed/null remain checked.
Task: `$S/codex-string-compare-2026-11-10.md`. Baseline artifacts: `$SIO`; new evidence: `$L`.
Public spellings follow that instruction; no new grammar, descriptor, schema version or StringRead family.

## Rules fixed before implementation

C1. The issuer explicitly seals all six exact String/String comparisons with TrivialBool, NoSourceFault and NonSuspending; ordering, observability, borrowing and ownership remain unchanged.
C2. A single query on the original operation loan checks the exact source domain, comparison family, Bool result and both axes; frame selection and physical projection consume its answer.
C3. Mixed Dynamic/String and null preserve their existing checked envelopes and do not enter the new read path, even if runtime values happen to be Text.
C4. An exact comparison that cannot supply valid Text operands refuses by a named existing boundary; it never retries or falls back to checked Compare.
C5. The existing private read plan retains up to two already evaluated operands and the resolved read operation, including comparison op; its internal operation discriminant is storage, not a second source classifier.
C6. Length, IndexOf and exact comparison use one normal Text-read writer and one expected-instruction projection for source correspondence; no purpose-specific producer is added.
C7. Left then right are evaluated exactly once in source order, retaining original loans, Copy/formal identity and use positions.
C8. Independent validation shares Text domain, availability, dominance and current-liveness checks for normal rows and Invoke, preserving duplicate operand occurrences.
C9. A common result-kind/origin walk distinguishes I64 and Bool through Copy and every PHI incoming; Bool cannot masquerade as I64 or vice versa when metadata is removed.
C10. StringCompare is a nonterminator READ with dst/op/left/right and Bool result, with no frame/site/Normal/Fault edges and no acquisition, transfer or End.
C11. The v2 row has exactly op,dst,comparison,left,right, and comparison is one of eq/ne/lt/le/gt/ge; operation kind:string_compare remains a distinct checked transport.
C12. The exact runtime leaf returns only 0 with a Bool word or 2 with output unchanged; malformed descriptors/leases/op/output never become source Faults.
C13. Exact and checked comparison share one scalar-sequence comparison and the existing Text borrow helper, with lease then registry locked once each and result publication after guard release.
C14. Checked comparison classifies and validates both sides before choosing equality or TypeError, including the Text side when the other side is null/other; no stale lease is hidden by an early unequal result.
C15. The helper may accept absent Text positions for checked null/other; it does not invent empty Text values, consume ownership or invoke carrier access/publish/drop while locks are held.
C16. Opaque carriers retain their existing snapshot boundary if no lease/token view exists; the snapshot is taken outside Text locks and is not cloned again for comparison.
C17. C uses shared row validation/operand access/prologue slots/edge forwarding, chooses the export by row op rather than arity, and preserves Bool as the existing i64 0/1 slot.
C18. Every executable Normal and surviving operand/call/End Fault path preserves acquisition/transfer/End responsibility and order; static End counts are reported per input, not used as an equality gate.
C19. Source acceptance/refusal text and existing EXE output/exit remain fixed; incidental newly executable inputs and all changed MIR are listed, and no heavier MIR is accepted.
C20. Functionality and cleanup use separate normalized production/test counts; cleanup includes its shared destinations and closes net negative without movement/formatting credit.

## Source to executable routes

| Responsibility | Existing owner and connection |
|---|---|
| Source operand classes | `resolved_semantics/home_expression_operations.rs:30-52`; Bool qualification `home_bool_condition.rs:350-480` |
| Exact/mixed/null issuer | `dynamic_operator_contract/issuer.rs:82-99,395-420`; original loan `home_expression_operator_loan.rs:33-39` |
| Shared exact-source query | `home_expression_comparison_nodes.rs:13`; existing eligibility `home_expression_evaluation.rs:213`, frame requirement `:47` |
| Sole checked producer | `ordinary_new_local_commit/root_call_entry/scalar_condition_operation.rs:6,99,126` |
| Shared bindings/source checks | same file `:179`; `expression_source_values.rs:196`, `expression_effect_order.rs:101`, `expression_projection.rs:284` |
| Sole scalar writer | `builder/ordinary_new_admission/selected/expression_value/invoke.rs:75` |
| Existing normal read plan/writer | `root_call_entry/expression_source_values.rs:399`; selected `expression_value.rs:523-539` |
| if and loop condition | raw Bool transport -> selected `bool_condition.rs:179-212`; loop frames `source_loop_frames.rs:472,505` |
| local/return/print/argument/rebind | selected purpose adapters -> `expression_value.rs:613-619` -> the same scalar writer |
| Conditional physical proof | `bool_condition_projection.rs:56-67`, `bool_condition_source.rs:443-484,543-556`; extend existing inline-interval responsibility, not a new context lane |
| Independent Text/result checks | `verification/invoke_string_read.rs:64-196,261`, lifetime/source checks and `invoke_call_edge.rs:88` |
| JSON and C | existing physical-v2 normal row, shared Text read admission/live validation and `lv4_string_read_alloca/emit` |
| Runtime | `fault_checked_string.rs:177-272` -> shared lease borrow -> exact registry identity; carrier snapshot is separate |

Loop Text Compare is a portable logical node, not another physical producer: `source_loop_bool.rs:200-220` -> Bool region -> common condition lowering.
Its conservative `bool_region::may_fault` / `schema_v2.rs:272` remains: making it nonfaulting by opcode would change child-loop admission (`source_loop_child.rs:29-34`).
The existing conditional proof assumes Invoke/Normal delimiters; normal comparisons must use the existing inline-range validation without dropping child Fault/cleanup evidence.
Map-presence callers of `text_comparison_producers_v1` still require their real Invoke/Normal pair.

## Suspension and Fault readers before the change

The read-only repository-wide audit found no production consumer of these DynamicOperator axes; production references are the model, issuer and reexports.
Private fields are read only by `model.rs:173,189` accessors. Source loans retain issuer envelope identity rather than reconstructing fields.

| Reader | Current responsibility | Effect of exact String refinement |
|---|---|---|
| `dynamic_operator_contract/tests.rs:35,48` | Dynamic Add | none |
| same `:82` | mixed Less Fault | none |
| same `:100,108` | String/Dynamic suspension/Fault | none |
| same `:141,149` | Dynamic/null | none |
| same `:268,277` | Integer arithmetic | none |
| same `:332,348` | common String/Integer/Bool comparison test | String expectation only changes; others remain as-is |
| `dynamic_invocation_contract/tests.rs:101` | a different envelope type | unrelated |

Stage A changes contract/docs/tests only, before connecting the new frame/plan query, and must leave the 1,788 MIR/refusal results identical.
After connection, the new source query is the intentional production axis reader. NoSourceFault is not Pure, speculative motion, permission to cross End, or an alias guarantee.

## Evidence by rule

| Shape/boundary | Rules |
|---|---|
| Six comparisons in if/loop/local/return/print | C1-7, C17-19; unsupported source forms stay refused |
| Unicode scalar order, prefix, empty, no normalization | C12-14; shared comparator and real EXE probes |
| Static/Owned/Borrowed/TextOperand and repeated same pair | C7-9, C13-16; both occurrences validated |
| metadata-free Bool Copy/PHI/Branch and wrong kind | C8-11; independent result-kind walk |
| mixed/null/invalid side/out unchanged | C3-4, C12-16; existing mixed matrix retained |
| concatenated operands and surviving Fault prefixes | C7, C18; shared evaluator and cleanup owner |
| strict normal op vs checked kind and invalid predicates | C10-12, C17 |

## Explicitly deferred decisions

Integer equality still uses the existing MaySuspend/TypeError envelope although its direct MIR route is frame-free; reader impact and a proposed contract correction are report-only.
Checked null/null and other/other are outside the issuer's mixed String domain; their current Normal/TypeError behavior is preserved, with InvalidContract only a report proposal.
The opaque carrier snapshot cannot borrow a host lease it does not contain; retaining that bounded copy is an explicit exception, not fallback.

## Opus の確認（2026-11-10）

Codex の `8377d69d20`〜`61bdc5f6b7`（8 commit）を、main `57bde328c4` から監査して fast-forward した。

- 約束: `DynamicOperatorFaultV1::NoSourceFault` を足し、`string_comparison` だけが `sealed_with_fault` で NoSourceFault と NonSuspending を明示する。`sealed()` の初期値と、整数・Bool・null の envelope は変えていない。
- 選ぶ所: `frame_free_text_comparison_v1`（`home_expression_comparison_nodes.rs`）1 つ。元の loan と source が同じ、左右とも String の class、envelope の domain・family・TrivialBool・NoSourceFault・NonSuspending・lifecycle 無し、operand の移転無し、を全部確かめる。class と domain の片方だけが String の時は、checked に戻らず `bool-condition/text-comparison-source` で止まる。枠の判定（`home_expression_evaluation.rs`）も同じ問い合わせを使う。
- SSA: `materialize.rs` と `edge_rematerialization.rs` は変わっていない（新しい行を既存の `Compare` の複製の規則に入れていない）。増えたのは id の付け替え（`joinir_id_remapper*.rs`、`value_uses.rs`）と名前の表だけ。
- 独立検証: Bool の結果の型を、metadata ではなく実際の定義から Copy と PHI をたどって決める（`has_text_read_scalar_origin`）。Bool の結果を And/Or 以外の算術に使う所や、型の食い違う PHI・Return は拒否する。
- C: export を operand の数ではなく `op` の名前で選ぶ（`lv4_string_read_emit`）。知らない op は `%invalid` へ飛ぶ。JSON の鍵は `op,dst,comparison,left,right` の 5 つで、`comparison` の値も検査する。
- runtime: `compare_text_v1` は `with_text_operands` で 2 つの Text を借りて、複製せずに比べる。checked の比較も同じ helper と比べる関数（`compare_text_value`）を使い、Text の複製をやめた。opaque carrier だけは既存の snapshot の複製が残る（carrier は host の lease を持たないため）。
- 行数（`fmtcount.py` の `src/` だけ）: production +558 / test +47。inline test を test に分けると、`src/` の production +378 / test +227 で、Codex の数（`src/` の合計 605）と合う。全体は機能 +444 / +382、整理 −11 / +57。変更したファイルは生の行数で最大 795 行。
- 未見の 21 本（`/mnt/workdisk/claude-scratch/scmp-audit`、基準は `57bde328c4` の CLI と、Codex が build の前に写した FFI と runtime）: 出力と拒否は 21 本とも同じ。MIR が変わった 11 本は全部軽くなって `string_compare` を使い、答えも正しい（substring の結果、`&&` と `||`、loop の中、局所の Bool、比較の結果どうしの `==`、同じ owned を両側、else if の連鎖、日本語の 5 つの比較を `&&` で、3 つの `if`、2 つの substring どうし）。
- 残した事:
  - 比較の結果どうしの順序比較 `(s < t) < (t < s)` と `(s < t) + 1` は、前と同じく拒否のまま。ただし理由の名前は `TerminalNotCovered` / `ReturnValueNotCovered` で、「Bool の順序比較」としての名前ではない。
  - String の field との比較、`Cmp.pick(a, b)` の formal、String を返す static method、`ArrayBox.get` との mixed、`s == null`、`loop(s != "aaa")` は前と同じ拒否のまま。`not (...)` は前と同じく canonical の道に乗らない。
  - 速さ: 200 万回の比較の 1 本で、前 0.305 秒、借用だけ 0.262 秒、後 0.200 秒（3 回の中央値）。
  - 整数の約束は変えていない。ユーザー決定（[no-fault-ops](no-fault-ops-2026-11-10.md) の最後）により、次の比較の正規化の回で、exact な入力と checked の見方を分けて直す。
