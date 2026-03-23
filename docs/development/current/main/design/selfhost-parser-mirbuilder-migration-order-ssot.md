---
Status: SSOT
Scope: single-developer 運用で `.hako` mirbuilder / parser 移植順序を固定する
Related:
- docs/development/current/main/design/selfhost-bootstrap-route-ssot.md
- docs/development/current/main/design/selfhost-compiler-structure-ssot.md
- docs/development/current/main/design/hako-mirbuilder-migration-phase0-entry-contract-ssot.md
- docs/development/current/main/design/runtime-gc-policy-and-order-ssot.md
- docs/development/current/main/design/selfhost-language-v1-freeze-ssot.md
- docs/development/current/main/design/optimization-hints-contracts-intrinsic-ssot.md
- docs/development/current/main/design/rune-v0-contract-rollout-ssot.md
- docs/development/current/main/design/parser-extensions-param-implements-interface-generic-ssot.md
- docs/development/current/main/phases/phase-29bq/29bq-90-selfhost-checklist.md
- docs/development/current/main/phases/phase-29bq/29bq-91-mirbuilder-migration-progress-checklist.md
- docs/development/current/main/phases/phase-29bq/29bq-109-hako-mirbuilder-handler-extraction-backlog.md
- docs/development/current/main/phases/phase-29bq/29bq-113-hako-recipe-first-migration-lane.md
---

# Selfhost Parser/MirBuilder Migration Order (SSOT)

## Goal

`.hako` 移植を “順番迷子” にしない。  
特に、single-developer 運用で **mirbuilder 先行 / parser 後行** を守る。

## Non-negotiables

- selfhost は目的化しない（compiler-first）。
- 1ブロッカー = 1受理形 = fixture+gate = 1コミット。
- blocker が `none` の間は `.hako` mirbuilder を failure-driven で運用し、先回りの受理拡張/fixture追加を行わない。
- AST rewrite 禁止。analysis-only 観測で進める。
- silent fallback 禁止（失敗は fail-fast）。
- `.hako mirbuilder` 側の失敗タグは `[freeze:contract][hako_mirbuilder]` で統一。

## Entry signature migration contract (main args)

Canonical:
- static box entry は `main()` を正とする（`method main(args)` は legacy 互換）。

Staged migration order:
1. 実ファイル上の outer entry（stub/test/driver）で `args` 未使用箇所を `main()` へ先行移行する。
2. fixture payload string（`src = src + " ... method main(args) ... "`）は scanner coverage 契約があるため、先に置換しない。
3. payload string を触るときは、対象 fixture の期待値固定（PROBE→PROMOTE）と同コミットで行う。
4. `compiler_stageb.hako` / `compiler.hako` の legacy main 検出縮退は、payload inventory が `0` になってから実施する。

Args-kept exceptions (current SSOT):
- なし（`stageb_*` fixture の outer/payload entry は `main()` へ移行済み）。
- `lang/src/compiler/tests/stageb_min_sample.hako` / `lang/src/compiler/tests/stageb_mini_driver.hako` は `synthetic_args` 明示注入で `args.length()/get(i)` 経路の coverage を維持する。
- legacy entry 文字列（`"static method main"` / `"method main"`）の検出契約は `compiler_stageb.hako` 側 SSOT（`MainDetectionHelper.findLegacyMainBody`）で維持する。

Legacy detection literals decision (accepted, 2026-02-08):
- 保持理由: Stage-A fallback と legacy source 互換を fail-fast で受け止める検出契約であり、cleanup対象ではなく compatibility boundary として扱う。
- SSOT境界: literals の探索は `MainDetectionHelper.findLegacyMainBody` のみが担い、他層で重複実装しない。
- 撤去条件:
  1. inventory で legacy entry producer が 0（`rg -n "method\\s+main\\(args\\)|static method main" lang/src/compiler apps/tests`）。
  2. Stage-A fallback 依存が 0（`compiler.hako` 側が `MainDetectionHelper.findLegacyMainBody` を呼ばない設計へ移行）。
  3. 上記状態で `phase29bq_fast_gate_vm.sh --only bq` と selfhost identity smoke が緑。

