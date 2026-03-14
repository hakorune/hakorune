# Runner Facade / Stage1 CLI — Runner Layer Guide

Pointers:
- repo-wide selfhost compiler ownership map:
  - `docs/development/current/main/design/selfhost-compiler-structure-ssot.md`
- current bootstrap/authority contract:
  - `docs/development/current/main/design/selfhost-bootstrap-route-ssot.md`
- active MIR-direct bootstrap phase:
  - `docs/development/current/main/phases/phase-29ch/README.md`

## Responsibility
- Provide script-side orchestration primitives for execution:
  - Runner facade (`runner_facade.hako`) for entry selection and pre/post hooks.
  - Stage1 CLI launcher (`launcher.hako`) for top-level command dispatch.
- Delegate actual execution to existing backends（Rust VM / LLVM / ny-llvmc）。既定挙動は変えない。

## Files

- `stage1_cli.hako`
  - Contract:
    - Entry: `Main.main(args: array<string>) -> i64`
    - Role: embedded/raw Stage1 CLI lane for emit/run bootstrap contracts.
  - Current status:
    - authority is still `stage1_cli_env.hako`; this file is a future-retire/raw subcmd lane
    - checked `BuildBox` / `MirBuilderBox` calls stay behind owner-local helpers
    - source/program-json orchestration stays behind same-file helpers (`_resolve_emit_program_source_text(...)`, `_resolve_program_json_for_emit_mir(...)`, `_resolve_program_json_for_run(...)`, `_load_program_json_from_path_or_source(...)`)

- `runner_facade.hako`
  - Contract（draft）:
    - Entry: `Runner.run(entry: string, args: array<string>) -> i64`
    - Gate: `HAKO_SCRIPT_RUNNER=1`（default OFF）。
  - Role:
    - Script-first runner facade（Phase 20.10）。
    - Pre-hooks: validate entry/args, emit short diagnostics。
    - Post-hooks: normalize result / metrics（将来）。
  - Notes:
    - Keep this layer pure; platform I/O は C-ABI 側に委譲。
    - Fail-Fast: invalid entry/args は非0で即終了。
    - Short diagnostics:
      - Success: `[script-runner] invoke`
      - Failure: `[script-runner] invoke: FAIL`

- `launcher.hako`
  - Contract（draft）:
    - Entry: `Main.main(args: array<string>) -> i64`
    - Role: Stage1 hakorune CLI のトップレベル dispatcher。
      - コマンド: `run` / `build` / `emit` / `check`（詳細は docs/development/runtime/cli-hakorune-stage1.md）。
  - Current status（Phase 25.1）:
    - `build exe` / `emit program-json` / `emit mir-json` は Stage-B / MirBuilder / codegen bridge へ接続済み。
    - `run` / `check` はまだプレースホルダで、`"[hakorune] <cmd>: not implemented yet"` を出力して終了コード 90–93 を返す。
    - checked Program(JSON) / MIR routes は owner-local helper に固定され、caller-side choreography も same-file helper に寄せている。
  - Design reference:
    - `docs/development/runtime/cli-hakorune-stage1.md` を Stage1 CLI の仕様 SSOT として参照すること。

## Notes
- Runner 層は「構造とオーケストレーション専用レイヤ」として扱う。
  - 言語意味論・最適化ロジックは compiler / opt / AotPrep に留める。
  - VM/LLVM の実行コアは Rust 側（Stage0 / NyRT）に委譲する。
- current selfhost authority entry is `stage1_cli_env.hako`; `launcher.hako` / raw subcmd lane は authority ではなく compat/future retire target として扱う。
- shared env/source resolution contract is isolated in `Stage1InputContractBox` inside `stage1_cli_env.hako`; keep input shaping out of `Main` and out of authority/compat boxes.
- emit-program authority is isolated in `Stage1ProgramAuthorityBox` inside `stage1_cli_env.hako`; keep defs synthesis/materialization out of `Main`.
- materialized Program(JSON) validation is isolated in `Stage1ProgramResultValidationBox` inside `stage1_cli_env.hako`; keep emit-program on the same thin-dispatch pattern as emit-mir.
- shared Program(JSON) -> MIR checked call is isolated in `Stage1ProgramJsonMirCallerBox` inside `stage1_cli_env.hako`; keep the direct `MirBuilderBox.emit_from_program_json_v0(...)` contract out of both source authority and compat keep.
- source-only authority call is isolated in `Stage1SourceMirAuthorityBox` inside `stage1_cli_env.hako`; the box now owns the source-entry `BuildBox.emit_program_json_v0(...)` shim locally and delegates only the Program(JSON) -> MIR step through `Stage1ProgramJsonMirCallerBox`.
- shared MIR materialization/validation is isolated in `Stage1MirResultValidationBox` inside `stage1_cli_env.hako`; keep result checking out of Main and out of the compat box.
- explicit Program(JSON) compat keep is quarantined in `Stage1ProgramJsonCompatBox` inside `stage1_cli_env.hako`; current callers are probe/helper-owned only, so keep it outside reduced authority evidence and reuse the shared `Stage1ProgramJsonMirCallerBox` contract slice-by-slice.
- Fail-Fast 原則:
  - 未実装コマンドや不正な引数は明示的なメッセージ＋非0終了コードで返す。
  - 暗黙のフォールバックや静かな無視は行わない。
