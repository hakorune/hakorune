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
  - method emit_program_json_v0(src, null) -> String
  - method verify_program_json_v0(json) -> Bool
  - method plan() -> String  // returns a short description of enabled stages
}
- static box BuildBundleFacadeBox {
  - method emit_program_json_v0(src, opts) -> String
}

Error policy
- Fail‑Fast: missing header / malformed JSON returns a tagged error string
  (e.g. "[build/json/header] …").

Notes
- Keep the box thin and stable. Heavy lifting (resolver/bridge/ny-llvmc) stays behind
  dedicated boxes to preserve a clean boundary and testability.
- `BuildBox` is the source-only source-to-Program(JSON v0) authority. It does
  not import bundle collector/resolver boxes, and it fail-fast rejects non-null
  opts or bundle env inputs instead of silently ignoring them.
- Live Stage-B bundle entry uses `BuildBundleFacadeBox` as a thin bundle-aware
  adapter. The facade prepares merged scan source, then delegates to `BuildBox`
  for Program(JSON v0) emission.
- `lang/src/compiler/entry/bundle_resolver.hako` is a legacy compat and JoinIR
  fixture surface, not the live BuildBox dependency.
- `BuildBundleResolverBox` owns live bundle duplicate/require validation and
  merged-prefix materialization for BuildBox.
- `BuildBundleInputBox` owns live bundle opts/env input collection, alias-table
  parsing, require CSV parsing, and bundle-input presence checks for BuildBox.
- `BuildProgramFragmentBox` owns live defs/imports fragment construction,
  using-to-imports conversion, and Program(JSON v0) fragment injection.
- Defs enrichment consumes `FuncScannerBox.collect_defs_fragment_json(source)`
  as a scanner-owned text fragment seam; the public Build path must not consume
  `FuncScannerBox.scan_all_boxes(source)` object records.
- Defs method-body parsing uses the existing `ParserBox.parse_program2` seam;
  the public Build path must not reintroduce `parse_block2` result stripping.
- The legacy object-defs JSON builders were removed from this box after the
  text-fragment seam became the only public defs enrichment path.
- `BodyExtractionBox` owns parse-source narrowing from wrapped `Main.main`
  sources to the method body.
- Current shape:
  - `scan_src`: full merged source, observed by `BuildProgramFragmentBox`
  - `parse_src`: `BodyExtractionBox.extract_main_body(scan_src)` when available, else `scan_src`
  - owner-local helper split:
    - `BuildBundleFacadeBox._prepare_scan_src(...)`: bundle input collector plus resolver handoff only
    - `BuildBundleFacadeBox._new_prepare_scan_src_result(...)` / `BuildBundleFacadeBox._fail_prepare_scan_src(...)` / `BuildBundleFacadeBox._apply_prepare_scan_src_result(...)` / `BuildBundleFacadeBox._resolve_prepare_scan_src_if_needed(...)`: prepared-scan-src result/error/resolve handoff only
    - `BuildBundleFacadeBox._resolve_scan_src_from_bundle_ctx(...)`: `BuildBundleResolverBox` call only
    - `BuildBox.emit_program_json_v0(...)`: public source-only producer seam
    - `_parse_program_json(...)`: parser entry only
    - `_resolve_parse_src(...)`: `BodyExtractionBox` parse-source narrowing handoff plus source-text fallback only
    - `_coerce_text_compat(...)`: fallback source-text materialization only
    - `BuildProgramFragmentBox._inject_defs_json(...)` / `_inject_enum_decls_json(...)` / `_inject_imports_json(...)`: direct fragment owner paths
