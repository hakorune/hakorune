# Hako Check — Diagnostics Contract (MVP)

This tool lints .hako sources and emits diagnostics.

Quick entry (toolbox index):
- `docs/tools/README.md`

Canonical helpers
- `bash tools/hako_check/run_tests.sh`
- `bash tools/hako_check/deadcode_smoke.sh`
- `bash tools/hako_check/deadblocks_smoke.sh`
- top-level `tools/hako_check_deadcode_smoke.sh` remains a compatibility shim only

Execution lane
- `hako_check` no longer treats explicit `--backend vm` as its canonical runtime.
- The CLI/scripts should run through the normal `hakorune` ingress (mainline/default route) and keep backend choice out of the tool surface unless a dedicated product-lane proof is being debugged.
- Product/native LLVM proof is a separate concern. Keep `hako_check` docs/tests focused on the analyzer contract first; do not re-pin legacy VM just to make the wrapper run.

Diagnostics schema (typed)
- Map fields:
  - `rule`: string like "HC011"
  - `message`: string (human-readable, one line)
  - `file`: string (path)
  - `line`: int (1-based)
  - `severity`: string ("error"|"warning"|"info"), optional (default: warning)
  - `quickFix`: string, optional

Backwards compatibility
- Rules may still `out.push("[HCxxx] ...")` with a single-line string.
- The CLI accepts both forms. String diagnostics are converted to typed internally.

Suppression policy
- HC012 (dead static box) takes precedence over HC011 (unreachable method).
- If a box is reported by HC012, HC011 diagnostics for methods in that box are suppressed at aggregation.

Quiet / JSON output
- When `--format json-lsp` is used, output is pure JSON (pretty). Combine with `NYASH_JSON_ONLY=1` in the runner to avoid extra lines.
- Note: some runtimes still print plugin/deprecation banners to stdout/stderr; `tools/hako_check/run_tests.sh` filters these banners before JSON extraction for stable diffs.
- Non-JSON formats print human-readable lines per finding.

Planned AST metadata (parser_core.hako)
- `boxes[].span_line`: starting line of the `static box` declaration.
- `methods[].arity`: parameter count as an integer.
- `boxes[].is_static`: boolean.

Notes
- Prefer AST intake; text scans are a minimal fallback.
- TextOps utilities are restricted-loop only (no recursion, no nested loops, no continue; step at end).
- TextOps is the SSOT for common text scans (split/trim/CSV/alias). Avoid re-implementing helpers in rules; add/extend in TextOps instead.
- For tests, use `bash tools/hako_check/run_tests.sh` (run_tests.sh is invoked via bash for consistency).

Restricted-loop policy (generic loop v0.2)
- No nested loops.
- No continue in loop body.
- Step is either at the tail, or a single in-body step that is safe to normalize (no loop-var use after it).

Analyzer policy (plugins)
- Tests/CI/Analyzer run without plugins by default: `NYASH_DISABLE_PLUGINS=1` and `NYASH_JSON_ONLY=1`.
- File I/O is avoided by passing source text via `--source-file <path> <text>`.
- When plugins are needed (dev/prod), set `NYASH_FILEBOX_MODE=auto` and provide [libraries] in nyash.toml.

Performance / MIR cache
- `tools/hako_check.sh` may reuse the existing L1 MIR cache (`tools/cache/phase29x_l1_mir_cache.sh`) before falling back to the normal emit route.
- Goal: repeated directory runs (especially selfhost trees) should skip redundant MIR emission for unchanged files while keeping analyzer behavior unchanged.
- Default operation is cache-first, emit-second:
  1. try L1 MIR cache
  2. if cache lookup/build fails, fall back to the existing `emit_mir_route.sh` path
- The wrapper may also memoize an `emit-failed` marker for the same cache key so repeated runs do not keep paying the same failed MIR emit cost for unchanged inputs.
- Control knobs:
  - `HAKO_CHECK_MIR_CACHE=0` disables the cache fast path
  - `HAKO_CHECK_MIR_CACHE_PROFILE` overrides the cache profile label
  - `HAKO_CHECK_MIR_CACHE_BACKEND` overrides the cache backend label
  - `HAKO_CHECK_MIR_CACHE_TARGET` overrides the cache target label
  - `HAKO_CHECK_MIR_CACHE_ROOT` overrides the cache root path
