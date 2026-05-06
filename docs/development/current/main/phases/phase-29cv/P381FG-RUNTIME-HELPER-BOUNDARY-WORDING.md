# P381FG Runtime Helper Boundary Wording

Date: 2026-05-06
Scope: align Stage1 Program(JSON) helper comments/docs with the current runtime-boundary ownership model.

## Context

After P381FD/P381FE/P381FF, the BuildBox Stage1 Program(JSON) path is no longer
owned by Stage0 rewrites or parser-body lowering. The remaining Rust-side
surrogate path is a narrow runtime helper boundary.

Some kernel-side comments and README files still described that path primarily as
`compat quarantine`, which understated the now-explicit runtime-helper role.

## Change

Updated the kernel-side wording around:

- `crates/nyash_kernel/src/exports/stage1.rs`
- `crates/nyash_kernel/src/plugin/module_string_dispatch.rs`
- `crates/nyash_kernel/src/plugin/module_string_dispatch/build_surrogate.rs`
- `crates/nyash_kernel/src/plugin/README.md`
- `crates/nyash_kernel/src/plugin/module_string_dispatch/README.md`

The new wording keeps the same constraints:

- shrink-only
- not a semantic owner
- authority remains in `nyash_rust::stage1::program_json_v0`

but names the surface more precisely as a narrow runtime-boundary residue bucket.

## Verification

Commands:

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

The remaining Stage1 helper bridge is now documented consistently with the
current design: a thin runtime-boundary residue path, not a new owner and not a
reason to widen Stage0 semantics again.
