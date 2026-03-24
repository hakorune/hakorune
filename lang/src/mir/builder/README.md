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
  - outer Program(JSON) input validation/coercion now lives in `hako.mir.builder.internal.program_json_input_contract`, so this file no longer keeps null/header checks inline.
  - the normal registry-first Program(JSON)->MIR authority block now lives in `hako.mir.builder.internal.registry_authority`
  - the non-registry/internal fallback chain now lives in `hako.mir.builder.internal.fallback_authority`
  - the delegate/provider compat gate now lives in `hako.mir.builder.internal.delegate_provider`
  - the delegate-side `user_box_decls` finalize plus handoff into shared normalize now also live in `hako.mir.builder.internal.delegate_finalize`
  - the shared outer finalize chain now also lives in `hako.mir.builder.internal.finalize_chain`
  - this file keeps route sequencing, generic unsupported/no-match decision, and only the remaining outer compat tails around those internal owners
  - outer Program(JSON) entry validation now stays on the internal owner `BuilderProgramJsonInputContractBox`, while `_emit_mir_from_program_json_text_checked(...)` keeps route dispatch only
  - Program(JSON) fail-fast tiny leaves now stay on that internal input-contract owner instead of widening `MirBuilderBox.hako`
  - route sequencing is owner-local via `BuilderFuncDefsGateBox.lower_if_enabled(...)`, `_emit_internal_program_json(...)`, and `_emit_delegate_program_json(...)`; raw env/hostbridge branching does not stay duplicated inline
  - delegate compat gate/provider call is now internal via `BuilderDelegateProviderBox.try_emit(...)`, and delegate-side finalize is now internal via `BuilderDelegateFinalizeBox.finalize_mir(...)`, so the delegate lane reads as internal gate/provider -> internal finalize -> shared normalize
  - shared finalize chain is now internal via `BuilderFinalizeChainBox.apply(...)` and `BuilderFinalizeChainBox.log_fail(...)`, so route leaves no longer carry inject/methodize/normalize/fail-tag logic inline
  - the remaining source-entry compat tail stays owner-local via `_emit_program_json_from_source_raw(...)`, while the func-def pre-lowering gate now lives in `hako.mir.builder.internal.func_defs_gate`; those tiny leaves no longer mix inline with checked handoff
  - internal route leaves are owner-local via `_try_emit_loop_force_jsonfrag(...)`, `_try_emit_registry_program_json(...)`, and `_try_emit_fallback_program_json(...)`, so `_emit_internal_program_json(...)` only shows loop-force / registry / fallback / fail-fast route order
  - internal unsupported tail is now isolated in `_fail_internal_unsupported(...)` and `_program_json_has_ternary(...)`, so `_emit_internal_program_json(...)` stays a readable route table
  - mini internal lowers are allowed to keep tiny owner-local stringify helpers such as `_coerce_text_compat(...)` when their only legacy `"" + x` usage is the Program(JSON) entry coercion
  - `builder_config` and `delegate_finalize` now also centralize env/program-json text coercion through owner-local `_coerce_text_compat(...)`, so route/config owners no longer repeat raw `"" + x` on their remaining compat seams
- `emit_from_source_v0(source_text: String, opts: Map|Null) -> String|Null`
  - Source-entry shim only; current stage1 authority no longer depends on this route.
  - source-entry compat now lives in `MirBuilderSourceCompatBox`; `MirBuilderBox` keeps the Program(JSON) route sequencing, while the compat box owns source-entry coercion / source->Program(JSON) check / Program(JSON)->MIR handoff.
  - direct `BuildBox.emit_program_json_v0(...)` check remains owner-local before delegating through `MirBuilderProgramJsonBuildBox.emit_program_json_v0(...)`, so the source-entry shim now has an explicit raw leaf instead of keeping the BuildBox call inline.

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
