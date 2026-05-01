# JIT依存スモークテスト移行計画

Historical note:

- `tools/cross_backend_smoke.sh` and `tools/async_smokes.sh` were archived in
  `tools/archive/manual-smokes/` during `phase-30x / 30xG1`.
- `apps_tri_backend_smoke.sh` now also lives under
  `tools/archive/manual-smokes/`.
- This file is kept as a historical migration note, not as a current
  engineering entrypoint.

## 📊 現状分析（2025-09-24）

### JIT依存テスト数
- **アクティブ**: 17個（要対応）
- **アーカイブ済み**: 5個（対応完了）
- **合計**: 22個

## 🔧 対応方針

### 1. 即座にアーカイブ（JIT専用）
```bash
tools/archive/manual-smokes/aot_counter_smoke.sh
tools/build_aot.sh
tools/archive/manual-tools/build_python_aot.sh
```

### 2. ビルド行のみコメントアウト（VM/LLVM部分は有効）
```bash
tools/smoke_plugins.sh
tools/archive/manual-smokes/modules_smoke.sh
tools/archive/manual-smokes/apps_tri_backend_smoke.sh
```

### 3. 重要テスト（修正して維持）
```bash
# Phase 15セルフホスティング関連
tools/ny_roundtrip_smoke.sh
tools/ny_parser_bridge_smoke.sh
tools/bootstrap_selfhost_smoke.sh
tools/selfhost/selfhost_vm_smoke.sh
tools/dev_selfhost_loop.sh

# using system関連（codex実装中）
tools/using_e2e_smoke.sh
tools/using_resolve_smoke.sh
tools/using_strict_path_fail_smoke.sh
tools/using_unresolved_smoke.sh
```

## 📋 作業手順

### Phase 1: 即座の対応
1. ✅ mir15_smoke.sh → archive/
2. ✅ phase24_comprehensive_smoke.sh修正
3. ⏳ `build_aot.sh` remains active; `aot_counter_smoke.sh` and `build_python_aot.sh` are archived

### Phase 2: ビルド修正（コメントアウト）
- [ ] 3個のスモークでcranelift-jitビルドをコメントアウト
- [ ] VM/LLVMビルドのみ残す

### Phase 3: v2統合
- [ ] 重要テストをv2/profiles/に段階的移行
- [ ] 旧tools/直下を徐々に削減

## 🎯 目標
- **短期**: JITビルドエラーを回避
- **中期**: v2構造への統合
- **長期**: tools/直下のスモーク数を10個以下に
