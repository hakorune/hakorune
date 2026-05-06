# P381FO Imports Inject Wrapper Collapse

Date: 2026-05-06
Scope: collapse the `_inject_imports_json/2` wrapper so the live blocker names
the imports-source owner directly instead of `_build_imports_fragment_json/1`
and `_inject_imports_fragment_if_present/2`.

## Context

After P381FN, the smallest concrete enrichment-side blocker was:

```text
BuildProgramFragmentBox._inject_imports_json/2
  -> BuildProgramFragmentBox._build_imports_fragment_json/1
     blocker: ParserCommonUtilsBox.esc_json/1
  -> BuildProgramFragmentBox._inject_imports_fragment_if_present/2
```

Both child calls were orchestration helpers around the real imports-source work.

## Change

`BuildProgramFragmentBox._inject_imports_json/2` now inlines:

- `UsingCollectorBox.collect(scan_src)`
- `convert_usings_to_imports(usings_json)`
- `inject_json_fragment(ast_json, ",\"imports\":" + imports_obj)`

and keeps the same empty/`{}` fast exits.

## Result

The imports-side stop is now closer to the real owner. The next probe should
name `UsingCollectorBox.collect/1` directly instead of the old wrapper pair.

This is still a BoxShape cleanup slice only:

- no new Stage0 semantics
- no new lowering contract
- just one less orchestration hop in the active blocker chain
