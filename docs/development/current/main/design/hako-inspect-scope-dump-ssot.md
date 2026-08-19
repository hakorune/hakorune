# Hako Inspect Scope Dump SSOT

Status: SSOT
Scope: source anchors, `hako_check inspect`, MIR / LLVM IR / assembly dump
boundaries, and AI-readable inspect artifacts.

## Current execution brief

Decision: Open one design stop to select an observation-only normalized
MIR/LLVM/assembly shape report; `hako_check inspect` remains a renderer and
`tools/perf` remains the C/reference and measurement owner.
Source authority + canonical issuer: The exact source hash, emitted MIR JSON,
final LLVM artifact, executable symbol, and compiler-emitted mapping metadata
own their respective facts. The D0 may only join already-issued artifacts from
one candidate.
Non-authority: ValueId, MIR adjacency, labels, helper/symbol names, instruction
counts, assembly diffs, C code, and timing ratios cannot issue source meaning,
route selection, a keeper, provenance, or promotion.
Fail-fast boundary: The D0 must name candidate/function/symbol identity,
artifact digests, mapping-quality floor, optional external C input, and a
counterexample where symbol-level similarity cannot justify an exact edge.
Smallest next slice: `HAKO-INSPECT-LOWERING-SHAPE-REPORT-D0` performs a
read-only owner/contract census and accepts one six-line brief before any code.
Non-claims: No report implementation, C build/measurement, provenance emission,
compiler/backend edit, keeper selection, benchmark verdict, or production
change during the design stop.

## Decision

Scope-wide MIR / LLVM IR / assembly dump is a tool query, not a `.hako`
source command.

Do not add source syntax such as:

```hako
__output__MIR__ {
    local a
    a = a + 1
}
```

The source may contain optional observation anchors, but the dump itself belongs
to `hako_check` / compiler tooling.

## Contract

- `.hako` source keeps ordinary program meaning.
- `__mir__.mark(label)` and `__mir__.log(label, ...)` are point observations.
- Scope dump is an external inspect operation.
- The compiler may preserve source spans, debug markers, route metadata, and
  diagnostic region metadata.
- `hako_check` renders emitted artifacts and metadata only.
- `hako_check` must not select routes, infer optimizer truth, or claim keeper
  evidence from helper names.
- Optimization truth remains `Plan` / `RouteDecision` / verifier evidence /
  report keys.
- Current implementation slice: MIR / report / LLVM / ASM bundles are live.
  Mapping quality is explicit: source→MIR exact, MIR→LLVM block, LLVM→ASM
  symbol-level evidence by default.

## Source Surface

Preferred selectors:

```hako
// hako:inspect begin alloc_fastpath
local a
a = a + 1
// hako:inspect end alloc_fastpath
```

Point observations:

```hako
__mir__.mark("alloc_fastpath.enter")
__mir__.log("a_after", a)
```

`__mir__.mark` / `__mir__.log` may become MIR debug instructions and can affect
hot-path shape. For performance keeper evidence, prefer CLI span selectors or
comment anchors over debug instructions.

## Tool Surface

Initial CLI shape:

```bash
bash tools/hako_check.sh inspect scope \
  --span src/hako_alloc.hako:120:145 \
  --emit mir,mir-json,report \
  --out target/hako-inspect/alloc_fastpath
```

Optional comment-anchor selector:

```bash
bash tools/hako_check.sh inspect scope \
  --region alloc_fastpath \
  --emit mir,mir-json,llvm,asm,report \
  --format bundle \
  --out target/hako-inspect/alloc_fastpath
```

Route-focused selector:

```bash
bash tools/hako_check.sh inspect route \
  --selected-route hako.typed_object.slot_load_i64 \
  --emit mir,asm,report
```

Mark-focused selector:

```bash
bash tools/hako_check.sh inspect mark \
  --label alloc_fastpath.enter \
  --window 12 \
  --emit mir
```

## Bundle Contract

Default output directory:

```text
target/hako-inspect/<region_id>/
  manifest.json
  source.slice.hako
  source.map.json
  mir.raw.txt
  mir.raw.json
  mir.planned.txt
  mir.planned.json
  route_decisions.json
  verifier.json
  llvm.ir
  asm.s
  asm.map.json
  report.kv
  summary.md
```

Required report keys:

```text
output_contract=hako-check-inspect-scope-v0
tool_surface=hako_check_inspect_scope
observation_only=1
rewrite_executed=0
keeper_selection=0
source_file=<path>
source_hash=<sha256>
selector_kind=span|comment_anchor|mark|route|function
region_id=<id>
function=<function-or-empty>
backend=<backend>
emit_mir=0|1
emit_llvm=0|1
emit_asm=0|1
source_to_mir_mapping=exact|block|function|missing
mir_to_llvm_mapping=exact|block|function|missing
llvm_to_asm_mapping=exact|block|symbol|missing
selected_route_count=<n>
compat_helper_call_count=<n>
runtime_helper_call_count=<n>
debug_instruction_count=<n>
summary=ok|fail
```

Assembly mapping must include a quality label. Optimized assembly may move,
inline, merge, or delete code; tooling must not pretend a source region has an
exact assembly slice when the backend can only provide symbol-level evidence.

## Fail-Fast Rules

- `source_hash_mismatch` fails.
- `region_not_found` fails.
- `region_ambiguous` fails.
- requested MIR artifacts missing fails.
- requested assembly unavailable fails unless the command explicitly permits
  unavailable assembly.
- mapping quality below `--require-mapping` fails.
- `--require-selected-route <route>` fails when the selected route is absent.

## Task Ladder

- `INSPECT-000` through `INSPECT-006`: landed source/MIR/backend bundle,
  route/mark queries, and report-key diff surface.
- `HAKO-INSPECT-SCOPE-OWNER-SPLIT-I0` (**landed BoxShape**): the former
  916-line owner is a 692-line CLI/effect facade plus one 245-line pure
  metadata/report child. The reusable guard runs all five focused tests,
  rejects duplicate/effect-bearing model ownership, and enforces 760 lines.
- `HAKO-INSPECT-LOWERING-SHAPE-REPORT-D0` (**selected design stop**): fix the smallest
  normalized MIR/LLVM/assembly vocabulary, same-candidate identity, optional C
  reference input, mapping-quality floor, and one S6C counterexample. The D0
  must not add compiler provenance or infer an edge from IDs/names.
- `HAKO-INSPECT-LOWERING-SHAPE-REPORT-I0` (**parked behind D0**): one
  evidence-only report shows MIR blocks/edges/PHIs/calls, LLVM
  blocks/branches/calls/loads, and selected-symbol assembly
  instructions/branches/calls beside an externally supplied C artifact. A thin
  `tools/perf` wrapper may build the reference; hako_check remains the renderer.
- `HAKO-INSPECT-PROVENANCE-D0` (**conditional**): open only if block/symbol
  quality cannot identify the residual owner. Any exact MIR-edge-to-assembly
  mapping must be compiler/backend-emitted sidecar evidence; no ValueId,
  adjacency, label, or symbol-name reconstruction is allowed.

## Non-Goals

- no `__output__MIR__` / `__output__ASM__` source syntax.
- no optimizer decisions from comment anchors.
- no route inference from helper symbol names.
- no keeper or winner claim from `hako_check inspect`.
