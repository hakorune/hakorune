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
