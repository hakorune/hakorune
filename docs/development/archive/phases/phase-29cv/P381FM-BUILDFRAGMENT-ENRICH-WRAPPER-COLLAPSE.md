# P381FM BuildProgramFragment Enrich Wrapper Collapse

Date: 2026-05-06
Scope: remove `BuildProgramFragmentBox.enrich/2` from the public BuildBox
authority path so the live blocker chain names the concrete enrichment owners.

## Context

After P381FL, the public blocker chain still contained:

```text
BuildBox.emit_program_json_v0/2
  -> BuildBox._parse_program_json/2
  -> BuildProgramFragmentBox.enrich/2
```

`BuildProgramFragmentBox.enrich/2` is an orchestration wrapper over the concrete
defs / enum / imports injectors. Keeping that extra hop hid the actual remaining
owners.

## Change

`BuildBox.emit_program_json_v0/2` now calls the concrete injectors directly:

```text
BuildProgramFragmentBox._inject_defs_json(...)
BuildProgramFragmentBox._inject_enum_decls_json(...)
BuildProgramFragmentBox._inject_imports_json(...)
```

with the same freeze checks preserved between steps.

## Result

The live blocker chain no longer stops at `BuildProgramFragmentBox.enrich/2`.
The next probe can report the concrete enrichment owners instead of the wrapper
box.

This is a wrapper-collapse slice only. It does not add new Stage0 semantics or a
new lowering contract.
