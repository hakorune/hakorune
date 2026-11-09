# 正規化と共通化の設計相談（2026-11-09）

メンテナの問い: MIR builder は、正規化と共通化で本当に綺麗になるか。
- ChatGPT Pro に、commit `12cd9fc272` を読んで答えてもらった。
- 別の目として、Gemini 4 Argon にも読んでもらった。読むだけの plan モードで、同じ commit の写しを使った。
- Opus(Claude)が、両方の主な根拠をコードで確かめた。

背景は [MIR の書き手の整理](mir-writer-2026-11-09.md) の記録にある。
- 命令の literal を包む共通化は、効果がなかった。仕上げの後で、整形後 production +54 行だった。
- basic-expr の残りの壁は、2 つの証拠の世界にあった。
  - String の length の軽い形を、lifecycle が `call-not-birth` で拒否する。
  - Map の契約表が `Unprojected` である。

## 2 人とも同じ意見の所

- 揃えるのは命令の組み立て方ではない。元の source の証拠と、操作の法則と、Fault・所有の約束を揃える。
- 証拠のつなぎ方:
  - まず、証明済みの軽い NoSourceFault 操作を lifecycle に通す。
  - その後、意味を縮めてから選び直す古い raw の経路を退役させる。
  - 「常に checked の Invoke にする」は採らない。軽い形を保つ要件に反するからである。
- `PureRead` は NoFault の証明ではない(`src/mir/core_method_result_kind.rs:26`)。raw opcode、effect、receiver の名前から「Fault しない」を作らない。
- String の length の意味は、仕様どおり code point の数にする。
- 正規化の回と String の length の回を、basic-expr の残りより前に置く。
- Map は、`.hako` の契約(`lang/src/runtime/meta/core_method_contract_box.hako:202`)から意味を投影する仕事が先にある。既存の限定された証拠 `MapReadFactV1`(`src/mir/normal_callable_semantic_package/map_read_fact.rs:48`)を使う。

## Gemini が Pro と違った所(Opus が確かめて、採った)

1. **新しい正規形の型は作らない。**
   - Pro は、正規形を `FunctionControlV1` / `CanonicalCfgSessionV1` に置く案だった。しかし、これは命令を直接書く物理の CFG の session である(`src/mir/builder/resolved_lowering/canonical_ssa/function_control.rs:17`)。
   - `OriginalScalarAppendV1` は、命令を出した後の対応の receipt である(`src/mir/normal_callable_semantic_package/ordinary_new_scalar_operations.rs:23`)。
   - 土台には、今ある `CheckedExpressionEvaluationV1`、`CheckedExpressionOperationLoanV1`、`CheckedExpressionCoreSourceV1`、`LocalFieldReadRequestV1` を使う。そこへ「Fault の枠が要るか」を元の証拠から答える問い合わせを足す。
2. **最初の回は、2 つの専用一式を一緒に共通の評価器へ合流させる。** 対象は `TerminalI64AddReturnV1` と `TerminalI64FieldReturnV1` である。
   - 片方だけだと、古い `field_reads` の台帳と `TerminalFieldConsultV1` が残る。
   - Pro が見落としていた壁は次の 3 つ。どれも Opus が確かめた。
     - 共通の評価器は、出口で必ず block 2 つと Jump を作る(`src/mir/builder/ordinary_new_admission/selected/expression_value/cleanup.rs:361`)。
     - 対応の検査が `FaultFrameEnter` を必須にしている(`src/mir/normal_callable_semantic_package/ordinary_new_local_commit/root_call_entry/expression_terminal_cleanup.rs:121,166`)。
     - 借りた引数の field の読みは、式の一番上でしか許していない(`src/mir/normal_callable_semantic_package/ordinary_new_coseal_issue_source_field_batch.rs:98`)。
   - Gemini の見積もりでは、整形後で約 750 行の純減になる。まだ測っていない。
3. **String の length は、専用の軽い操作を 1 つ足す。**
   - 既存の `MirCall` を運び役に使い回す案は採らない。`MirCall` は Text operand の種類(`src/mir/instruction/invoke_string.rs:7`)を表せないので、後で取り直す処理が要るからである。
   - 前例として、`ArrayRead` の Count がある。lifecycle の JSON では `array_length` として出る(`src/mir/compiler/normal_default_pipeline/published_backend_view/physical_program_json.rs:383`)。
   - 要る物:
     - C 側の行の検査
     - 独立検証での block の中の生存の検査(`src/mir/verification/invoke_string_lifetime.rs:550` の loop は、今は `StringResidenceInstall` しか見ない)
     - Fault の枠を取らない code point の数の runtime の関数
   - 公開の形の追加なので、その回の調査の後で、メンテナがもう一度決める。
4. **物差しを足す。**
   - raw の dispatch にある `emit_terminal_*` の専用の分岐の数
   - 台帳にある文脈専用の欄の数
   - 700 行・800 行を超える file の数

## Opus が確かめた事

- String の length の意味は、経路によって違う。
  - 古い raw の経路は `nyash.string.len_fast_h` を呼び、`text.len()`(byte の数)を返す(`crates/nyash_kernel/src/exports/string_helpers.rs:96`)。`docs/reference/language/strings.md:15` にも、legacy の既定は byte とある。
  - checked の経路は code point の数を返す(`crates/nyash_kernel/src/exports/fault_checked_string.rs:234`)。この関数は Fault を書かない。
- 試した結果:
  - `"あい"` の length は、checked の Invoke で 2 だった。
  - raw の length の呼び出しを使う入力は、EXE の手前で全部拒否された。forms の r07・r09 と、Opus の新しい入力で確かめた。
  - なので、軽い形を code point の意味で作っても、今 EXE まで通る物の出力は変わらない(測った範囲で)。
- RULES 第 8 節: 整理の series は純減で閉じ、純増のまま次の機能追加へ進まない(`docs/development/RULES.md:97–98`)。MIR の書き手の回は +54 で閉じたので、正規化の 1 回目で純減させる。

## ユーザー決定（2026-11-09）

次の順番にする。

1. **正規化の 1 回目。**
   - Fault が起きないと元の証拠で示せる scalar の式は、Fault の枠なしで書く。
   - `TerminalI64AddReturnV1` と `TerminalI64FieldReturnV1` の専用一式を、共通の評価器へ合流させて消す。
   - 整形後 production の純減で閉じる。
2. **String の length の回。** 専用の軽い NoSourceFault 操作を足す。具体的な公開の形は、調査の後でメンテナが決める。
3. **basic-expr の残り。** 約束が閉じた操作から進める。
4. **Map の契約を投影する回。**
5. `edit` → 3b と旧経路の退役 → 徹底的な整理 → `.hako` の parser の parity → selfhost。
