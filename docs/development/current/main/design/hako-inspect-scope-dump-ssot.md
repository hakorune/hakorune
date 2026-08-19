# Hako Inspect Scope Dump SSOT

Status: SSOT
Scope: source anchors, `hako_check inspect`, MIR / LLVM IR / assembly dump
boundaries, and AI-readable inspect artifacts.

## Current execution brief

Decision: Expose the landed selected-Dynamic lowered-provenance product through
one thin hako_check command without changing its evidence boundary.
Source authority + canonical issuer: The landed atomic ingress and its exact
source/no-clone MIR plus emitter-issued lowered-LLVM sidecar remain sole owners.
Non-authority: CLI spelling, wrapper temporary paths, compiler names, displayed
counts, final LLVM, ASM, timings, and command success issue no new meaning.
Fail-fast boundary: The command delegates once to the existing ingress; driver
build or ingress failure publishes no output directory or partial artifact.
Smallest next slice: `HAKO-INSPECT-SELECTED-DYNAMIC-PROVENANCE-UX-I0` adds one
dedicated child dispatch while leaving the 753-line generic router unchanged.
Non-claims: No new provenance relation, final-LLVM/ASM mapping, keeper,
measurement, production, fallback, retry, or performance claim.

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

For the selected S6C canary, `lowering.provenance.json` raises only the
MIR→LLVM floor to `issuer_exact`. Its rows are emitted at the selected lowering
site and exhaustively cover the sealed candidate's 8 MIR blocks/8 edges and 22
final LLVM blocks/31 edges. The renderer groups those rows by MIR origin while
keeping `LLVM→ASM correspondence: unavailable`; it never reconstructs an edge
from equal labels, adjacency, ValueIds, counts, or disassembly.

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
- `HAKO-INSPECT-S6C-OBSERVATION-INGRESS-I0` (**landed BoxCount**): the
  test-only envelope and dedicated private-transaction adapter carry exact
  source bytes, strict JSON, final LLVM, the object carrier, and object-derived
  disassembly into one V1 seal. Foreign/stale/projected artifacts reject before
  publication; the 753-line generic inspect entry and 754-line trace script
  remain untouched.
- `HAKO-INSPECT-S6C-SHAPE-CANARY-R0` (**landed observation row**): the first
  seal exposed two counter defects subsequently repaired. The current census is
  MIR `8 blocks / 8 edges / 24 instructions`, final LLVM `22 / 31 / 82`, and
  selected-symbol assembly `69 instructions / 13 branches / 3 calls / 2 returns`.
- `HAKO-INSPECT-PROVENANCE-D0` (**accepted**): first provenance product is an
  issuer-emitted, candidate-local MIR→final-LLVM block/edge relation. It natively
  represents preserved/split/merged/deleted/introduced sets. LLVM→machine
  remains unavailable because optimization may fold, merge, delete, or create
  instructions without a carried machine origin.
- `HAKO-INSPECT-MIR-CFG-COUNT-I0` (**landed BoxShape**): teach only the pure
  shape model that `checked_callout` and `pinned_text_residence_enter` each own
  two canonical successors. Add exact positive/negative counters and update the
  canary to MIR `8 blocks / 8 edges / 5 branches / 24 instructions`; no sidecar.
- `HAKO-INSPECT-LLVM-FUNCTION-SLICE-I0` (**landed BoxShape**): replace the
  multiline-consuming `^\s*define` selector with horizontal whitespace only.
  Reject ambiguous/missing functions as before and prove leading blank lines do
  not create an implicit LLVM block or instruction.
- `HAKO-INSPECT-PROVENANCE-MIR-LLVM-I0` (**landed BoxCount**): one
  compile-time observation sink in the selected dispatch/lowerer plus a new
  private relation model/validator. Direct `bbN`, WidthAt/ScalarEq internal
  regions, consumed direct-continuation branch arms, and lifecycle edges must
  be issued while lowering; the ingress only validates, digest-binds, and
  renders them. The real canary closes 8/8 and 22/31 through 53 rows, the
  generic lowerer remains 759 lines, the generic inspect entry remains 753,
  and ASM exactness is explicitly unavailable. Twenty-two focused inspect
  tests plus structural/preflight/pointer guards are green.
- `HAKO-INSPECT-PROVENANCE-GENERALIZATION-D0` (**accepted**): the second
  consumer is the source-backed selected Dynamic
  `ParserScanLoopBox.skip_while/4`. Selected admission promotes that helper to
  `program.entry`, so its real owner is the generic active walker plus the C1
  checked-callout emitter—not the same-module body emitter. Ordinary simple
  loop fixtures currently stop earlier at `PhysicalHeader::CompletionNotValue`
  and cannot be used as provenance consumers without mixing another BoxCount.
- `HAKO-INSPECT-SELECTED-DYNAMIC-WALK-SPLIT-I0` (**landed BoxShape**): move
  the existing lines 516–700 active block/instruction walk verbatim from the
  759-line generic lowering owner into one private child include. Parent and
  child must remain below 760, while emitted LLVM bytes, object bytes, symbols,
  failure tags, routes, and accepted shapes remain identical. The landed owners
  are 575/185 lines; the extracted child SHA equals the old line range and the
  parent/current preprocessed C hashes are identical. No journal or schema was
  added. The focused Dynamic and S6C smokes are green. The broad
  `dynamic_v2_aot_activation_authority_guard.sh` is classified baseline-red at
  parent `fdb04a6cdd` with `selected package adapter must consume ... once`.
