---
Status: Active (WSM-G2-min1/min2/min3 done, G3 prep next)
Decision: pending
Date: 2026-02-26
Scope: wasm lane G2 "browser demo-run minimal" のタスクを docs-first で固定する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-120-wasm-demo-goal-contract-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-130-wsm02d-min2-demo-min-fixture-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-131-wsm02d-min3-demo-unsupported-boundary-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-132-wsm02d-min4-milestone-gate-promotion-lock-ssot.md
  - projects/nyash-wasm/README.md
  - docs/guides/wasm-guide/planning/unsupported_features.md
---

# 29cc-133 WSM-G2 Browser Demo Task Plan

## Purpose
WSM lane が `projects/nyash-wasm` を G2 (browser demo minimum) で再到達するための直近 3〜5 タスクを docs-first で固める。すでに確定済みの SSOT（29cc-120/130/131/132）を踏まえ、run loop 実装・gate・dev guide を並行して回す。

## Task list

1. **G2 run loop baseline (WSM-G2-min1, done)**
   - `projects/nyash-wasm/nyash_playground.html` を「run ボタンが `ConsoleBox` 5メソッドを呼び出す」最小デモとして定義し、29cc-120 の G2 記述と Phase 29cc の fixture (`apps/tests/phase29cc_wsm02d_demo_min.hako`) を紐づける。
   - コンパイル側は 29cc-130 で追加した demo-min fixture と `tests/wasm_demo_min_fixture.rs`、`tools/smokes/v2/profiles/integration/apps/phase29cc_wsm02d_demo_min_boundary_vm.sh` で現状固定済み。ここではその出力が `nyash_playground` の `console.log/warn/error/info/debug` 呼び出しと一致することを docs に拾う。
   - Acceptance: `projects/nyash-wasm/build.sh`（もしくは今後の `--compile-wasm` 書式）で wasm を再ビルドし、ローカル HTTP サーバーで `nyash_playground.html` を開いて Run ボタンをクリックしたときにログが 1行ずつ流れる手順を書く。これにより「ブラウザ run loop の最小仕様」が決まる。

2. **G2 run automation (WSM-G2-min2, done)**
   - ブラウザ run loop を headless で再現する smoke を追加（例: `tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_g2_browser_run_vm.sh`）し、`tools/checks/dev_gate.sh wasm-demo-g2` エントリで milestone 確認を入れる。
   - スモークは `projects/nyash-wasm/build.sh` で wasm を産み、`python3 -m http.server` などで静的サーバーを立て、`node`/`puppeteer`/`playwright` などの軽量 headless で `nyash_playground.html` を開いて Run → コンソール出力を期待値（`wsm02d_demo_min_*`）と比較する構成の手順を docs でまとめる。
   - Refer to bootstrap scripts such as `phase29cc_wsm02d_demo_min_boundary_vm.sh` for instrumentation (logging markers, strict failure on missing markers) so the cc gate remains deterministic.

3. **WSM guide alignment (WSM-G2-min3, done)**
   - `docs/guides/wasm-guide/wasm_quick_start.md` および `docs/guides/wasm-guide/README.md` の G2/Run セクションに、「`projects/nyash-wasm` を再building・Run する手順」「Headless smoke のコマンド」「Acceptance gate を通ること」の 3点を追記し、`29cc-133` を新しい SSOT pointer として紹介する。
   - `docs/guides/wasm-guide/planning/unsupported_features.md` にも、G2 で fail-fast させるべき追加 boundary（`ConsoleBox.group` など）を追記して `phase29cc_wsm02d_demo_unsupported_boundary_vm.sh` の目的を明示する。

4. **G3 prep (WSM-G3-min1, active next)**
   - G2 通過後は `projects/nyash-wasm/canvas_playground.html` や `enhanced_playground.html` に載っていた歴史的デモを G3 タスクとして再現する予定。これらの HTML で使われていた Box/Canvas/DOM API を `29cc-118` 以降の BoxCall/ExternCall coverage 拡張に接続するため、当該 HTML が呼ぶメソッド/extern 仕様と現行 backend のギャップを `docs/guides/wasm-guide/planning/unsupported_features.md` にまとめ、優先順位付きで列挙する。

## Next pointer
- この doc は `CURRENT_TASK.md`/`10-Now.md`/`phase-29cc/README.md` の WSM laneセクションから参照される。
- 次に必要なのは 29cc-120 で示した G3 目標を具体的 fixture/gate として後追いすること。
