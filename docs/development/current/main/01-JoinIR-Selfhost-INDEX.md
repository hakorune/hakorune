# JoinIR / Selfhost INDEX（読み始めガイド）

Status: Active  
Scope: JoinIR と Selfhost（Stage‑B/Stage‑1/Stage‑3）に関する「最初に読むべき現役ドキュメント」だけを集約した入口。

このファイルは、JoinIR と Selfhost ラインの主戦場をすばやく把握するためのインデックスだよ。  
歴史メモや詳細な Phase 文書に飛ぶ前に、まずここに載っている現役ドキュメントから辿っていくことを想定しているよ。

docs の置き場所（SSOT/Phase/調査ログの分離ルール）は、先にこれを読むと迷子になりにくいよ。

- `docs/development/current/main/DOCS_LAYOUT.md`


---

## 1. まず全体像だけ掴みたいとき

- JoinIR 全体像（SSOT）
  - `docs/development/current/main/joinir-architecture-overview.md`
- Selfhost / Stage‑B〜3 の代表フロー
  - `docs/development/current/main/selfhost_stage3_expected_flow.md`
- 「いまどこまで進んでいるか」の現状サマリ
  - `docs/development/current/main/10-Now.md`
    - 「JoinIR / Loop / If ライン」
    - 「JsonParser / Selfhost depth‑2 ライン」
- Phase 86–90（Loop frontends）の要約（1枚）
  - `docs/development/current/main/phase86-90-loop-frontends-summary.md`

---

## 2. JoinIR をこれから触る人向け

JoinIR の箱構造と責務、ループ/if の lowering パターンを把握したいときの読み順だよ。

1. JoinIR の基本設計（SSOT）
   - `docs/development/current/main/joinir-architecture-overview.md`
2. ループパターン空間とパターン番号の意味
   - `docs/development/current/main/loop_pattern_space.md`
3. Boundary / ExitLine / Carrier の具体パターン
   - `docs/development/current/main/joinir-boundary-builder-pattern.md`
4. JoinIR 設計地図（現役の地図）
   - `docs/development/current/main/design/joinir-design-map.md`
   - ControlTree / StepTree（構造SSOT）: `docs/development/current/main/design/control-tree.md`
5. Loop Canonicalizer（設計 SSOT）
   - `docs/development/current/main/design/loop-canonicalizer.md`
   - 実装（Phase 137-2）: `src/mir/loop_canonicalizer/mod.rs`
6. Phase 93: ConditionOnly Derived Slot（Trim / body-local）
   - `docs/development/current/main/phases/phase-93/README.md`
7. Phase 94: P5b “完全E2E”（escape skip / derived）
   - `docs/development/current/main/phases/phase-94/README.md`
8. Phase 95: MiniJsonLoader escape ループ（Phase 94 基盤の横展開）
   - `docs/development/current/main/phases/phase-95/README.md`
9. Phase 96: Trim policy 着手 + next_non_ws ループ
   - `docs/development/current/main/phases/phase-96/README.md`
10. Phase 97: MiniJsonLoader LLVM EXE parity（next_non_ws / escape）
    - `docs/development/current/main/phases/phase-97/README.md`
11. Phase 98: Plugin loader fail-fast + LLVM parity持続化
    - `docs/development/current/main/phases/phase-98/README.md`
12. Phase 100: Pinned Read‑Only Captures（設計メモ）
    - `docs/development/current/main/phases/phase-100/README.md`
13. Phase 102: real-app read_quoted loop regression（VM + LLVM EXE）
    - `docs/development/current/main/phases/phase-102/README.md`
14. Phase 103: if-only regression baseline（VM + LLVM EXE / plan）
    - `docs/development/current/main/phases/phase-103/README.md`
15. Phase 113: if-only partial assign parity（片側代入の保持 merge）
    - `docs/development/current/main/phases/phase-113/README.md`
16. Phase 114: if-only return+post parity（early return + post-if statements）
    - `docs/development/current/main/phases/phase-114/README.md`
17. Phase 115: if-only call result merge parity（関数呼び出し結果 merge）
    - `docs/development/current/main/phases/phase-115/README.md`
18. Phase 116: if-only keep+call merge parity（片側元値保持、片側 call merge）
    - `docs/development/current/main/phases/phase-116/README.md`
