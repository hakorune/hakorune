---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: classify Rust/public Program(JSON v0) delete-last surfaces before code removal.
Related:
  - docs/development/current/main/design/json-v0-route-map-ssot.md
  - src/cli/args.rs
  - src/runner/emit.rs
  - src/runner/stage1_bridge/program_json_entry/
  - src/runner/stage1_bridge/program_json/
  - src/stage1/program_json_v0.rs
  - src/runner/json_artifact/program_json_v0_loader.rs
  - src/runtime/deprecations.rs
---

# P66 Rust Public Delete-Last Surface Split

## Goal

Keep the final Rust/public Program(JSON v0) cleanup from mixing two different
compat surfaces:

- public emit: `--emit-program-json-v0`
- umbrella intake: `--json-file` with Program(JSON v0) payloads

## Decision

- `--emit-program-json-v0` is still parsed in `src/cli/args.rs` and exits
  through `src/runner/emit.rs` -> `stage1_bridge/program_json_entry/`
- bridge-local read/emit/write implementation for that flag lives in
  `src/runner/stage1_bridge/program_json/`
- payload emission for that flag delegates into
  `src/stage1/program_json_v0.rs::emit_program_json_v0_for_stage1_bridge_emit_program_json(...)`
- `src/stage1/program_json_v0.rs` also serves non-CLI Rust callers such as
  strict authority source emit and BuildBox/module-string bootstrap support
- `--json-file` Program(JSON v0) intake is a separate surface owned by
  `src/runner/json_artifact/program_json_v0_loader.rs`
- `HAKO_PROGRAM_JSON_FILE` / `HAKO_PROGRAM_JSON` is another Program(JSON v0)
  fixture transport used before runner setup in `src/main.rs`
- deprecation warnings in `src/runtime/deprecations.rs` stay live while either
  public compat surface remains

## Delete Order

1. Remove shell/tool callers of `--emit-program-json-v0`.
2. Then remove the `stage1_bridge/program_json_entry/` and
   `stage1_bridge/program_json/` emit-flag clusters.
3. Keep `src/stage1/program_json_v0.rs` until strict authority,
   `host_providers/mir_builder` handoff, BuildBox/module-string bootstrap, and
   stage1 bridge payload callers are replaced.
4. Keep `src/runner/json_artifact/program_json_v0_loader.rs` until
   `--json-file` Program(JSON v0) intake callers are replaced or archived.
5. Remove `HAKO_PROGRAM_JSON_FILE` transport only after fixture smokes no
   longer use Program(JSON v0) file handoff into `.hako` MirBuilder entries.

## Non-goals

- do not delete Rust code in this card
- do not remove `--json-file`
- do not treat `--json-file` MIR(JSON) intake as Program(JSON v0) debt

## Acceptance

```bash
rg --fixed-strings "emit-program-json-v0" src/cli/args.rs src/runner/emit.rs src/runner/stage1_bridge src/runtime/deprecations.rs
rg --fixed-strings "emit_program_json_v0_for_stage1_bridge_emit_program_json" src/stage1/program_json_v0.rs src/runner/stage1_bridge
rg --fixed-strings "emit_program_json_v0_for_current_stage1_build_box_mode" src crates
rg --fixed-strings "HAKO_PROGRAM_JSON_FILE" src tools/smokes/v2/profiles/integration/joinir
rg --fixed-strings "load_program_json_v0_to_module" src/runner/json_artifact
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
