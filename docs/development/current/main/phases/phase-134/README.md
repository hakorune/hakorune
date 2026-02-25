# Phase 134: Plugin Best-Effort Loading & Core Box Guard

**Date**: 2025-12-15
**Status**: ✅ Done (P0 + P1 complete)
**Scope**: プラグイン1個の失敗で全停止する問題を根治 + core box必須チェック

---

## 背景

**問題 (Before)**:
- PluginLoaderV2 が dlopen 失敗1個で `load_all_plugins()` を `Err` にして全プラグインが `disabled` になる
- MapBox の undefined symbol エラー → StringBox/IntegerBox など core box も巻き添えで使えなくなる

---

## 修正内容

### P0: Best-Effort Plugin Loading

**修正**: `src/runtime/plugin_loader_v2/enabled/loader/library.rs`
- `load_all_plugins()` を best-effort 化（失敗を蓄積して継続）
- config.libraries / config.plugins を決定的順序（ソート）で走査
- load_plugin() 失敗時も即座に `?` で落ちず、failures カウントして継続
- 最後に loaded_count / failed_count をログ出力

**結果**: 1個のプラグイン失敗 → 他のプラグインはロード継続（全停止しない）

### P1: Core Box Strict Guard

**修正**: `src/runner/modes/common_util/plugin_guard.rs`
- `gather_core_required_providers()` を SSOT として定義（StringBox/IntegerBox/ArrayBox/ConsoleBox）
- `check_and_report()` を拡張: strict モードの時に **core box が missing なら exit(1)**（Fail-Fast）
- strict モードでも non-core providers (FileBox/MapBox) の欠落は警告のみで継続

**結果**: NYASH_VM_PLUGIN_STRICT=1 時に core box 必須チェック（substring などの基本機能保証）

---

## 検証

### P0: Best-Effort Loading
- ✅ Smoke: `tools/smokes/v2/profiles/integration/apps/phase134_plugin_best_effort_init.sh`
- ✅ Test: "plugins disabled (config=nyash.toml)" ログなし
- ✅ Test: --dump-mir 成功 (exit code 0)

### P1: Core Box Guard
- ✅ Smoke: `tools/smokes/v2/profiles/integration/apps/phase134_core_boxes_minimal.sh`
- ✅ Test: NYASH_VM_PLUGIN_STRICT=1 で core box 揃っていれば PASS
- ✅ Test: core box SSOT 定義（4 boxes）

### 退行チェック
- ✅ Phase 132: `phase132_exit_phi_parity.sh` 3/3 PASS

---

## 設計原則 (Box-First / SSOT)

- **Best-Effort**: failures を個別に蓄積（責務分離）、部分的に機能する状態 > 全体失敗
- **Deterministic Order**: config を sorted で走査（HashMap イテレーション非決定性回避）
- **SSOT**: core box 定義を `gather_core_required_providers()` に集約（1箇所管理）
- **Fail-Fast**: strict モードで core box missing → exit(1)（実行系検証を再開可能に）

---

## 参考

- **P0 Smoke**: `tools/smokes/v2/profiles/integration/apps/phase134_plugin_best_effort_init.sh`
- **P1 Smoke**: `tools/smokes/v2/profiles/integration/apps/phase134_core_boxes_minimal.sh`
- **SSOT**: `src/runner/modes/common_util/plugin_guard.rs` (gather_core_required_providers)
- **修正コミット**:
  - `ccd23423` feat(plugin_loader): Phase 134 P0 - Best-effort plugin loading
  - (P1 commit pending)