19. Phase 117: if-only nested-if + call merge parity（ネストif + call merge）
    - `docs/development/current/main/phases/phase-117/README.md`
20. Phase 118: loop + if-else merge parity（loop + if-else 変数更新 merge / carrier PHI contract）
    - `docs/development/current/main/phases/phase-118/README.md`
21. Phase 119: StepTree cond SSOT（AST handle）
    - `docs/development/current/main/design/control-tree.md`
22. Phase 120: StepTree facts/contract SSOT（facts only → contract）
    - `docs/development/current/main/design/control-tree.md`
23. Phase 121: StepTree→Normalized Shadow Lowering（if-only, dev-only）
    - `docs/development/current/main/phases/phase-121/README.md`
24. Phase 122: StepTree→Normalized 実生成（if-only, dev-only）
    - `docs/development/current/main/phases/phase-122/README.md`
25. Phase 123: Normalized semantics（Return literal + If minimal compare, dev-only）
    - `docs/development/current/main/phases/phase-123/README.md`
26. Phase 124: Reads facts + Return(Variable from env)（dev-only）
    - `docs/development/current/main/phases/phase-124/README.md`
27. Phase 125: Reads-only inputs → Normalized env（dev-only）
    - `docs/development/current/main/phases/phase-125/README.md`
28. Phase 126: available_inputs SSOT wiring（dev-only）
    - `docs/development/current/main/phases/phase-126/README.md`
29. Phase 127: unknown-read strict Fail-Fast（dev-only）
    - `docs/development/current/main/phases/phase-127/README.md`
30. Phase 128: if-only Normalized partial-assign keep/merge（dev-only）
    - `docs/development/current/main/phases/phase-128/README.md`
31. Phase 129: Materialize join_k continuation + LLVM parity（P1-C done）
    - `docs/development/current/main/phases/phase-129/README.md`
32. Phase 130: if-only Normalized small expr/assign（DONE）
    - `docs/development/current/main/phases/phase-130/README.md`
33. Phase 104: loop(true) break-only digits（VM + LLVM EXE）
    - `docs/development/current/main/phases/phase-104/README.md`
34. Phase 107: json_cur find_balanced_* depth scan（VM + LLVM EXE）
    - `docs/development/current/main/phases/phase-107/README.md`
35. Phase 108: Pattern2 policy router SSOT（入口の薄さを固定）
    - `docs/development/current/main/phases/phase-108/README.md`
36. Phase 109: error_tags hints SSOT（Fail-Fast + hint の語彙固定）
    - `docs/development/current/main/phases/phase-109/README.md`
37. MIR Builder（Context 分割の入口）
    - `src/mir/builder/README.md`
38. Scope/BindingId（shadowing・束縛同一性の段階移行）
   - `docs/development/current/main/phase73-scope-manager-design.md`
   - `docs/development/current/main/PHASE_74_SUMMARY.md`
   - `docs/development/current/main/PHASE_75_SUMMARY.md`
   - `docs/development/current/main/PHASE_77_EXECUTIVE_SUMMARY.md`
   - `docs/development/current/main/phase78-bindingid-promoted-carriers.md`
   - `docs/development/current/main/phase80-bindingid-p3p4-plan.md`（P3/P4 への配線計画）
   - `docs/development/current/main/phase81-pattern2-exitline-contract.md`（promoted carriers の ExitLine 契約検証）
39. Boxification feedback（Phase 78–85 の振り返りと Phase 86 推奨）
   - `docs/development/current/main/phase78-85-boxification-feedback.md`
40. Phase 86: Carrier Init Builder + Error Tags ✅
   - **Status**: COMPLETE (2025-12-13)
   - **Modules**:
     - `src/mir/builder/control_flow/joinir/merge/carrier_init_builder.rs` (+8 tests)
     - `src/mir/join_ir/lowering/error_tags.rs` (+5 tests)
   - **Achievements**: SSOT 確立（CarrierInit → ValueId 生成統一、エラータグ中央化、DebugOutputBox 完全移行）
   - **Impact**: 987/987 tests PASS, +13 unit tests, Single Responsibility validated
