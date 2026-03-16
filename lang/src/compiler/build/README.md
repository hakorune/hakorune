Hakorune BuildBox (Skeleton)

Purpose
- Provide a Box-first boundary for compiling Hako sources in-process.
- Responsibility (initial):
  - Accept source string and build options
  - Build `scan_src` from the full source for defs/imports scanning
  - Build `parse_src` from `Main.main` body when available, and only then fall back to full-source parse
  - Emit Program JSON v0 and return as string
  - Validate header (version/kind)
  - (Future) call Bridge → MIR and (optional) ny-llvmc for EXE/O/Others

Interface (proposal)
- static box BuildBox {
  - method emit_program_json_v0(src, opts) -> String
  - method verify_program_json_v0(json) -> Bool
  - method plan() -> String  // returns a short description of enabled stages
}

Error policy
- Fail‑Fast: missing header / malformed JSON returns a tagged error string
  (e.g. "[build/json/header] …").

Notes
- Keep the box thin and stable. Heavy lifting (resolver/bridge/ny-llvmc) stays behind
  dedicated boxes to preserve a clean boundary and testability.
- Current shape:
  - `scan_src`: full merged source, used for `FuncScannerBox` / `UsingCollectorBox`
  - `parse_src`: `BodyExtractionBox.extract_main_body(scan_src)` when available, else `scan_src`
  - owner-local helper split:
    - `_prepare_scan_src(...)`: bundle/env normalization plus `BundleResolver.resolve(...)`
    - `_new_prepare_scan_src_result(...)` / `_fail_prepare_scan_src(...)` / `_apply_prepare_scan_src_result(...)` / `_resolve_prepare_scan_src_if_needed(...)`: prepared-scan-src result/error/resolve handoff only
    - `_bundle_inputs_requested(...)` / `_resolve_scan_src_from_bundle_ctx(...)`: bundle resolve decision plus merged `scan_src` materialization only
    - `_ensure_bundle_alias_arrays(...)` / `_ensure_require_mods_array(...)`: bundle ctx container setup only
    - `_fail_bundle_ctx(...)`: bundle opts validation error handoff only
    - `_collect_named_bundle_inputs(...)`: named-bundle pair validation and assign only
    - `_parse_program_json(...)`: parser entry only
    - `_main_body_parse_src_if_present(...)`: parse-src narrowing helper only
    - `_emit_program_json_from_scan_src(...)`: outer producer sequencing only
    - `_parse_program_json_from_scan_src(...)`: parse-source narrowing plus parser call only
    - `_inject_stageb_fragments_json(...)`: defs/imports enrichment tail only
    - `_build_defs_fragment_json(...)`: defs-scan plus defs-fragment build only
    - `_inject_defs_json(...)` / `_inject_imports_json(...)`: Stage-B fragment injection only
    - `_defs_scan_enabled(...)` / `_inject_defs_fragment_if_present(...)`: defs gate and inject tail only
    - `_build_imports_fragment_json(...)` / `_inject_imports_fragment_if_present(...)`: imports-fragment build and inject tail only