Inventory command (SSOT):
- `rg -n "method\\s+main\\(args\\)|static method main" lang/src/compiler`
- `rg -n "method\\s+main\\(args\\)|\\bargs\\b" lang/src/compiler/tests/stageb_*.hako`

Legacy literal removal readiness (docs-first, 2026-02-08):
- 目的: legacy literals 撤去判断を「検索と軽量 smoke」の1セットで毎回同じ順序で確認する。
- helper:
  - `tools/selfhost/legacy_main_readiness.sh`
- 実行順序（最小）:
  1. producer inventory:
     - `rg -n "method\\s+main\\(args\\)|static method main" lang/src/compiler apps/tests`
  2. Stage-A consumer inventory:
     - `rg -n "findLegacyMainBody|tryLegacyPattern" lang/src/compiler/entry/compiler_stageb.hako lang/src/compiler/entry/compiler.hako`
  3. identity smoke:
     - `tools/selfhost_identity_check.sh --mode smoke --skip-build`
     - 互換診断が必要な場合のみ: `tools/selfhost_identity_check.sh --mode smoke --skip-build --cli-mode auto --allow-compat-route --bin-stage1 target/release/hakorune --bin-stage2 target/release/hakorune`
- 判定:
  - producer inventory が 0 で、identity smoke が PASS のときにだけ legacy literals 撤去タスクへ進む。
  - 上記を満たさない状態では literals を cleanup として扱わない（compatibility boundary 維持）。
  - pre-promote gate command:
    - `bash tools/selfhost/pre_promote_legacy_main_removal.sh`

Producer inventory decomposition (2026-02-08 snapshot, non-comment hits):
- code-side (2):
  - `lang/src/compiler/entry/compiler_stageb.hako` (`findPattern("static method main")` / `extractBodyFromPosition(... + 19)`)
- test-side (2):
  - `apps/tests/minimal_to_i64_void.hako` (`method main(args)`)
  - `apps/tests/emit_boxcall_length_canary_vm.hako` (`method main(args)`)

Removal order decision (accepted, docs-only):
1. tests-first: test-side producer 2件を `main()` へ移行し、fixture intent は維持する。
2. readiness 再計測: `bash tools/selfhost/legacy_main_readiness.sh ...` で `producer_count=2`（code-side のみ）を確認する。
3. compiler-literals second: `compiler_stageb.hako` / `compiler.hako` の legacy literals を同一責務コミットで撤去する。
4. pre-promote strict gate は 3) の候補差分上で実行し、`bash tools/selfhost/pre_promote_legacy_main_removal.sh ...` が `exit 0` を返すことを受理条件にする。

## Fixed order (must follow)

### 0) Baseline

1. `cargo check --bin hakorune`
2. `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`

### 1) MirBuilder Phase-0 (entry + I/O contract)

1. Program(JSON v0) -> MIR JSON のみを実装範囲に固定する。
2. pin smoke（phase0/1/2）を先に緑化する。
3. 受理形は増やさず、fail-fast 導線だけを確定する。

### 2) MirBuilder Phase-1/2 (vocab expansion)

1. `stmt/effect` -> `if` -> `loop` -> `join/phi` の順で 1形ずつ増やす。
2. 追加ごとに fixture + gate を同コミットで固定する。
3. full selfhost gate は節目だけ実行する（常時は回さない）。

### 3) Parser handoff (after mirbuilder)

1. parser は v1 freeze 範囲のみ対象にする（仕様拡張しない）。
2. parser の変更は “既存 Stage-B JSON v0 契約を壊さないこと” を最優先にする。
3. parser 系 blocker は selfhost subset で 1本ずつ pin して PROMOTE する。

### 3.5) Post-parser pivot: `.hako` mirbuilder handler extraction