33. Phase 87: LLVM Exe Line SSOT ✅
   - **Status**: COMPLETE (2025-12-13)
   - **SSOT**: `tools/build_llvm.sh` - Single pipeline for .hako → executable
   - **Deliverables**:
     - Design doc: `phase87-selfhost-llvm-exe-line.md` (full troubleshooting + advanced usage)
     - Minimal fixture: `apps/tests/phase87_llvm_exe_min.hako` (exit code 42)
     - Integration smoke: `tools/smokes/v2/profiles/integration/apps/phase87_llvm_exe_min.sh` (SKIP if no LLVM)
   - **Policy**: No script duplication, integration smoke only (not quick), graceful SKIP
   - **Impact**: Standard procedure established, prerequisites documented
28. 代表的な Phase 文書（現役ラインとの接点だけ絞ったもの）
   - `docs/development/current/main/phase33-16-INDEX.md`
   - `docs/development/current/main/phase33-17-joinir-modularization-analysis.md`
   - `docs/development/current/main/phase183-selfhost-depth2-joinir-status.md`
29. Phase 86–90（Loop frontends）の要約（1枚）
   - `docs/development/current/main/phase86-90-loop-frontends-summary.md`

Phase 文書は歴史や検証ログも含むので、「JoinIR の現役設計を確認した上で、必要なときだけ掘る」という前提で読んでね。

---

## 3. Selfhost（Stage‑B / Stage‑1 / Stage‑3）を触る人向け

自己ホストコンパイラのフローや実行手順、Ny Executor ラインの計画を押さえたいときの読み順だよ。

1. Selfhost 全体フロー（Stage‑B / Stage‑1 / Stage‑3 と JSON v0）
   - `docs/development/current/main/selfhost_stage3_expected_flow.md`
2. 実行手順・クイックスタート
   - `docs/development/selfhosting/quickstart.md`
   - `docs/development/testing/selfhost_exe_stageb_quick_guide.md`
3. Ny Executor（Ny で MIR(JSON v0) を実行）のロードマップ
   - `docs/development/roadmap/selfhosting-ny-executor.md`
4. Selfhost を進める前に “compiler 側の表現力を先に上げる” 方針（SSOT）
   - `docs/development/current/main/design/compiler-expressivity-first-policy.md`
4. Stage‑3 / depth‑2 関連で「現役」として参照する Phase 文書
   - `docs/development/current/main/phase150_selfhost_stage3_depth1_baseline.md`
   - `docs/development/current/main/phase150_selfhost_stage3_depth1_results.md`
   - `docs/development/current/main/phase183-selfhost-depth2-joinir-status.md`
   - `docs/development/current/main/phase120_selfhost_stable_paths.md`

---

## 4. 迷ったときの読み分けガイド

- JoinIR の箱や契約で迷っているとき
  - → 2章の 1〜3 をこの順番で読む。
- Selfhost のビルド / 実行フローで迷っているとき
  - → 3章の 1〜3 をこの順番で読む。
- VM backend の Box 解決（ConsoleBox / plugin / builtin）で迷っているとき
  - → `docs/development/current/main/phase131-2-box-resolution-map.md`（経路図）
  - → `docs/development/current/main/phase131-2-summary.md`（要点）
- LLVM（Python llvmlite）lowering の不具合切り分けで迷っているとき
  - → `docs/development/current/main/phase131-3-llvm-lowering-inventory.md`（再現ケース表 + 根本原因候補）
  - → `docs/development/current/main/phase131-11-case-c-summary.md`（Case C: `loop(true)` + break/continue の本命計画）
  - → `docs/development/current/main/case-c-infinite-loop-analysis.md`（Case C: 詳細調査ログ）
  - → `docs/development/current/main/phase131-5-taglink-fix-summary.md`（TAG-LINK: symbol 名の修正ログ）
  - → `docs/development/current/main/phase131-6-ssa-dominance-diagnosis.md`（TAG-RUN の初期診断ログ・歴史）
  - → `docs/development/current/main/phase87-selfhost-llvm-exe-line.md`（実行パイプラインのSSOT）
- 「この Phase 文書は現役か？」で迷ったとき
  - → まず `docs/development/current/main/10-Now.md` と  
    `docs/development/current/main/30-Backlog.md` を確認し、そこで名前が挙がっている Phase 文書を優先して読んでね。
