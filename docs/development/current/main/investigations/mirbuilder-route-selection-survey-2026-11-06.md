---
Status: record — user decision 2026-11-06
Date: 2026-11-06
Scope: how the MIR builder chooses who lowers a site, and the agreed direction for restructuring it
Owner: [MirBuilder structure queue](mirbuilder-structure-refactor-queue-d0-2026-08-23.md), section「流れの作り直し」
---

# 経路の選び方の調査と、流れの作り直しの決定（2026-11-06）

ユーザーの見立て: site（source の式や文の場所）ごとに「認識する係」がたくさんいて取り合っている。型と所有の事実を一度だけ決め、lowering を構文の種類ごとに一つにしないと、MIR builder は小さくきれいにならない。読むだけの調査を2つ（意味の package 側、builder 側。Sonnet、実行なし）行い、主な数を Opus が確かめ、ChatGPT Pro に独立の意見を聞いた。行番号は main `d784753658`。

## 調査で分かったこと

- 構文の種類で分ける所は既に一か所ずつ（式 `src/mir/builder/raw_expression_dispatch/mod.rs:158`、文 `raw_expression_dispatch/statement_surface.rs:265`、top-level `src/mir/builder/program_root_work_plan.rs:541`）。関数呼び出しは一度決めて enum に入れてから実行する（`src/mir/builder/calls/function_call_preflight_route.rs:205`）。
- その下で「誰が受け持つか」の選び方が3種類混ざる。
  - 一度で決める: 関数呼び出し、loop の `route_loop`。
  - 先に当たった物が勝つ連鎖（2つ当たっても検出しない）: 文の連鎖 `src/mir/resolved_semantics/home_new_prefix_scan.rs:150-637`（どれも当たらないと `PrefixNotCovered` が残り、以降の文の判定も落ちる、`:589-591`）、Local 初期値の連鎖 `home_new_prefix_local_statement.rs:121-711`（call を書く試行が約12）、method 呼び出し `src/mir/builder/calls/member_route.rs:100-300`（静的 receiver で書き先の候補が9前後、最後は旧来の下ろし）、関数の入口 `normal_callable_semantic_loan_port/canonical_route.rs:45-65`。
  - 全部に聞いて2つ以上なら拒否: loop の5 family（`src/mir/loop_route_policy/family_selector.rs:147-169`）。
- loop の選択は3層（関数の入口の exact template、本体の loop 文の callable 用の分岐 `src/mir/builder/raw_loop_child_entry.rs:222-628`、`route_loop`）。LoopCond と LoopTrue は判定の語彙が2つ、lowering が2つ（`control_flow/plan` の `PlanLowerer`、`resolved_lowering/loop_recipe_physicalizer`）。`src/mir/builder/control_flow/plan/` は290 file、production 約5.7万行。
- 同じ事実を何か所でも決める: 位置の判定3つ以上（Assignment と文の位置の裸の call で答えが逆）、値の種類の語彙5つ、演算子の許可の一覧10か所以上、call の書き手の符号化5か所。route 記録は書き込み9か所の冪等な set で、誰が書いたかが残らない（`ordinary_new_local_commit/root_call_entry.rs:234-250`）。
- `compile_resolved*`（`src/mir/compiler/mod.rs`）は test からしか呼ばれない。それでしか届かない `resolved_{direct_accum,nested_predicate,generic_g0}_cutover.rs` は計514行。
- Opus の言い過ぎの訂正（Pro の指摘）: source 側にも分岐ごとの状態解析は既にある（`home_new_prefix_branch.rs:557`、`:691`）。loop には共通の CFG、SSA、`PhiTxn` が既にある。問題は、解析と選択が形ごとの観測に埋め込まれていること。

## ChatGPT Pro の意見（要点、引用は Opus が確かめた）

- 方向は賛成。ただし所有の解析は、完成した物理 MIR ではなく、**source の意図を保持した未公開の CFG** で行い、cleanup を含めてから最終の CFG と PHI を確定する。今の物理 MIR では `move` などの情報が失われる。今は seal した block への後続の辺を拒否する（`src/mir/builder/resolved_lowering/canonical_cfg/session.rs:337`、`:419`）ので、先に seal しない。
- 「元の名前は使えないが、呼んだ側にまだ終了の責任がある」状態が要る（`home_call_entry_transfer.rs:25` の Home の付け替えと `MovedOut`）。End 自身の Fault、最初の Fault の保持、残りの cleanup の継続、消費済みの責任の再試行禁止も守る。
- source 側で確定して保持する物: 型、callee、read/edit/take/from の契約、明示の move/share/take/copy の意図、Binding・Place・支持元・source site、スコープ、cleanup の登録順、評価順、構築・格納・call-entry の commit 契約。CFG の dataflow で求める物: 各地点で使える binding/Place、生きた所有の責任と移転待ちと移転済み、借用の生存区間と競合、各 Normal/Fault 辺の End、部分初期化・合流・loop の世代・要るなら drop flag。ADR-002 の「同一の semantic CFG で ownership と borrow を解析」（`docs/development/adr/adr-002-explicit-ownership-and-compiler-migration.md:72`）と一致し、文法の変更は要らない。
- 位置の判定3つを一つの boolean にしない。構造上の位置を一つに確定し、その位置を誰が処理するかは別に一度だけ決める。語彙を一つにすることより、同じ意味を再判定しないことを目標にする。
- 移行中は、同じ pipeline の中で効果を発行する前に担当を一意に決め、新旧の範囲を重ねず、選択後の失敗はエラーで再選択せず、移した範囲の旧判定を同じ series で消す（`docs/development/current/main/design/mirbuilder-inplace-replacement-policy-ssot.md:369` と同じ）。先着の連鎖を「全部に聞いて2つなら拒否」に単純に変えると優先順で通っていた入力を落としうるので、優先が要るならまず明示の選択規則にする。
- loop は、共通の `operations + inputs`（`src/mir/compiler/loop_node_physical_admission.rs:81`）、`CanonicalCfgSessionV1` と `PhiTxn`（`loop_recipe_physicalizer/loop_node_lowerer.rs:86`）へ資源の契約をつなぐ仕事。共通の physicalizer の値の class は I64/Bool/Unit だけ（`operation_type.rs:11`）で、`source_loop_driver` は call、所有 Array、Return、子 loop、辺の cleanup も扱う（`control_flow/plan/lowerer/source_loop_driver.rs:312`）ので、scalar 側に置き換えるだけでは機能を失う。`route_loop` が AST の構造一致で site を逆引きする所（`router.rs:163`）は、resolver の site を入口から渡す。
- 独立の verifier を End の生成器と兼ねない。規模の総量は予測しない（呼び出しと loop をまとめた時に実際に減った量を測ってから外挿する）。

## 決定（ユーザー、2026-11-06）

承認済みの順番は変えず、S2b から S4 を流れの作り直しの段階として使う。各段階の中身は[queue](mirbuilder-structure-refactor-queue-d0-2026-08-23.md)の節「流れの作り直し」。builder の調査は3bの後まで待たず、触る所ごとに「調べる・置き換える・消す」を一組で行う。4月の loop の流れの図（[migration order](../design/selfhost-parser-mirbuilder-migration-order-ssot.md)の「Loop owner split」）はこの決定で置き換える。
