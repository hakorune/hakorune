# Runner Facade / Stage1 CLI — Runner Layer Guide

Pointers:
- repo-wide selfhost compiler ownership map:
  - `docs/development/current/main/design/selfhost-compiler-structure-ssot.md`
- current bootstrap/authority contract:
  - `docs/development/current/main/design/selfhost-bootstrap-route-ssot.md`
- owner/proof reopen index:
  - `docs/development/current/main/design/frontend-owner-proof-index.md`
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
    - Entry: `Main.main() -> i64`
    - Role: embedded/raw Stage1 CLI lane for env/bootstrap emit/run contracts.
  - Current status:
    - authority is still `stage1_cli_env.hako`; this file is a future-retire/raw subcmd lane
    - `Stage1CliConfigBox.from_env()` is now the live env contract owner; `stage1_main(...)` reads one canonical config map (`mode`, `backend`, `source_path`, `source_text`, `program_json_path`) instead of re-reading env inline
    - route selection now lives in same-file `Stage1CliDispatchBox`, so `stage1_main(...)` and the raw `emit` selection no longer carry policy inline; the raw `run` body remains the same-file action path after `Stage1CliRawSubcommandInputBox` materializes its request
    - checked `BuildBox` / `MirBuilderBox` calls stay behind owner-local helpers
    - source/program-json orchestration now lives behind same-file `Stage1CliProgramJsonInputBox`, so `Stage1Cli` no longer keeps placeholder resolve, source-text readback, or path/source Program(JSON) shaping inline
    - raw/subcmd `emit mir-json` / `run` argv parsing now lives behind same-file `Stage1CliRawSubcommandInputBox`, so the future-retire raw lane no longer keeps option parsing or `NYASH_SCRIPT_ARGS_JSON` assembly inline
    - raw/subcmd wrappers may still read env for compat keep, but the live `stage1_main(...)` lane now passes typed source/program-json inputs through those helpers instead of re-resolving env at each tail
    - emit-mir checked contract is also split owner-locally (`_coerce_program_json_for_emit_mir_checked(...)`, `_emit_mir_from_program_json_text_checked(...)`, `_coerce_mir_output_checked(...)`, `_emit_validated_mir_from_program_json_text(...)`) so the raw subcmd lane no longer mixes Program(JSON) input validation, MirBuilder call, and MIR output validation inline
    - Program(JSON) marker predicates are now also centralized behind `_program_json_text_present(...)` and `_program_json_has_markers(...)`, so inline payload probing, emit-program output validation, and emit-mir input validation share one same-file contract
    - visible legacy stringify in env/debug/argv shaping is now centralized behind `_coerce_text_compat(...)`, so raw/subcmd cleanup can reduce `"" + x` residue without changing the lane contract

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
    - Entry: `Main.main() -> i64`
    - Role: Stage1 hakorune CLI のトップレベル dispatcher。
      - コマンド: `run` / `build` / `emit` / `check`（詳細は docs/development/runtime/cli-hakorune-stage1.md）。
  - Current status（Phase 25.1）:
    - `build exe` / `emit program-json` / `emit mir-json` は Stage-B / MirBuilder / backend boundary へ接続済み。
    - `run` / `check` はまだプレースホルダで、`"[hakorune] <cmd>: not implemented yet"` を出力して終了コード 90–93 を返す。
    - `LauncherInputContractBox` now owns argv/bootstrap parse+validate for `build exe`, `emit program-json`, `emit mir-json`, and `HAKORUNE_BOOTSTRAP_*`, so `HakoCli` is down to request parse -> checked execute dispatch
    - checked Program(JSON) / MIR routes は owner-local helper に固定され、caller-side choreography も same-file helper に寄せている。
    - `emit mir-json` checked contract is also split owner-locally (`_coerce_program_json_for_emit_mir_checked(...)`, `_emit_mir_from_program_json_text_checked(...)`, `_coerce_mir_output_checked(...)`) so the launcher lane no longer mixes Program(JSON) validation, MirBuilder call, and MIR output validation inline.
    - `emit program-json` checked tail is also split owner-locally (`_emit_program_json_raw(...)`, `_coerce_program_json_output_checked(...)`) so the launcher lane no longer mixes BuildBox call and Program(JSON) validation inline.
    - Program(JSON) marker predicates are now also centralized behind `_program_json_text_present(...)` and `_program_json_has_markers(...)`, so launcher emit-program output validation and emit-mir input validation share one same-file contract.
    - artifact file I/O and stdout-vs-file output selection now live in same-file `LauncherArtifactIoBox`, so `cmd_emit_program_json(...)` / `cmd_emit_mir_json(...)` / bootstrap paths no longer branch directly on readback/output side effects inline.
    - checked Program(JSON) / MIR payload validation and checked source->Program / Program->MIR handoff now also live in same-file `LauncherPayloadContractBox`, so `HakoCli` no longer mixes payload validation with top-level dispatch/build orchestration inline.
    - `build exe` now owns only temp MIR handoff / default output-path helper shape and lowers compile/link through `_compile_object_from_mir_path_checked(...)` / `_link_exe_object_checked(...)`, so the launcher lane no longer mixes compile/link fail-fast tails inline.
    - top-level route selection and bootstrap-vs-normal entry routing now live in same-file `LauncherDispatchBox`, so `HakoCli.run(...)` / `HakoCli.run_native_entry()` are thin delegates instead of carrying command policy inline.
    - visible legacy stringify/path coercion is now centralized behind `_coerce_text_compat(...)` and `_non_empty_text(...)`, so future string-coercion cleanup can tighten the contract owner-by-owner without touching every launcher call site at once.
  - Design reference:
    - `docs/development/runtime/cli-hakorune-stage1.md` を Stage1 CLI の仕様 SSOT として参照すること。

