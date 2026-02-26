# WASM Backend Unsupported Inventory

## Last Updated
- 2026-02-26

## Scope
This document tracks current unsupported surface for the Rust WASM backend based on actual source status.

- Entry: `src/backend/wasm/mod.rs`
- Instruction lowering: `src/backend/wasm/codegen/instructions.rs`
- BoxCall lowering: `src/backend/wasm/codegen/builtins.rs`
- Runtime imports: `src/backend/wasm/runtime.rs`

## Current Implementation Snapshot

### 1. Extern call support (partial)
Supported extern names in `instructions.rs`:
- `env.console.log`
- `env.console.warn`
- `env.console.error`
- `env.console.info`
- `env.console.debug`
- `env.canvas.fillRect`
- `env.canvas.fillText`
- `env.canvas.clear`
- `env.canvas.strokeRect`

Unsupported extern calls fail-fast with:
- `Unsupported extern call: <name> (supported: ...)`

### 2. BoxCall support (partial)
Supported methods in `builtins.rs`:
- `toString`
- `print`
- `equals`
- `clone`
- `log`
- `info`
- `debug`
- `warn`
- `error`

Unsupported methods fail-fast with:
- `Unsupported BoxCall method: <name> (supported: ...)`

### 3. Core instruction support (partial)
Supported core instructions in `instructions.rs` include:
- `Const`, `BinOp`, `Compare`, `Return`
- `Jump`, `Branch`
- `Copy`
- `ReleaseStrong` (no-op lowering)
- `KeepAlive` (no-op lowering)

Still unsupported (fail-fast):
- `Load`
- `Store`
- other MIR instructions not explicitly matched

### 4. Executor status
- `src/backend/wasm/executor.rs` is not currently active in mainline.
- `src/backend/wasm/mod.rs` exports compiler/codegen/runtime only.

## WSM-01 + WSM-02a Decision (accepted)
- Do not add broad fallback behavior.
- Keep unsupported paths fail-fast with explicit supported-list diagnostics.
- Keep this inventory synchronized to actual source files.

## WSM G2 ブラウザデモとの接続
- `projects/nyash-wasm/nyash_playground.html` を G2 run loop 稼働対象として再定義し、`apps/tests/phase29cc_wsm02d_demo_min.hako` の `console.log/warn/error/info/debug` 呼び出しと gate を一致させる狙いを `docs/development/current/main/phases/phase-29cc/29cc-133-wsm-g2-browser-demo-task-plan.md` が定義している。
- `ConsoleBox.group` のような scope-out helper は `phase29cc_wsm02d_demo_unsupported_boundary_vm.sh` で fail-fast なので、このドキュメントで inventory に登録して unsupported inventory 側の記録を補完する。
- headless smoke `phase29cc_wsm_g2_browser_run_*` を `tools/checks/dev_gate.sh wasm-demo-g2` 等に追加する際は、この unsupported list を参照して fail-fast 判定を gate に突き合わせる。

## WSM G3-min1 Gap Inventory（canvas/enhanced demo）

### 対象デモ
- `projects/nyash-wasm/canvas_playground.html`
- `projects/nyash-wasm/enhanced_playground.html`

### 呼び出しギャップ（優先順）
1. Canvas drawing core
   - used by demo: `setFillStyle`, `setStrokeStyle`, `setLineWidth`, `strokeRect`, `beginPath`, `arc`, `fill`, `stroke`, `clear`
   - backend status: `env.canvas.fillRect`, `env.canvas.fillText`, `env.canvas.clear`, `env.canvas.strokeRect` が supported
   - gap: 残りメソッド（`beginPath`/`arc`/`fill`/`stroke` など）の extern contract / runtime import / codegen route 未整備
2. Console helper expansion
   - used by demo: `console.log/error`（JS側は多用）
   - backend status: `log/warn/error/info/debug` は supported
   - gap: `group/groupEnd/separator` は scope-out（fail-fast維持）
3. DOM/event bridge
   - demo needs direct DOM updates and animation loop (`requestAnimationFrame`)
   - backend status: dedicated extern contract なし
   - gap: `env.dom.*` / `env.anim.*` の語彙設計未着手（G3以降）

## Next Candidates (WSM-02+)
- Expand extern-call coverage beyond current 3 names.
- Expand BoxCall coverage for core methods used by selfhost fixtures.
- Cover `Load` / `Store` path required by assignment/local deep shapes.
- Add wasm-focused gate fixtures that assert supported/unsupported boundaries.
- G3 queue:
  - `canvas.beginPath` の 1語彙を `1 blocker = 1 shape` で追加し、fixture/gate を先に固定する。
