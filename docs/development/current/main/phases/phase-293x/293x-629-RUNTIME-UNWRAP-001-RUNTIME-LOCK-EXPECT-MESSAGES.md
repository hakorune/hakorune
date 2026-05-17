# 293x-629 RUNTIME-UNWRAP-001 Runtime Lock Expect Messages

Status: landed
Date: 2026-05-18

## Decision

`RUNTIME-UNWRAP-001` is the next BoxShape cleanup row selected by
`MIMAP-124A`.

Claude's source-structure review identified production `runtime/` lock
`unwrap()` calls as a small cleanup candidate. This row keeps behavior unchanged
but replaces focused lock / global-registry `unwrap()` calls with explicit
`expect(...)` messages so poison or initialization panics point to the owning
runtime subsystem.

## Scope

- Update focused production runtime lock / registry `unwrap()` calls in:

```text
src/runtime/box_registry.rs
src/runtime/plugin_loader_unified.rs
src/runtime/unified_registry.rs
```

- Use stable, plain expect messages.
- Leave tests and broad non-runtime unwrap cleanup for later rows.

## Stop Lines

- No poison recovery policy in this row.
- No behavior change beyond panic message clarity.
- No allocator behavior.
- No provider activation.
- No plugin ABI change.
- No source syntax.
- No backend route change.
- No broad unwrap sweep.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `RU1.1` | Replace focused runtime lock/global unwraps with expect messages. | build passes; no API change. | no recovery policy |
| `RU1.2` | Run focused verification. | cargo build and pointer guard pass. | no broad test bundle |
| `RU1.3` | Close row and select next task. | current pointers are updated. | no behavior bundle |

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
cargo build --release --bin hakorune
git diff --check
```

## Landed Result

- Replaced focused production runtime lock/global-registry `unwrap()` calls
  with explicit `expect(...)` messages in the selected files.
- Left test unwraps and broad unwrap cleanup out of scope.
- Selected `WASM-LOG-001`.

Observed evidence:

```text
bash tools/checks/current_state_pointer_guard.sh
cargo build --release --bin hakorune
git diff --check
```