- Contract:
  - cache use must be conservative and behavior-preserving
  - cache failure must not silently drop MIR-dependent rules; it must fall back to the existing emit route
  - an `emit-failed` marker is advisory only and must remain key-scoped (source/profile/toolchain changes naturally invalidate it)

Default test env (recommended)
- `NYASH_DISABLE_PLUGINS=1` – avoid dynamic plugin path and noise
- `NYASH_BOX_FACTORY_POLICY=builtin_first` – prefer builtin/ring‑1 for stability
- `NYASH_USE_NY_COMPILER=0` – disable inline compiler in tests
- `NYASH_JSON_ONLY=1` – stdout is pure JSON (logs go to stderr)

## Known Limitations

### HC020: Dead Block Detection Producer Coverage

**Status**: consumer-side CFG handoff is wired; live producer coverage is still shape-dependent

**What is green now**:
- `deadblocks_smoke.sh` proves the HC020 consumer/rule contract with a prebuilt MIR JSON fixture that already contains `cfg.functions[*].blocks[*].reachable`.
- The wrapper now accepts `--dead-blocks` without mis-parsing it as a file path.

**What may still lag**:
- Some live `.hako` fixtures do not currently emit dead blocks in the active producer lane, so wrapper-driven HC020 runs may legitimately produce no findings even though the consumer path is working.

### HC017: Non-ASCII Quotes Detection (Temporarily Skipped)

**Status**: ⏸️ Skipped until UTF-8 support is available

**Reason**: This rule requires UTF-8 byte-level manipulation to detect smart quotes (" " ' ') in source code. Nyash currently lacks:
- Byte array access for UTF-8 encoded strings
- UTF-8 sequence detection capabilities (e.g., detecting 0xE2 0x80 0x9C for ")
- Unicode character property inspection methods

**Technical Requirements**: One of the following implementations is needed:
- Implement `ByteArrayBox` with UTF-8 encoding/decoding methods (`to_bytes()`, `from_bytes()`)
- Add built-in Unicode character property methods to `StringBox` (e.g., `is_ascii()`, `char_code_at()`)
- Provide low-level byte access methods like `string.get_byte(index)` or `string.byte_length()`

**Re-enable Timeline**: Planned for **Phase 22** (Unicode Support Phase) or when ByteArrayBox lands

**Test Files**:
- [`tests/HC017_non_ascii_quotes/ng.hako`](tests/HC017_non_ascii_quotes/ng.hako) - Contains intentional smart quotes for detection testing
- [`tests/HC017_non_ascii_quotes/ok.hako`](tests/HC017_non_ascii_quotes/ok.hako) - Clean code without smart quotes (baseline)
- [`tests/HC017_non_ascii_quotes/expected.json`](tests/HC017_non_ascii_quotes/expected.json) - Empty diagnostics (reflects disabled state)

**Implementation File**: [`rules/rule_non_ascii_quotes.hako`](rules/rule_non_ascii_quotes.hako) - Currently returns 0 (disabled) in `_has_fancy_quote()`

**Current Workaround**: The test is automatically skipped in `run_tests.sh` to prevent CI failures until UTF-8 support is implemented.

---

Rules
- Core implemented (green): HC011 Dead Methods, HC012 Dead Static Box, HC013 Duplicate Method, HC014 Missing Entrypoint, HC015 Arity Mismatch, HC016 Unused Alias, HC018 Top‑level local, HC021 Analyzer IO Safety, HC022 Stage‑3 Gate, HC031 Brace Heuristics
- Temporarily skipped: HC017 Non‑ASCII Quotes (UTF-8 support required)
- Opt-in: HC032 Restricted Loop (nested loop/continue/step tail) — run via `--rules restricted_loop`

CLI options
- `--rules a,b,c` limit execution to selected rules.
- `--skip-rules a,b` skip selected.
- `--no-ast` (default) avoids AST parser; `--force-ast` enables AST path (use sparingly while PHI is under polish).

Tips
- JSON-only output: set `NYASH_JSON_ONLY=1` to avoid log noise in stdout; diagnostics go to stdout, logs to stderr.
- For multiline `--source-file` payloads, CLI also provides HEX-escaped JSON in `NYASH_SCRIPT_ARGS_HEX_JSON` for robust transport; the VM prefers HEX→JSON→ARGV.
