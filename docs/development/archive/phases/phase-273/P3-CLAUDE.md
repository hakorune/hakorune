# Phase 273 P3 — “Plan Lowering SSOT Finalize” (Claude Code Instructions)

目的:
- Phase 273 P2 で導入した generalized CoreLoopPlan を SSOT として固定し、legacy fallback を撤去して収束を完成させる。

前提:
- Extractor は pure（builder 触り禁止）
- Pattern知識は Normalizer に閉じる
- Lowerer は CorePlan のみを処理（pattern-agnostic）
- terminator SSOT は `Frag → emit_frag()` のみ

## Scope

P3 でやる:
1. Pattern6 を generalized CoreLoopPlan（`frag/block_effects/phis/final_values`）へ移行
2. `lower_loop_legacy()` を撤去し、generalized 経路を SSOT 化（Fail-Fast）
3. CoreLoopPlan の `Option<...>` を必須化して “揺れ” を構造で消す（可能なら）

P3 でやらない:
- Pattern8/その他 pattern の Plan 化（別フェーズ）
- CorePlan の vocabulary 増殖（variant追加禁止）

## Tasks

### Task 1: Pattern6 を generalized CoreLoopPlan へ移行

対象:
- `src/mir/builder/control_flow/plan/normalizer.rs`

やること:
- ScanWithInit の normalize で以下を構築する:
  - `block_effects`: header/body/step の効果（ValueId は normalizer で生成・型登録）
  - `phis`: header の loop-carrier PHI
  - `frag`: header/body の BranchStub + wires（step→header, found→Return(i), after→Return(-1) など現行仕様に合わせる）
  - `final_values`: `i` の最終値（after で使うならその ValueId）
- legacy field（`header_effects/body/step_effects/carriers/cond_*` など）を空/未使用にするか、最終的に削除する。

注意:
- ScanWithInit は “return” を含むので、Frag wires に Return を含める（emit_frag SSOT）。
- `ensure_block_exists` が必要なブロック（after/found 等）は Lowerer で担保する。

### Task 2: Lowerer の legacy fallback を撤去

対象:
- `src/mir/builder/control_flow/plan/lowerer.rs`

やること:
- `lower_loop_legacy()` を削除
- `lower_loop()` は generalized フィールドを必須として扱い、欠落時は Err（Fail-Fast）
  - 例: `block_effects/phis/frag/final_values` が None なら `[lowerer] missing generalized loop fields` で Err

狙い:
- Lowerer から `emit_scan_with_init_edgecfg()` 等の pattern 参照を完全に消す。

### Task 3: CoreLoopPlan の “必須化”（可能な範囲で）

対象:
- `src/mir/builder/control_flow/plan/mod.rs`

やること:
- `block_effects/phis/frag/final_values` を `Option` から非Optionへ変更（できるなら）
- legacy fields を削除（この時点で Pattern6/7 が generalized を使っていることが前提）

### Task 4: Verifier の拡張（generalized 専用の不変条件）

対象:
- `src/mir/builder/control_flow/plan/verifier.rs`

例:
- `phis` が空でないこと（carrierがある場合）
- `frag.entry` が header_bb に一致すること（loopのentry SSOT）
- `block_effects` に header/body/step が含まれること（最低限）

### Task 5: 回帰テスト

VM:
- `bash tools/smokes/v2/profiles/integration/apps/archive/phase254_p0_index_of_vm.sh`
- `bash tools/smokes/v2/profiles/integration/apps/archive/phase256_p0_split_vm.sh`
- `bash tools/smokes/v2/profiles/integration/apps/archive/phase258_p0_index_of_string_vm.sh`

LLVM（必ず harness で、mock禁止）:
- `cargo build --release --features llvm`
- `bash tools/smokes/v2/profiles/integration/apps/archive/phase256_p0_split_llvm_exe.sh`
- `bash tools/smokes/v2/profiles/integration/apps/archive/phase258_p0_index_of_string_llvm_exe.sh`

注意:
- `NYASH_LLVM_USE_HARNESS=1` で `--features llvm` が無い場合は fail-fast する（mock禁止）。

## Acceptance Criteria

- Lowerer から `emit_scan_with_init_edgecfg()` 等の pattern 固有参照が消えている
- Pattern6/7 が generalized CoreLoopPlan を使用している
- legacy fallback が撤去され、欠落時は Err（Fail-Fast）
- 上記 smokes がすべて PASS（VM/LLVM）

