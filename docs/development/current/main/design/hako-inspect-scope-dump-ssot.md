# Hako Inspect Scope Dump SSOT

Status: SSOT
Scope: source anchors, `hako_check inspect`, MIR / LLVM IR / assembly dump
boundaries, and AI-readable inspect artifacts.

## Current execution brief

Decision: Bind the exact bytes of
`apps/tests/scan_with_init_typed_ok_min.hako` to the returned `MirFunction`
inside the existing producer, then publish those carried bytes and strict JSON
as one private observation cohort.
Source authority + canonical issuer: The existing caller-zero S6C compiler
test parses and transforms those exact `include_str!` bytes, owns the installed
source/Facts/Recipe cohort, verified unpublished `MirFunction`, and strict JSON
serialization. The test export is the sole observation-envelope issuer.
Non-authority: Test names, filenames, manual copies, current target paths,
separately reopened source paths, ordinals, nonces, `ny_main`, ValueId,
assembly similarity, and C/reference artifacts cannot issue source/candidate
lineage. The path is diagnostic; carried bytes and digests are binding.
Fail-fast boundary: Producer-copied source and JSON must match the write-last
producer manifest; the existing structural driver consumes those exact JSON
bytes once and emits one nonempty final module and object; disassembly is
regenerated from that object. Foreign/stale digest, selector, module, object,
projected meso artifact, or mapping downgrade issues no V1 seal.
Smallest next slice: `HAKO-INSPECT-S6C-OBSERVATION-INGRESS-I0` adds one
test-only carrier for `{source_path, source_bytes, function}`, one tiny producer
call, and one dedicated Python adapter that owns a private transaction, reuses
the structural driver, accepts the object machine carrier, and writes the V1
seal last.
Non-claims: No new semantic `Verified*`/`Prepared*` receipt,
Facts/Recipe issuance, compiler/backend behavior, C/reference owner,
measurement, residual owner, keeper, promotion, or production change.

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

Sealed lowering-shape comparison:

```bash
bash tools/hako_check.sh inspect shape \
  --bundle target/hako-inspect/loop \
  --c-asm target/hako-inspect/reference/c-loop.objdump.txt \
  --c-symbol c_loop \
  --out target/hako-inspect/loop-shape
```

`--c-asm` and `--c-symbol` are optional but must appear together. `hako_check`
does not build the C reference; it validates its exact symbol, records its
digest, and labels it `external_reference_only`.

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

The historical `manifest.json` is a V0 observation manifest. It does not bind
the copied MIR/LLVM/executable/assembly bytes strongly enough for cross-layer
shape inspection and is not a candidate-lineage authority.

The selected V1 slice adds one seal written only after validation:

```text
target/hako-inspect/<region_id>/
  executable.bin              # when backend artifacts were requested
  identity.json               # hako-inspect-bundle-identity-v1, written last
  shape/shape.json            # hako-lowering-shape-report-v0
  shape/report.kv
  shape/summary.md
```

`identity.json` binds the digest of every available artifact. A backend-ready
seal additionally requires explicit, unique MIR function, LLVM function, and
assembly symbol selectors. The candidate seal is derived from the canonical
identity payload; callers cannot provide it. Partial MIR-only bundles may bind
the artifacts they contain, but they are not shape-ready.

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
- a V1 backend-ready seal fails when any MIR/LLVM/assembly selector is omitted,
  missing, or ambiguous.
- a V1 seal never falls back to `ny_main`, `main`, or the first assembly label.
- a copied executable/disassembly digest mismatch fails before `identity.json`
  publication.

## Lowering Shape Boundary

After the identity seal lands, `hako_check inspect shape` may render independent
normalized columns for MIR, LLVM, selected-symbol assembly, and an optional
externally supplied C assembly artifact. Its minimum mapping floor is:

```text
source -> MIR   exact
MIR -> LLVM     block
LLVM -> ASM     symbol
```

This is sufficient for counts and localizing a suspicious layer, but not for
an instruction correspondence graph. For example, S6C direct-continuation may
delete the Bool merge and rebranch, merge blocks, or redirect PHI predecessors.
Two artifacts may therefore have similar branch counts inside the same symbol
while no assembly branch has a sound one-to-one MIR-edge identity. The report
must print `cross_layer_correspondence=unclaimed`, `keeper_selection=0`, and
`measurement_authority=0`.

## Task Ladder

- `INSPECT-000` through `INSPECT-006`: landed source/MIR/backend bundle,
  route/mark queries, and report-key diff surface.
- `HAKO-INSPECT-SCOPE-OWNER-SPLIT-I0` (**landed BoxShape**): the former
  916-line owner is a 692-line CLI/effect facade plus one 245-line pure
  metadata/report child. The reusable guard runs all five focused tests,
  rejects duplicate/effect-bearing model ownership, and enforces 760 lines.
- `HAKO-INSPECT-LOWERING-SHAPE-REPORT-D0` (**accepted**): the current bundle
  cannot yet support the report because V0 lacks MIR/LLVM/executable/assembly
  digests and exact selectors. Symbol/name fallback cannot issue lineage.
- `HAKO-INSPECT-BUNDLE-IDENTITY-SEAL-I0` (**landed BoxCount**): one V1
  bundle seal, explicit selectors, executable copy, artifact digests, a derived
  candidate identity, fail-fast negatives, and guard coverage are live. The
  entry/model/identity owners are 745/245/172 lines; seven focused tests and the
  reusable guard are green. Report vocabulary and compiler behavior stay
  unchanged.
- `HAKO-INSPECT-LOWERING-SHAPE-REPORT-I0` (**landed BoxCount**): one
  evidence-only report shows MIR blocks/edges/PHIs/calls, LLVM
  blocks/branches/calls/loads, and selected-symbol assembly
  instructions/branches/calls beside an externally supplied C artifact. A thin
  `tools/perf` wrapper may build the reference; hako_check remains the renderer.
  The entry/model/identity/shape-cli/shape-model owners are
  753/245/172/107/207 lines. Eleven focused tests, the reusable owner guard,
  command help, and pointer guard are green.
- `HAKO-INSPECT-S6C-OBSERVATION-INGRESS-D0` (**accepted**): the canonical seam
  is the existing `pinned_text_real_candidate_json_preserves_carrier_lineage`
  export after verified function serialization. Its carrier retains the exact
  parsed fixture bytes with the function; reopened paths, target directories,
  test names, ordinals, nonces, and historical perf artifacts are non-authority.
- `HAKO-INSPECT-S6C-OBSERVATION-INGRESS-I0` (**selected BoxCount**): add the
  test-only envelope and one dedicated private-transaction adapter over the
  existing structural driver. Accept one exact object carrier into V1 without
  editing the 753-line generic inspect entry or the 754-line trace script.
- `HAKO-INSPECT-S6C-SHAPE-CANARY-R0` (**parked behind ingress D0**): produce
  one real sealed S6C bundle and render its MIR/LLVM/ASM counts beside the
  existing independent C assembly artifact.
- `HAKO-INSPECT-PROVENANCE-D0` (**conditional**): open only if block/symbol
  quality cannot identify the residual owner. Any exact MIR-edge-to-assembly
  mapping must be compiler/backend-emitted sidecar evidence; no ValueId,
  adjacency, label, or symbol-name reconstruction is allowed.

## Non-Goals

- no `__output__MIR__` / `__output__ASM__` source syntax.
- no optimizer decisions from comment anchors.
- no route inference from helper symbol names.
- no keeper or winner claim from `hako_check inspect`.
