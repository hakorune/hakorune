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
- **[WSM-G3-min9 実装ロック](../../development/current/main/phases/phase-29cc/29cc-145-wsm-g3-min9-canvas-setstrokestyle-lock-ssot.md)** - `env.canvas.setStrokeStyle` の contract/import/binding/gate を固定した記録。
- **[WSM-G3-min10 実装ロック](../../development/current/main/phases/phase-29cc/29cc-146-wsm-g3-min10-canvas-setlinewidth-lock-ssot.md)** - `env.canvas.setLineWidth` の contract/import/binding/gate を固定した記録。
- **[WSM-G3-min11 設計ロック](../../development/current/main/phases/phase-29cc/29cc-147-wsm-g3-min11-fillcircle-drawline-gap-lock-ssot.md)** - `fillCircle -> drawLine` の固定順を docs-first でロックした記録。
- **[WSM-G3-min12 実装ロック](../../development/current/main/phases/phase-29cc/29cc-148-wsm-g3-min12-canvas-fillcircle-lock-ssot.md)** - `env.canvas.fillCircle` の contract/import/binding/gate を固定した記録。
- **[WSM-G3-min13 実装ロック](../../development/current/main/phases/phase-29cc/29cc-149-wsm-g3-min13-canvas-drawline-lock-ssot.md)** - `env.canvas.drawLine` の contract/import/binding/gate を固定した記録。
- **[WSM-P1-min1 実装ロック](../../development/current/main/phases/phase-29cc/29cc-150-wsm-p1-min1-emit-wat-cli-lock-ssot.md)** - `--emit-wat` CLI 入口を固定した記録。
- **[WSM-P1-min2 実装ロック](../../development/current/main/phases/phase-29cc/29cc-151-wsm-p1-min2-wat-parity-lock-ssot.md)** - fixture単位の WAT parity（direct vs `--emit-wat`）を固定した記録。
- **[WSM-P2-min1 実装ロック](../../development/current/main/phases/phase-29cc/29cc-152-wsm-p2-min1-wat2wasm-bridge-lock-ssot.md)** - `wat2wasm` bridge（normal/boundary/error）を固定した記録。
- **[WSM-P3-min1 実装ロック](../../development/current/main/phases/phase-29cc/29cc-153-wsm-p3-min1-import-object-lock-ssot.md)** - JS import object 生成契約（supported list / fail-fast 文言）を固定した記録。
- **[WSM-P4-min1 設計ロック](../../development/current/main/phases/phase-29cc/29cc-154-wsm-p4-min1-binary-writer-doc-lock-ssot.md)** - wasm binary writer（section/LEB128）最小契約を docs-first で固定した記録。
- **[WSM-P4-min2 実装ロック](../../development/current/main/phases/phase-29cc/29cc-155-wsm-p4-min2-binary-writer-skeleton-lock-ssot.md)** - wasm binary writer skeleton（magic/version + section/LEB128 + main export）を unit/smoke で固定した記録。
- **[WSM-P4-min3 設計ロック](../../development/current/main/phases/phase-29cc/29cc-156-wsm-p4-min3-hako-writer-entry-parity-doc-lock-ssot.md)** - `.hako` writer 入口（最小 fixture）と bytes parity gate の契約を docs-first で固定した記録。
- **[WSM-P4-min4 実装ロック](../../development/current/main/phases/phase-29cc/29cc-157-wsm-p4-min4-hako-writer-const-parity-lock-ssot.md)** - const-return fixture 1件の binary-writer parity pilot を固定した記録。
- **[WSM-P4-min5 実装ロック](../../development/current/main/phases/phase-29cc/29cc-158-wsm-p4-min5-neg-const-parity-lock-ssot.md)** - `return -1` 形（signed LEB128 境界）の parity を固定した記録。
- **[WSM-P4-min6 実装ロック](../../development/current/main/phases/phase-29cc/29cc-159-wsm-p4-min6-shape-table-lock-ssot.md)** - pilot shape 判定を shape table（箱化）へ移し、table 経由 lock を固定した記録。
- **[WSM-P5-min1 設計ロック](../../development/current/main/phases/phase-29cc/29cc-160-wsm-p5-min1-default-cutover-doc-lock-ssot.md)** - default cutover（既定 route 切替）の境界・互換・gate を docs-first で固定した記録。
- **[WSM-P5-min2 実装ロック](../../development/current/main/phases/phase-29cc/29cc-161-wsm-p5-min2-route-policy-lock-ssot.md)** - `NYASH_WASM_ROUTE_POLICY` による default/legacy route policy SSOT と fail-fast 境界を固定した記録。
- **[WSM-P5-min3 実装ロック](../../development/current/main/phases/phase-29cc/29cc-162-wsm-p5-min3-default-hako-lane-lock-ssot.md)** - `default` route を hako-lane 名義（bridge実装）へ切替し、legacy との差分 parity gate を固定した記録。
- **[WSM-P5-min4 実装ロック](../../development/current/main/phases/phase-29cc/29cc-163-wsm-p5-min4-hako-lane-bridge-shrink-lock-ssot.md)** - default(hako-lane) の native/bridge 計画境界（`NativePilotShape`/`BridgeRustBackend`）を固定した記録。
- **[WSM-P5-min5 実装ロック](../../development/current/main/phases/phase-29cc/29cc-164-wsm-p5-min5-native-helper-lock-ssot.md)** - default(hako-lane) pilot 1shape の native helper 実体路（bridge非依存）を固定した記録。
- **[WSM-P5-min6 実装ロック](../../development/current/main/phases/phase-29cc/29cc-165-wsm-p5-min6-shape-expand-lock-ssot.md)** - native shape-table を `const->copy->return` まで拡張し、default(hako-lane) の fallback 範囲を縮退した記録。
- **[WSM-P5-min7 実装ロック](../../development/current/main/phases/phase-29cc/29cc-166-wsm-p5-min7-shape-trace-lock-ssot.md)** - route trace に shape_id を固定出力し、default/legacy 判定の観測契約を lock した記録。

## 🚀 クイックスタート

> 注記: 下記の `--compile-wasm` / `--aot` は長期目標の一般経路。現時点で日常運用する browser demo は上の「現行の運用入口（G2固定）」を使うこと。

### WASM コンパイル
```bash
# 基本コンパイル
./target/release/hakorune --compile-wasm program.hako
# -> program.wasm が出力される

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
