# 🌐 Nyash WASM ガイド

Nyash WebAssembly（WASM）実行に関する包括的ガイド

## ✅ 現行の運用入口（G2固定）

- build: `bash projects/nyash-wasm/build.sh`
- min1 gate: `bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_g2_min1_bridge_build_vm.sh`
- min2 gate: `bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_g2_browser_run_vm.sh`
- 日常まとめ実行: `tools/checks/dev_gate.sh wasm-demo-g2`
- G3最小実行: `tools/checks/dev_gate.sh wasm-demo-g3-core`
- G3フル実行: `tools/checks/dev_gate.sh wasm-demo-g3-full`（`wasm-demo-g3` は互換alias）
- SSOT:
  - `docs/development/current/main/phases/phase-29cc/29cc-134-wsm-g2-min1-bridge-run-loop-lock-ssot.md`
  - `docs/development/current/main/phases/phase-29cc/29cc-135-wsm-g2-min2-headless-run-lock-ssot.md`
  - `docs/development/current/main/design/wasm-hako-only-output-roadmap-ssot.md`

## 📖 ドキュメント一覧

### 基本ガイド
- **[Rust依存性分析](rust-dependency-analysis.md)** - 実行時Rust依存性の詳細分析
- **[Phase比較](phase-comparison.md)** - 9.77手動実装 vs 9.8+FFI基盤の比較
- **[配布ガイド](deployment-guide.md)** - WASM配布・実行方法

### 技術仕様
- **[FFI/BIDチュートリアル](ffi-bid-tutorial.md)** - 外部API統合方法
- **[メモリ管理](memory-management.md)** - WASM メモリレイアウト・最適化

### 現在のWASMデモタスク
- **[WSM G2 ブラウザデモタスク](../../development/current/main/phases/phase-29cc/29cc-133-wsm-g2-browser-demo-task-plan.md)** - `projects/nyash-wasm` を G2 (browser demo minimum) で再到達するための直近 3〜5 タスクを docs-first で固定し、run loop → headless smoke → dev guide の順で gate を固める。
- **[WSM-G2-min1 実装ロック](../../development/current/main/phases/phase-29cc/29cc-134-wsm-g2-min1-bridge-run-loop-lock-ssot.md)** - 独立 bridge crate で browser run loop 最小構成（ConsoleBox 5メソッド）を固定した受け入れ記録。
- **[WSM-G2-min2 実装ロック](../../development/current/main/phases/phase-29cc/29cc-135-wsm-g2-min2-headless-run-lock-ssot.md)** - headless chromium autorun smoke と `dev_gate.sh wasm-demo-g2` 追加による自動検証ロック。
- **[WSM-G2-min3 実装ロック](../../development/current/main/phases/phase-29cc/29cc-136-wsm-g2-min3-guide-alignment-lock-ssot.md)** - guide/quickstart の運用入口一本化（現行導線と履歴資料の分離）を固定した記録。
- **[WSM-G3-min1 台帳ロック](../../development/current/main/phases/phase-29cc/29cc-137-wsm-g3-min1-gap-inventory-lock-ssot.md)** - canvas/enhanced demo で必要な API ギャップを優先順で固定した台帳。
- **[WSM-G3-min2 実装ロック](../../development/current/main/phases/phase-29cc/29cc-138-wsm-g3-min2-canvas-clear-lock-ssot.md)** - `env.canvas.clear` の contract/import/binding/gate を固定した記録。
- **[WSM-G3-min3 実装ロック](../../development/current/main/phases/phase-29cc/29cc-139-wsm-g3-min3-canvas-strokerect-lock-ssot.md)** - `env.canvas.strokeRect` の contract/import/binding/gate を固定した記録。
- **[WSM-G3-min4 実装ロック](../../development/current/main/phases/phase-29cc/29cc-140-wsm-g3-min4-canvas-beginpath-lock-ssot.md)** - `env.canvas.beginPath` の contract/import/binding/gate を固定した記録。
- **[WSM-G3-min5 実装ロック](../../development/current/main/phases/phase-29cc/29cc-141-wsm-g3-min5-canvas-arc-lock-ssot.md)** - `env.canvas.arc` の contract/import/binding/gate を固定した記録。
- **[WSM-G3-min6 実装ロック](../../development/current/main/phases/phase-29cc/29cc-142-wsm-g3-min6-canvas-fill-lock-ssot.md)** - `env.canvas.fill` の contract/import/binding/gate を固定した記録。
- **[WSM-G3-min7 実装ロック](../../development/current/main/phases/phase-29cc/29cc-143-wsm-g3-min7-canvas-stroke-lock-ssot.md)** - `env.canvas.stroke` の contract/import/binding/gate を固定した記録。
- **[WSM-G3-min8 実装ロック](../../development/current/main/phases/phase-29cc/29cc-144-wsm-g3-min8-canvas-setfillstyle-lock-ssot.md)** - `env.canvas.setFillStyle` の contract/import/binding/gate を固定した記録。

## 🚀 クイックスタート

> 注記: 下記の `--compile-wasm` / `--aot` は長期目標の一般経路。現時点で日常運用する browser demo は上の「現行の運用入口（G2固定）」を使うこと。

### WASM コンパイル
```bash
# 基本コンパイル
./target/release/hakorune --compile-wasm program.hako

# AOT コンパイル（配布用）
./target/release/hakorune --aot program.hako
```

### ブラウザー実行
```html
<!DOCTYPE html>
<html>
<body>
    <script>
        WebAssembly.instantiateStreaming(fetch('program.wasm'), importObject)
            .then(instance => instance.exports.main());
    </script>
</body>
</html>
```

## 🎯 実行方式選択

| 用途 | 方式 | コマンド |
|------|------|----------|
| **開発・テスト** | インタープリター | `nyash program.hako` |
| **高速実行** | VM | `nyash --backend vm program.hako` |
| **Web配布** | WASM | `nyash --compile-wasm program.hako` |
| **ネイティブ配布** | AOT | `nyash --aot program.hako` |

## 📊 性能比較

| バックエンド | 実行速度 | 配布サイズ | 依存関係 |
|-------------|----------|------------|----------|
| インタープリター | 1x | - | Rust |
| VM | 20.4x | - | Rust |
| **WASM** | **13.5x** | **小** | **なし** |
| AOT | 目標1000x+ | 小 | なし |

## 🔗 関連ドキュメント
- [言語ガイド](../LANGUAGE_GUIDE.md)
- [実行バックエンド](../execution-backends.md)
- [ビルドガイド](../build/README.md)

---
**最終更新**: 2025-08-15
