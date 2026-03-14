# MirBuilderBox — Program(JSON v0) → MIR(JSON v0)

Pointers
- repo-wide selfhost compiler ownership map:
  - `docs/development/current/main/design/selfhost-compiler-structure-ssot.md`
- current bootstrap/authority contract:
  - `docs/development/current/main/design/selfhost-bootstrap-route-ssot.md`
- active reduction phase:
  - `docs/development/current/main/phases/phase-29ch/README.md`

Responsibility
- Convert Stage‑B Program(JSON v0) into MIR(JSON v0) for VM/LLVM lines.
- Keep the boundary/contract stable and Fail‑Fast; no silent fallback to stub MIR.

Interface (stable)
- `emit_from_program_json_v0(program_json: String, opts: Map|Null) -> String|Null`
  - Returns canonical MIR(JSON v0) on success; returns null and prints a tagged diagnostic on failure.
  - delegate branch now finalizes returned MIR locally by injecting `user_box_decls` before normalization; this is the first `.hako`-side ownership move inside the Program(JSON)->MIR path.
  - gate decisions (`internal_on`, `delegate_on`, `selfhost_no_delegate_on`, `methodize_on`, `jsonfrag_normalize_on`) are centralized in `hako.mir.builder.internal.builder_config`, so this file is the owner of route sequencing, not raw env reads.
  - the normal registry-first Program(JSON)->MIR authority block now lives in `hako.mir.builder.internal.registry_authority`
  - the non-registry/internal fallback chain now lives in `hako.mir.builder.internal.fallback_authority`
  - this file keeps route sequencing, generic unsupported/no-match decision, and compat tails around those internal owners
  - outer Program(JSON) entry validation now stays owner-local via `_coerce_program_json_checked(...)` and `_emit_mir_from_program_json_text_checked(...)`, so the public entrypoint only shows checked handoff plus route dispatch
  - route sequencing is owner-local via `_lower_func_defs_if_enabled(...)`, `_emit_internal_program_json(...)`, and `_emit_delegate_program_json(...)`; raw env/hostbridge branching does not stay duplicated inline
  - internal unsupported tail is now isolated in `_fail_internal_unsupported(...)` and `_program_json_has_ternary(...)`, so `_emit_internal_program_json(...)` only shows loop-force / registry / fallback / fail-fast route order
- `emit_from_source_v0(source_text: String, opts: Map|Null) -> String|Null`
  - Source-entry shim only; current stage1 authority no longer depends on this route.
  - source-entry coercion / source->Program(JSON) check / Program(JSON)->MIR handoff now stay owner-local via `_coerce_source_text_checked(...)`, `_emit_program_json_from_source_checked(...)`, and `_emit_mir_from_source_program_json_checked(...)`.
  - direct `BuildBox.emit_program_json_v0(...)` check remains owner-local before delegating to `emit_from_program_json_v0(...)`.

Tags (Fail‑Fast, stable)
- `[mirbuilder/input/null]` — input is null
- `[mirbuilder/input/invalid]` — header missing (version/kind)
- `[mirbuilder/internal/unsupported] ...` — Program(JSON) shape not yet supported by internal lowers
- `[builder/selfhost-first:unsupported:defs_only]` — only defs を lowering できる状態（main なし）のため中止
- `[builder/selfhost-first:unsupported:no_match]` — internal lowers / defs のどちらにもマッチせず中止
- `[builder/selfhost-first:unsupported:inject_funcs_null]` — internal lower 後の defs inject が null を返したため中止
- `[builder/selfhost-first:unsupported:methodize_null]` — internal lower 後の methodize が null を返したため中止
- `[builder/selfhost-first:unsupported:normalize_null]` — internal lower 後の normalizer が null を返したため中止
- `[builder/funcs:unsupported:loopform]` — Loop を含むが LoopForm 制約に当てはまらないか、selfhost builder ではまだ扱えない Loop 構造のため中止（Rust provider に退避可能）
- `[builder/funcs:fail:no-main]` — inject_funcs が main を含まない MIR に defs を差し込もうとしたため拒否（`HAKO_MIR_BUILDER_REQUIRE_MAIN=1` 時）
- `[mirbuilder/delegate]` — delegate path selected（Runner/extern provider 経由）
- `[mirbuilder/delegate/missing]` — delegate/provider not wired yet

Toggles
- `HAKO_MIR_BUILDER_INTERNAL=0/1` — internal lowers gate（既定=1）
- `HAKO_MIR_BUILDER_REGISTRY=0/1` — pattern registry gate（既定=1）
- `HAKO_MIR_BUILDER_DELEGATE=1` — use Runner/extern provider (`env.mirbuilder.emit`) 経由で Program→MIR
- `HAKO_SELFHOST_NO_DELEGATE=1` — selfhost-first 時に delegate 経路を完全無効化し、internal lowers のみで成否を判定する
- `HAKO_MIR_BUILDER_FUNCS=1` — enable defs lowering via `FuncLoweringBox.lower_func_defs`
- `HAKO_MIR_BUILDER_METHODIZE=0/1` — call→mir_call(Method) rewrite。既定ON（未設定または"1"）、"0" のときのみ無効化。
- `HAKO_MIR_BUILDER_JSONFRAG_NORMALIZE=1` — apply JsonFrag normalizer to selfhost/provider output
- `HAKO_MIR_BUILDER_LOOP_FORCE_JSONFRAG=1` — dev‑only: minimal loop MIR を強制生成（テスト用）
- `HAKO_MIR_BUILDER_REQUIRE_MAIN=1` — inject_funcs で `"name":"main"` を持たない MIR に defs を追加するのを禁止（既定=0）

Notes
- Box‑First policy: define the interface and tags first, then evolve implementation behind the same contract.
- Large payloads: implementation is currently string/JsonFrag‑based; later phases may stream or segment JSON, but I/F は維持する。
- Phase 25.1b では `FuncLoweringBox` との連携を拡張し、「main + defs」構造を前提とした multi‑function MIR の土台を整える（defs‑only モジュールは Fail‑Fast）。 