1. parser handoff Tier-18 を完了したら、次は `.hako` mirbuilder handler 抽出レーンへ切り替える。
2. 実行順序は `29bq-109` の M0->M1->M2->M3->M4 を固定で守る（1コミット=1 handler）。
3. parser handoff commit と mirbuilder extraction commit を混ぜない。
4. 各コミットで quick verify（`tools/hakorune_emit_mir.sh` + `phase29bq_fast_gate_vm.sh --only bq`）を必須にする。

### 3.6) Recipe-first pivot: `.hako` mirbuilder core migration

1. `29bq-113` の R0->R6 を順序固定で実行する（1コミット=1項目）。
2. Tier-20+ fixture 追加は failure-driven のみ（green維持中の先回り追加は禁止）。
3. no-try/no-throw 方針を維持し、Facts->Recipe->Verifier->Lower の導線を先に固定する。

### 4) Stage1/Stage2 identity (milestone)

1. `tools/selfhost_identity_check.sh --mode full` の一致を確認する。
2. 不一致ならその stage で停止して、次工程に進まない。

## Daily / milestone commands

### Daily (quick)

- `cargo check --bin hakorune`
- `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`
- `tools/selfhost/run.sh --gate --planner-required 1 --max-cases 5`

### Probe (single case)

- `tools/selfhost/run.sh --gate --planner-required 1 --filter <case_substring> --max-cases 1`

### Milestone (heavy)

- `tools/selfhost/run.sh --gate --planner-required 1 --timeout-secs 120`
- `bash tools/smokes/v2/profiles/integration/joinir/phase29bs_fast_gate_vm.sh --full`

## Definition of done (migration order)

- Phase-0/1/2 pin smokes が緑。
- selfhost subset は blocker 群を coverage 済み（未PROMOTE 0）。
- full selfhost gate が緑。
- その後に parser の追加移植へ進む（順序逆転しない）。

## Parser Syntax Extension Contract（optimization annotations）

`@hint` / `@contract` / `@intrinsic_candidate` を言語文法として導入する場合の順序を固定する。

結論:

- 文法拡張を有効化する最終状態では **Rust parser / .hako parser の両方が必要**。
- ただし初手は parser 非依存（registry metadata のみ）で開始してよい。

固定順序:

1. docs-first（provisional）:
   - `optimization-hints-contracts-intrinsic-ssot.md`
   - `docs/reference/language/EBNF.md`
2. Rust parser 側で受理（flagged）し、Program(JSON v0) 属性出力を固定。
3. `.hako` parser 側で同形を受理し、Program(JSON v0) 形状 parity を固定。
4. parser parity gate が緑になってから、mirbuilder/lowering で注釈を本利用する。

最小 gate:

- `cargo test parser_opt_annotations -- --nocapture`
- `bash tools/smokes/v2/profiles/integration/parser/parser_opt_annotations_dual_route_noop.sh`

禁止:

- 片側 parser のみ受理した状態で既定ONにすること。
- parser 差分を runtime 側 workaround で吸収すること。

## Parser Syntax Extension Contract（Rune v0）

`@rune` を language grammar として有効化する場合の順序を固定する。

結論:

- final active grammar では **Rust parser / `.hako` parser の両方が必要**。
- first slice は contract-only で、backend consumer は `ny-llvmc` に限定する。
- `llvmlite` は compat/noop keep のまま読む。

固定順序:

1. docs-first（provisional）
   - `rune-and-stage2plus-final-shape-ssot.md`
   - `rune-v0-contract-rollout-ssot.md`
   - `docs/reference/language/EBNF.md`
   - `docs/reference/ir/ast-json-v0.md`
2. Rust parser 側で `NYASH_FEATURES=rune` を受理し、declaration metadata を保持する。
3. `.hako` parser 側で同じ Rune surface を受理し、同形 metadata を保持する。
4. AST / Program(JSON v0) parity gate を緑化する。
5. parity 後に only `ny-llvmc` consumer を有効化する。

禁止:

- `.hako` parser だけ、または Rust parser だけで Rune grammar を active にすること。
- parser parity 前に runtime/backend workaround を足して Rune 不在を吸収すること。
- `llvmlite` parity を v0 の unblock 条件にすること。
