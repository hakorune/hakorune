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
    - 構造のみ実装（`HakoCli` box にコマンド別のメソッドを定義）。
    - 各コマンドはまだプレースホルダで、`"[hakorune] <cmd>: not implemented yet"` を出力して終了コード 90–93 を返す。
    - 実際のパイプライン（Stage‑B / MirBuilder / AotPrep / ny-llvmc など）への接続は後続フェーズで段階的に実装する。
  - Design reference:
    - `docs/development/runtime/cli-hakorune-stage1.md` を Stage1 CLI の仕様 SSOT として参照すること。

## Notes
- Runner 層は「構造とオーケストレーション専用レイヤ」として扱う。
  - 言語意味論・最適化ロジックは compiler / opt / AotPrep に留める。
  - VM/LLVM の実行コアは Rust 側（Stage0 / NyRT）に委譲する。
- current selfhost authority entry is `stage1_cli_env.hako`; `launcher.hako` / raw subcmd lane は authority ではなく compat/future retire target として扱う。
- source-only authority call is isolated in `Stage1SourceMirAuthorityBox` inside `stage1_cli_env.hako`; keep Main as a thin dispatcher.
- shared MIR materialization/validation is isolated in `Stage1MirResultValidationBox` inside `stage1_cli_env.hako`; keep result checking out of Main and out of the compat box.
- explicit Program(JSON) compat keep is quarantined in `Stage1ProgramJsonCompatBox` inside `stage1_cli_env.hako`; keep both the mixed-input fail-fast gate and the explicit compat call separate from the source-mainline authority, then retire that box slice-by-slice.
- Fail-Fast 原則:
  - 未実装コマンドや不正な引数は明示的なメッセージ＋非0終了コードで返す。
  - 暗黙のフォールバックや静かな無視は行わない。