- `HAKO-INSPECT-SELECTED-DYNAMIC-PROVENANCE-D0` (**accepted**): the production
  default uses external `opt`/`llc`; its post-opt CFG has no carried MIR origin.
  The optional legacy CAPI `ModuleRef` is an alternate test route and may not
  become authority. Therefore the exact product stops at the pre-opt lowered
  LLVM bytes emitted by the generic walker plus C1 owner. Post-opt LLVM and ASM
  remain unmapped rather than receiving inferred origins.
- `HAKO-INSPECT-SELECTED-DYNAMIC-LOWERED-LLVM-PROVENANCE-I0` (**landed
  caller-zero BoxCount**): one exact source constant drives parsing and
  publication; the collector moves the sole completed draft without cloning.
  The generic active walker and actual C1 CallOut/End emitters issue 64 unique
  rows covering MIR 10 blocks/10 edges and lowered-pre-opt LLVM 32 blocks/32
  edges. Staged source/MIR digests are rechecked, conflicting ownership and
  incomplete coverage reject, and forced journal failure leaves neither named
  output nor `/tmp/hako_pure_gen_<pid>.ll`. Post-opt and machine mapping remain
  unavailable.
- `HAKO-INSPECT-SELECTED-DYNAMIC-PROVENANCE-UX-I0` (**selected BoxShape**):
  route `hako_check inspect selected-dynamic-provenance --out ...` directly to
  one dedicated child which builds the private driver and invokes the landed
  atomic ingress. Do not edit the 753-line generic inspect router or add a
  second validator/renderer.
- `SELECTED-DYNAMIC-C1-PHI-PREDECESSOR-PROJECTION-D0` (**accepted physical
  BoxCount**): the first real full-source provenance canary exposed an invalid
  pre-opt module. C1 expands the loop backedge through
  `c1_end_continue_4_2` and the second normal landing through
  `c1_validate_normal_1_6_1`, but PHIs still name `%bb4` and `%bb6` as their
  incoming predecessors. One block-tail row cannot represent C1 normal and
  fault exits from the same MIR block. The C1 physical CFG preflight therefore
  issues an exact edge-tail inventory keyed by `(MIR pred, MIR successor)`;
  emission co-checks it and the existing PHI writer consumes it. Provenance,
  label formatting, and post-hoc LLVM scans may not issue correctness.
- `SELECTED-DYNAMIC-C1-PHI-PREDECESSOR-PROJECTION-I0` (**landed BoxCount**):
  add the pre-effect edge-tail inventory without growing the 575-line generic
  parent. Positive acceptance requires `%r4` from
  `%c1_end_continue_4_2`, `%r15` from
  `%c1_validate_normal_1_6_1`, exact normal/fault/end rows, and LLVM18
  parse/verify plus external opt green. Missing, duplicate, conflicting,
  dangling, late, or emitter-drift rows reject before output; a projected edge
  has no `%bb<pred>` fallback. The focused fixture covers the CallOut-normal
  and End-backedge PHIs plus dangling-target rejection; the real full-source
  candidate passes LLVM18 verification and emits an ELF object. Source/MIR,
  C1 meaning, provenance, production, retry, and performance stay unchanged.
- `NORMAL-CALLABLE-PHYSICAL-HEADER-ROW-SPARSE-COHORT-D0` (**accepted
  BoxCount design**): preserve one package owner but make header availability
  row-local. An annotated exact-I64 callable with its own complete parameter
  contract and verified value Completion may issue one row; unannotated
  siblings issue no row and may not erase eligible rows. Unsupported explicit
  annotations remain rejected. The unchanged full parser scan source must lend
  exactly the `skip_while` row with two Completion sites and no rows for its
  three unannotated siblings. A-prime's header requirement remains unchanged.
  The final product is one always-present, possibly empty sparse cohort; only
  `cohort.row(batch_slot)` returns `Option`. Package-wide
  `missing_parameter_contract`, `missing_result_annotation`, and the outer
  `Option<Cohort>` are retired together so no second availability authority
  remains. S6C seed consumption remains prior and affine.
- `NORMAL-CALLABLE-PHYSICAL-HEADER-ROW-SPARSE-COHORT-I0` (**landed
  BoxCount**): implement the accepted product shape in completion seed,
  physical header, package model/issuer/install, and focused tests. Positive:
  unchanged full parser scan source lends exactly one exact-I64 `skip_while`
  row with two explicit Completion sites and the existing A-prime demand
  succeeds. Negative: unannotated single/mixed rows lend no header; unsupported
  explicit annotations and malformed covered rows still reject. No source,
  Dynamic catalog header, A-prime validator, Builder, or provenance changes.
  The package owns one always-present cohort, package-wide missing bits are
  gone, and only row lookup remains optional. All 35 package tests plus two
  selected-emitter tests are green. The reusable complete-batch guard's new
  sparse-cohort assertions are green, but the whole guard remains classified
  parent-baseline-red at `e3adb2de8e` on its unchanged selected-mapping
  vocabulary regex.
- `HAKO-INSPECT-LLVM-ASM-D0` (**conditional NoSafeSlice**): open only if exact
  machine attribution remains necessary. LLVM18 block-name assembly comments
  are useful diagnostics but are not exact object-address provenance.

## Non-Goals

- no `__output__MIR__` / `__output__ASM__` source syntax.
- no optimizer decisions from comment anchors.
- no route inference from helper symbol names.
- no keeper or winner claim from `hako_check inspect`.
