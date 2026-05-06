# P381FR BuildBundle Public Seam Cleanup

Date: 2026-05-06
Scope: remove the last BuildBundle-side dependency on the private
`BuildBox._emit_program_json_from_scan_src` wrapper and prune the now-dead scan-src
wrapper helpers from `BuildBox`.

## Why

`BuildBox.emit_program_json_v0/2` is the public source-only authority seam.
After the wrapper-collapse cards through P381FQ, `BuildBundleFacadeBox` was still
calling the private helper:

```text
BuildBundleFacadeBox.emit_program_json_v0(...)
  -> BuildBox._emit_program_json_from_scan_src(scan_src)
  -> BuildBox.emit_program_json_v0(scan_src, null)
```

That left a private wrapper in the active bundle path even though the public seam
already had the correct null-opts contract.

## Change

- `BuildBundleFacadeBox.emit_program_json_v0(...)` now calls
  `BuildBox.emit_program_json_v0(scan_src, null)` directly.
- Removed dead private helpers from `BuildBox`:
  - `_emit_program_json_from_scan_src/1`
  - `_parse_program_json_from_scan_src/1`

## Result

- Bundle-aware Stage-B entry now hands off to the same public BuildBox seam as the
  non-bundle path.
- The remaining parse-side stop is the real private owner
  `BuildBox._parse_program_json/2`, not a scan-src wrapper.
- No Stage0-visible parser contract was added.