## Notes
- Runner 層は「構造とオーケストレーション専用レイヤ」として扱う。
  - 言語意味論・最適化ロジックは compiler / opt / AotPrep に留める。
  - VM/LLVM の実行コアは Rust 側（Stage0 / NyRT）に委譲する。
- current selfhost authority entry is `stage1_cli_env.hako`; `launcher.hako` / raw subcmd lane は authority ではなく compat/future retire target として扱う。
- shell-side exact env transport lives in `tools/selfhost/lib/stage1_contract.sh`; `tools/selfhost/run_stage1_cli.sh` is a compatibility wrapper around that contract, not a second authority route.
- shared env/source resolution contract is isolated in `Stage1InputContractBox` inside `stage1_cli_env.hako`; keep input shaping out of `Main` and out of authority/compat boxes.
- reduced-artifact mode/env resolution is isolated in `Stage1ModeContractBox` inside `stage1_cli_env.hako`; keep `Main.main()` as a pure dispatcher over the exact `stage1-env-program` / `stage1-env-mir-source` contract.
- `Stage1InputContractBox` now also centralizes env/debug stringify behind `_coerce_text_compat(...)`, `_env_flag_enabled(...)`, and `_stage1_debug_on(...)`, so source/program-json resolution no longer repeats raw `"" + x` checks inline.
- emit-program authority is isolated in `Stage1SourceProgramAuthorityBox` inside `stage1_cli_env.hako`; keep source-to-Program(JSON) orchestration out of `Main`.
- `Stage1SourceProgramAuthorityBox` now centralizes source text coercion, same-file using-prefix merge, and the checked `BuildBox.emit_program_json_v0(...)` handoff behind owner-local helpers, so source-to-Program(JSON) orchestration no longer repeats raw `"" + x` conversions inline.
- materialized Program(JSON) validation is isolated in `Stage1ProgramResultValidationBox` inside `stage1_cli_env.hako`; keep emit-program on the same thin-dispatch pattern as emit-mir.
- `Stage1ProgramResultValidationBox`, `Stage1ProgramJsonTextGuardBox`, and `Stage1SourceMirAuthorityBox` now also centralize their checked text coercion through owner-local `_coerce_text_compat(...)`, so the remaining raw `"" + x` residue in `stage1_cli_env.hako` is limited to helper implementations and intentional JSON string assembly.
- shared Program(JSON) text-presence guard is isolated in `Stage1ProgramJsonTextGuardBox` inside `stage1_cli_env.hako`; keep source authority and explicit compat keep on one same-file fail-fast leaf for non-empty Program(JSON) input.
- shared Program(JSON) -> MIR checked call is isolated in `Stage1ProgramJsonMirCallerBox` inside `stage1_cli_env.hako`; keep the direct `MirBuilderBox.emit_from_program_json_v0(...)` contract out of both source authority and compat keep, and keep the helper itself on the same checked split (`_coerce_program_json_text_checked(...)` -> `_emit_mir_from_program_json_text_checked(...)`) as the other runner owners.
- source-only `emit-mir` authority call is isolated in `Stage1SourceMirAuthorityBox` inside `stage1_cli_env.hako`; it owns the checked `MirBuilderBox.emit_from_source_v0(...)` handoff for the exact `stage1-env-mir-source` route.
- `Stage1SourceProgramAuthorityBox` and `Stage1ProgramResultValidationBox` together now own the exact `stage1-env-program` route, while `Stage1SourceMirAuthorityBox` remains the MIR authority leaf and `Stage1ProgramJsonCompatBox` stays compat-only.
- the raw/subcmd `stage1_cli.hako` keep now also owns its emit-program checked tail behind `_emit_program_json_raw_with_debug(...)`, `_fail_emit_program_json_null(...)`, and `_coerce_program_json_output_checked(...)`, so the future-retire raw lane no longer mixes BuildBox call, null fail-fast, and Program(JSON) validation inline.
- the raw/subcmd `stage1_cli.hako` keep now also owns shared Program(JSON) marker predicates behind `_program_json_text_present(...)` and `_program_json_has_markers(...)`, so inline Program(JSON) payload detection, emit-program validation, and emit-mir validation reuse the same same-file check.
- the raw/subcmd `stage1_cli.hako` keep now also owns checked emit+output MIR handoff behind `_emit_validated_mir_from_program_json_text(...)` and `_mir_output_has_functions(...)`, so `_emit_mir_json_from_program_json_checked(...)` is down to Program(JSON) coercion -> checked emit handoff only.
- shared MIR materialization/validation is isolated across `Stage1MirResultValidationBox` and same-file `Stage1MirPayloadContractBox` inside `stage1_cli_env.hako`; keep result checking out of Main and out of the compat box.
- `Stage1MirPayloadContractBox` now owns MIR text materialization, structural payload predicates, and the final print/fail tail, so `Stage1MirResultValidationBox` is down to selected-input debug -> materialize/debug -> checked payload handoff only.
- `Stage1MirResultValidationBox` now also centralizes debug/materialized stringify behind `_coerce_text_compat(...)`, so result-kind/is-empty trace output no longer repeats raw `"" + x` conversions inline.
- explicit Program(JSON) compat keep is quarantined in `Stage1ProgramJsonCompatBox` inside `stage1_cli_env.hako`; current callers are probe/helper-owned only, so keep it outside reduced authority evidence and reuse the shared `Stage1ProgramJsonMirCallerBox` contract slice-by-slice.
- `Stage1ProgramJsonCompatBox` now also keeps explicit Program(JSON) input coercion behind `_coerce_program_json_text_checked(...)`, which itself reuses `Stage1ProgramJsonTextGuardBox.coerce_text_checked(...)`, so the compat lane no longer mixes input validation with the shared Program(JSON)->MIR caller handoff.
- `Stage1ProgramJsonCompatBox` now also keeps mixed-source fail-fast behind `_has_explicit_program_json_text(...)` and `_fail_mixed_source_mode(...)`, so the compat lane no longer mixes the predicate with the fail tag.
- `Stage1ProgramJsonCompatBox` now also keeps the final explicit Program(JSON) checked emit behind `_emit_mir_from_text_checked(...)`, so the public compat entry is down to input coercion -> checked emit handoff only.
- `launcher.hako` now also keeps shared Program(JSON) marker predicates behind `_program_json_text_present(...)` and `_program_json_has_markers(...)`, so emit-program output validation and emit-mir input validation reuse the same same-file check there too.
- `launcher.hako` now also keeps build-exe compile/link fail-fast tails behind `_compile_object_from_mir_path_checked(...)` and `_link_exe_object_checked(...)`, so `_emit_exe_from_mir_json_checked(...)` is down to path resolve/write -> compile -> link orchestration only.
- current compile-safe `launcher.hako` build-exe route now stops at direct `LlvmBackendBox.{compile_obj,link_exe}` calls instead of a quoted module-string backend literal; compiled-stage1 surrogate residue remains a temporary proof keep behind kernel module-string dispatch, not a visible launcher caller owner.
- compiled launcher helper defs still carry file-I/O methods, so the native keep is a narrow llvm-py by-name fallback for `FileBox.{open,read,readBytes,write,writeBytes,close}` only; this is a compile-safe keep for helper defs, not a generic fallback policy.
- Fail-Fast 原則:
  - 未実装コマンドや不正な引数は明示的なメッセージ＋非0終了コードで返す。
  - 暗黙のフォールバックや静かな無視は行わない。
