Status: SSOT
Scope: hakorune/selfhost line の「モジュール単位オブジェクト化 + リンク + 3層キャッシュ」設計を固定する。
Related:
- docs/development/current/main/design/selfhost-stageb-json-streaming-design.md
- docs/development/current/main/design/de-rust-compiler-thin-rust-roadmap-ssot.md
- docs/development/current/main/design/selfhost-bootstrap-route-ssot.md
- docs/development/current/main/phases/phase-29x/29x-68-cache-key-determinism-ssot.md
- docs/development/current/main/phases/phase-29x/29x-69-l1-mir-cache-ssot.md
- docs/development/current/main/phases/phase-29x/29x-70-l2-object-cache-ssot.md
- docs/development/current/main/phases/phase-29x/29x-71-l3-link-cache-ssot.md
- docs/development/current/main/phases/phase-29x/29x-72-cache-gate-integration-done-sync-ssot.md
- tools/cache/phase29x_cache_keys.sh
- tools/checks/phase29x_cache_key_determinism_guard.sh
- tools/cache/phase29x_l1_mir_cache.sh
- tools/checks/phase29x_l1_mir_cache_guard.sh
- tools/cache/phase29x_l2_object_cache.sh
- tools/checks/phase29x_l2_object_cache_guard.sh
- tools/cache/phase29x_l3_link_cache.sh
- tools/checks/phase29x_l3_link_cache_guard.sh
- tools/checks/phase29x_cache_gate_integration_guard.sh
- tools/smokes/v2/profiles/integration/apps/phase29x_cache_lane_gate_vm.sh
- docs/reference/language/using.md
- docs/reference/abi/ABI_BOUNDARY_MATRIX.md

# Hako Module Cache Build SSOT

## Context

- 現状の Stage-B / Stage1 build は「全体を再生成して最後に実行/リンク」寄りで、モジュール単位の再利用境界が弱い。
- `hako.toml` には依存解決情報（`[module_roots]`, `[modules]`, `[using]`）があるため、モジュール単位 build graph は構築可能。
- 目的は、意味論を変えずに build 時間と再実行コストを下げること。

## Goals

- モジュール単位で `MIR -> object -> link` を分離し、再利用可能なキャッシュ境界を固定する。
- 依存モジュールの変更影響を最小化し、変更なしモジュールは再コンパイルしない。
- daily/milestone 運用で使える deterministic cache key を定義する。
- strict/dev で非canonical混入を早期検出する（fail-fast）。

## Non-Goals

- 言語仕様変更（構文/意味論/GC方針）を行わない。
- AST rewrite や評価順序変更を導入しない。
- Rust lane 依存を hidden fallback で温存しない。

## Build Unit Contract

- build unit は「解決後モジュール（resolved module）」を 1単位とする。
- モジュール解決の正本は `hako.toml`（`[module_roots]` + `[modules]` + `[using]`）とする。
- DAG 化できない依存循環は fail-fast（silent fallback しない）。
- 出力 artifact は build unit ごとに独立保存する（モノリシック出力へ戻さない）。

## Artifact Model

各 module で次の artifact を扱う。

1. `mir artifact`:
`MIR(JSON v0)` の canonical 出力（compile/lower の入力境界）。
2. `abi artifact`:
公開シグネチャ・export 形状の digest 元データ（依存先の再コンパイル判定に使う）。
3. `object artifact`:
backend 依存の object/bitcode（LLVM line）。
4. `link manifest`:
entry, object list（順序固定）, link flags, ABI digest を束ねるリンク入力。
5. `linked binary`:
最終実行物。

## Cache Key Model

### ModuleCompileKey (L1: MIR)

`hash(source_digest, resolver_digest, toolchain_digest, profile_digest, deps_interface_digest[])`

- `source_digest`: モジュール本体（前処理後の実入力）。
- `resolver_digest`: using/module_roots/modules 解決結果（依存パス込み）。
- `toolchain_digest`: hakorune/stage1 compiler の build identity。
- `profile_digest`: strict/dev/release + relevant env toggles。
- `deps_interface_digest[]`: 直接依存の interface digest（順序固定）。

### ObjectKey (L2: object)

`hash(mir_digest, backend_digest, abi_boundary_digest, target_digest)`

- `backend_digest`: backend 種別と lowering policy。
- `abi_boundary_digest`: C ABI / type ABI の正規境界 digest。
- `target_digest`: target triple / linker mode など。

### LinkKey (L3: link)

`hash(entry_module, ordered_object_digest[], link_flags_digest, runtime_abi_digest)`

- object list は deterministic order（topo + stable tie-break）で固定。
- runtime ABI の境界差分があれば link cache は必ず無効化する。

## Three-Layer Cache

### L1: MIR cache

- hit 条件: `ModuleCompileKey` 一致。
- miss 時: compile/lower を実行し、`mir artifact` + `abi artifact` を保存。

### L2: Object cache

- hit 条件: `ObjectKey` 一致。
- miss 時: backend compile を実行し、`object artifact` を保存。

### L3: Link cache

- hit 条件: `LinkKey` 一致。
- miss 時: link を実行し、`linked binary` と manifest を保存。

## Cache Layout (v1)

```text
target/hako-cache/v1/
  <profile>/
    <target>/
      mir/<module-id>/<module-compile-key>.mir.json
      abi/<module-id>/<module-compile-key>.abi.json
      obj/<module-id>/<object-key>.o
      link/<entry-module>/<link-key>.manifest.json
      bin/<entry-module>/<link-key>/app
```

- `<module-id>` は解決後 canonical 名（dot name）を使う。
- cache schema 変更時は `v1 -> v2` を増やし、破壊的上書きはしない。

## Fail-Fast Contracts

代表契約（タグ名は安定化対象）:

- `[freeze:contract][cache-build/module-graph-cycle]`
- `[freeze:contract][cache-build/module-not-found]`
- `[freeze:contract][cache-build/module-roots-ambiguous]`
- `[freeze:contract][cache-build/non-canonical-abi]`
- `[freeze:contract][cache-build/link-input-missing]`

上記は dev/strict で必須。non-strict でも silent success は許可しない。

## Rollout Order (CB lane)

1. `CB-0` docs 固定（本SSOT）: done。
2. `CB-1` key determinism:
module/object/link key を同一入力で再現可能にし、差分理由を観測可能にする（done）。
3. `CB-2` L1 MIR cache:
MIR/ABI artifact を module 単位で保存・再利用（done）。
4. `CB-3` L2 object cache:
backend compile を module object 再利用で短縮（done）。
5. `CB-4` L3 link cache:
entry + object set が同一なら link を再利用（done）。
6. `CB-5` gate integration:
daily/milestone に cache hit/miss 観測を追加し、効果を契約化（done）。

## Acceptance Criteria

- 同一入力を連続実行したとき、2回目以降は L1/L2/L3 の少なくとも一部が hit する。
- 依存1モジュールのみ変更したとき、非依存モジュールは再コンパイルされない。
- ABI 境界変更時は link cache が無効化され、誤リンクしない。
- strict/dev で非canonical 混入時は fail-fast タグで停止する。

## Decision

- Decision: accepted（2026-02-13）
- 理由: `.hako` 依存解決資産（`hako.toml`）を活かしつつ、最小差分で build 時間短縮と再現性向上を両立できるため。
