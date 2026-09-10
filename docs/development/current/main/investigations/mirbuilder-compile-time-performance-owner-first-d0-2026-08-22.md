---
Status: snapshot D0/I0, emit-clone P0, lazy payload P0, postprocess walk census D0/P0, variable-read accessor S0, PHI analysis batch I0, LocalSSA self-cache reuse I0, C invocation-driver arity repair R0 closed, and LocalSSA guard path repair R0 active; lookup-facade S0 is ParkedSealed__NoExclusiveDeleteSet; prepared-operand D0 remains deferred
Task: MIR-COMPILE-TIME-PERF-OWNER-FIRST-D0
Date: 2026-09-02
Priority: measure compiler-time fixed costs before changing the canonical MIR spine
Parent: MIRBUILDER-FINAL-PIPELINE-v1
NextCard: MIR-LOCAL-SSA-GUARD-PATH-REPAIR-R0
---

## `MIR-LOCAL-SSA-GUARD-PATH-REPAIR-R0` fast row (2026-09-10)

Decision: repair only the registered COPY-UNKNOWN0 guard after the LocalSSA
materialization split. The guard must count each production consumer in its
landed owner file (`local/materialize.rs`, `local/post_success.rs`, and
`local/copy_type.rs`) while retaining the existing `local.rs` entry checks.

Source authority + canonical issuer: the existing LocalSSA split and its
current guard contract. Non-authority: guard text as a semantic owner,
`variable_map`, PHI/Loop state, a new index, or any new proof receipt.

Fail-fast boundary: `python3 tools/checks/lib/mirbuilder_copy_unknown_authority_guard.py .`
must pass with the current split. Acceptance is path-aware guard success and
`git diff --check`; no LocalSSA behavior, fallback, or production route changes.
The exclusive delete-set is stale path/count assumptions in this guard only.

## `MIR-C-INVOCATION-DRIVER-ARITY-REPAIR-R0` closeout (2026-09-10)

Decision: repair the three focused C test drivers to call the existing
four-argument `hako_llvmc_invocation_init` owner. The fourth argument is the
driver's existing ingress profile: `STATIC_V2` for the static V2 driver and
`GENERIC_COMPAT` for the compatibility/query drivers.

Source authority + canonical issuer: the declaration and existing production
callers in `lang/c-abi/shims`; the test drivers only exercise that private
invocation owner. Non-authority: Recipe, Loop, SSA, MIR meaning, runtime ABI,
and production compiler routing.

Fail-fast boundary: each focused driver must compile against the four-argument
declaration. Acceptance is compile success for
`static_v2_execution_driver.c`, `allocation_config_capture_driver.c`, and
`named_query_driver.c`, with no runtime or ABI semantic change. The exclusive
delete-set is the three stale three-argument call sites; no new dispatcher,
fallback, receipt, or guard is introduced.

This row is a mechanical prerequisite before selecting the queued Loop PHI
consumer rows. It does not claim Loop production reachability, OBJ/EXE
execution, or a compiler-speed improvement.

Implementation evidence: `d5658dc759` passes direct compilation of all three
focused drivers with the existing C/yyjson sources. The static V2 driver uses
`HAKO_LLVMC_INGRESS_STATIC_V2`; the allocation-capture and named-query drivers
use `HAKO_LLVMC_INGRESS_GENERIC_COMPAT`. No other C source or invocation
semantics changed.

# MIRBuilder compile-time performance owner-first D0

## Six-line brief

```text
Decision: accept the feedback as a bounded performance queue, not as a speed claim. The first three observations are code-backed opportunities; their frequency and 2–4x estimate remain unmeasured. Do not open pass fusion, cache redesign, or broad cleanup in this card.
Source authority + canonical issuer: the compiler invocation and its existing phase owners are the measurement authority; a future observation-only compile-time baseline issuer may co-seal source digest, binary/profile, environment snapshot, warm/cold mode, phase counters, and wall-clock samples. It must not issue semantic Facts, Recipe, Join, physical, or publication meaning.
Non-authority: worker estimates, generated-program runtime speed, benchmark names, cargo build time alone, source-line counts, one local wall-clock sample, or a guessed number of env reads. Generic env OnceLock is not an authority until process/test configuration lifetime is fixed.
Fail-fast boundary: measure before every optimization edit; reject a proposed speedup if the same source digest, compiler artifact, route, environment, correctness gate, and sample protocol are not preserved, or if a semantic owner/fallback route changes.
Smallest next slice: MIR-COMPILE-TIME-PERF-BASELINE-P0 — collect a repeatable compile-only baseline and a hot-owner census for env reads, emit clones, lazy debug arguments, and refresh/shadow waves. Then open one P0 per confirmed owner.
Non-claims: no 2–4x speed claim, no pass integration, no cache/differential compilation, no parser/resolver redesign, no production route switch, no backend/runtime performance claim, and no generic env helper rewrite.
```

## Postprocess walk census D0 decision (2026-09-10)

The existing `semantic_refresh` and `run_postprocess_stages` owners remain the
authority for refresh order. The audit found these production caller classes:

```text
normal/canonical compiler finish
raw published Script/App finish
early declaration/layout subset
JSON-v0 bridge subset
rune immediate refresh
```

The function-local refresh wave currently exposes 59 direct refresh call sites
(62 when the string-corridor helper's internal calls are expanded), followed
by 13 post-fixpoint consumer calls per function. These numbers describe the
source call graph only; they are not a count of block or instruction visits.
The current timing hooks expose five coarse semantic stages and route-fixpoint
iteration/family counters, but no actual function/block/instruction visit
counts, read/write mutation class, or later-consumer usage of intermediate
products.

Decision: preserve the existing stage order and shared postprocess kernel and
open one observation-only execution slice,
`MIR-SEMANTIC-REFRESH-WALK-COUNTERS-P0`, at the existing
`compile_timing` owner. The slice may report caller/family/stage,
function/block/instruction visits, metadata-only versus MIR-read/MIR-write
class, and whether a later stage consumes an intermediate product. It must not
fuse stages, add a cache, issue a semantic receipt, or change the production
route. A finite report with explicit boundary, includes/excludes, and stable
focused evidence is the acceptance; speedup and duplicate-walk claims remain
unproven until then.

### Observation counter P0 — 2026-09-10

The selected slice is implemented as an opt-in observation scope in the
existing `src/mir/compile_timing.rs` owner. `BasicBlock::all_spanned_instructions`
reports one block event and one instruction event per yielded instruction while
the scope is active; the existing function-owner loops report function events.
The scope is thread-local, restores any outer scope, and is completely
inactive when `NYASH_MIR_COMPILE_TRACE` is unset. No refresh helper, metadata
product, ordering, route, or cache was changed.

The scaling runner now parses the stable `[mir-compile/walk]` rows into its
existing machine-readable result. One 50-method probe with the accepted
`static_methods` shape produced the following observation (the compiler exited
successfully and route shadow parity remained zero):

| stage | access | intermediate consumer | functions | blocks | instructions |
| --- | --- | ---: | ---: | ---: | ---: |
| `layout_and_decl` | `metadata_read_write` | false | 0 | 0 | 0 |
| `all_functions` | `mir_read_write` | false | 51 | 153 | 306 |
| `route_convergence` | `mir_read_write` | true | 51 | 0 | 0 |
| `post_fixpoint` | `mir_read_write` | true | 51 | 153 | 306 |
| `contracts` | `mir_read_write` | true | 0 | 867 | 1734 |

The measured boundary is explicit: function counts cover the existing
function-owner loops, block counts cover active canonical spanned-instruction
walks, and instruction counts cover yielded items from those iterators. Direct
field iteration outside those owners, module declaration-only work, and route
family-internal recomputation accounting remain outside these counters and are
not inferred from zeroes. The default-off probe emitted no timing or walk rows.

Focused evidence:

```text
CARGO_BUILD_JOBS=4 cargo check --profile quick -p nyash-rust --lib  # passed
CARGO_BUILD_JOBS=4 cargo test --profile quick --lib semantic_refresh # 10 passed
python3 -m unittest tools.perf.test_mir_compile_scaling              # 7 passed
python3 tools/perf/mir_compile_scaling.py --bin target/quick/hakorune \
  --profile-label quick --method-counts 50 --warmup-runs 0 --repeat-runs 1 # rc=0
```

This closes the observation counter slice only. It does not claim a speedup,
complete whole-repository walk census, pass fusion, cache safety, or a
production route change. Any fusion proposal must still prove the missing
direct-iteration boundary and semantic dependency equivalence.

The callable Loop findings are a separate correctness queue. They must close
PHI generation binding and local completion publication before a source-bound
Loop normalizer/physical consumer is selected, but they do not change this
performance observation owner while the Loop source edge remains caller-zero.

## What the feedback confirms locally

The following are source-backed observations in the current branch:

| Observation | Evidence | Confidence | Treatment |
| --- | --- | --- | --- |
| Generic env helpers read the process environment on every call | `src/config/env.rs:249-324`; `env_bool`, `env_present`, and `env_string` call `std::env::var` directly | high | measure hot call counts; do not add a global cache yet |
| `emit_instruction` has unconditional clone candidates | `src/mir/builder/builder_emit.rs:52-57` clones an unused function name; `:263` clones the current name; `:368-373` clones the instruction before append | high | isolate as one behavior-preserving hot-path P0 |
| Debug resolve metadata is built before the hub gate | `src/mir/builder/calls/unified_emitter.rs:258-300` performs candidate lookup and prepares the event; `src/debug/hub.rs:20-40` checks env gates only after receiving `serde_json::Value` | high | add a hub-owned permit/lazy-argument design task |
| Refresh orchestration has a function-local wave and later post-fixpoint consumers | `src/mir/semantic_refresh.rs:203-220` and `:339-363` are separate passes; `builder_metadata.rs:66-83` also refreshes rune plans at setters | medium | census exact duplicate work before fusion |
| Script shadow has multiple observation entrypoints | `src/mir/resolved_semantics/shadow/entry.rs:118-172` and `:240-256` call the shared traversal with different products | medium | call-graph/site census; no automatic single-pass merge |
| AST deep clone, prelude double-read/strip, and exact 7–9 env reads per instruction | not established by this local source audit | unverified | keep as hypotheses until counters or a bounded trace proves them |

The expected speedup is therefore a prioritization hypothesis only. In
particular, no current evidence supports the statement “2–4x faster”.

## Authority map

| Owner | Owns | Must not own |
| --- | --- | --- |
| compile-time baseline harness | invocation timing, phase counters, source/binary/config identity | semantic acceptance or route selection |
| `src/config/env.rs` and flag modules | environment vocabulary and parsing policy | per-instruction hot-path policy without a lifetime decision |
| `builder_emit.rs` | ordinary instruction emission and the one append point | debug-event policy or semantic revalidation |
| `src/debug/hub.rs` | debug enable/kind/sink/sample gate | constructing caller-owned JSON before the gate |
| `src/mir/semantic_refresh.rs` | refresh-wave order and phase ownership | silently deleting a phase because it looks duplicated |
| resolver/shadow source owners | source observations and their products | performance-derived semantic merging |

The performance layer observes these owners. It does not become a second
source, resolver, Recipe, or physical authority.

## Finite task sequence

### P0 — `MIR-COMPILE-TIME-PERF-BASELINE-P0`

Before code changes, fix one compile-only protocol:

```text
same source digest
same prebuilt target/quick compiler binary digest
same backend and env snapshot
warm process and cold process reported separately
one warmup followed by five retained samples per case
median + p95, not one best sample
correctness/probe result recorded beside timing
```

Use a small source, a representative MIR-heavy source, and the existing
merged callable probe. External `/usr/bin/time` is acceptable for the first
observation; phase counters may be test-only or debug-off observation fields.
Do not use generated executable runtime time as compile-time evidence.

The existing runner is the only owner. It now emits schema
`mir-compile-scaling-v1`, pins generated probes to the accepted `static box
Main` root and `NYASH_DISABLE_PLUGINS=1`, records the compiler binary SHA-256,
source SHA-256, selected environment, and keeps one warmup plus five retained
subprocess observations. Shadow-route rows remain strict by default; a caller
must opt in to `--allow-missing-shadow-contract` when measuring a source that
does not expose those rows. The optional loop-bound probes are still allowed
to report a compile rejection; they are not silently counted as a timing
baseline.

Required counters/census rows:

```text
emit_instruction calls
env helper calls by key and owner
MirInstruction clone count at the append seam
resolve.try candidate lookup and JSON construction count
debug hub gate pass/reject count
refresh_function_semantic_metadata calls
post-fixpoint refresh calls
shadow traversal entrypoint/site counts
source read/parse count
```

Exit: a checked-in report or machine-readable observation with the protocol,
not a speed winner. If a proposed hotspot is not visible in the baseline,
park it.

### P0 observation — 2026-09-02

Command:

```text
python3 tools/perf/mir_compile_scaling.py \
  --bin target/quick/hakorune --profile-label quick \
  --warmup-runs 1 --repeat-runs 5
```

The compiler binary was rebuilt from the active HEAD before this run:

```text
path:   target/quick/hakorune
bytes:  40,799,544
sha256: e4ecbad221fb101fc5ac98149e961ed815b074c2fd37c01acfa387b430d335c0
env:    NYASH_DISABLE_PLUGINS=1, NYASH_MIR_COMPILE_TRACE=1
env fingerprint (NYASH_/HAKO_ keys): 7ce1910d777280fe211c8154b0bdbaecc0daf8bf842e71f24e81a7932663beed
shadow: strict contract passed for every retained sample
```

The generated `static_methods` probes all compiled successfully. Values are
wall-clock milliseconds for the five retained subprocesses; the warmup is
listed separately and is not included in the median or p95.

| shape | source SHA-256 | warmup ms | retained ms | median | p95 | `build_module` median | `semantic_refresh` median |
| --- | --- | ---: | --- | ---: | ---: | ---: | ---: |
| 50 methods | `c9002d7351c92caf7baa47d526bcbc423faa7f3747a4869cb4eeffc20d1c9d80` | 17 | 14, 14, 13, 14, 14 | 14 | 14 | 4 | 1 |
| 100 methods | `c0b894697aa7efa03680230785810ce612f1bec6362c664c32ee7520a6f225d6` | 29 | 54, 57, 65, 69, 70 | 65 | 70 | 40 | 3 |
| 250 methods | `0c4e579b50c912b12c9c827aee8cbb96a3ddfc473578f671c516dec69b8eb462` | 232 | 227, 205, 235, 231, 224 | 227 | 235 | 200 | 8 |

This is an observation, not a performance budget or a speedup claim. The
optional literal/dynamic loop-bound probes were also rechecked; both are
rejected before timing with the current `callable-loop/route-not-front-selected`
contract (`GenericLoopV0`/`GenericLoopV1` overlap). They remain a separately
parked route-coverage observation and are not converted into a false green
baseline.

The first measured hot owner is therefore the existing per-instruction
configuration/emit path. No clone or environment policy change is authorized
by this observation alone; the next row must still name the exact invocation
snapshot owner and preserve the existing environment matrix.

### P1 — `MIR-BUILDER-HOT-CONFIG-SNAPSHOT-P0`

Do not change generic `env_bool`/`env_string` to `OnceLock` directly. Tests use
scoped environment overrides, and a process-global cache would turn test
order into configuration authority. First choose one of these explicitly:

```text
immutable compiler-invocation snapshot created after CLI/config resolution
or
hot-key cache with an explicit test reset/override boundary
```

The snapshot may be borrowed by Builder sessions. It must preserve current
defaults, aliases, and debug-off behavior. Acceptance is equal flag values
under the existing environment matrix plus a reduced hot-call count; no
semantic route change.

The physical placement is also fixed. The snapshot is one field below the
existing compilation-owned context (`comp_ctx` or its existing config child),
not a new top-level `MirBuilder` axis. It must not add a field to
`RawInvocationChildPortV1`, add a dispatch-port supertrait, or be copied through
the Call forwarding stack. Emit code borrows the already-installed snapshot.
One context-owned config field may replace scattered process-env reads; the
number of cross-layer capability axes must not increase.

The measured current debug-OFF static counts are ordinary emit `7`, Copy `8`,
and Call `9` process-env reads. The later P0 closes only when the selected emit
boundary reaches zero while two separately-created compiler sessions can
still observe two explicit test configurations. The supplied `7-13` range is
not used as an acceptance value.

### P2 — `MIR-BUILDER-EMIT-CLONE-SHAPE-P0`

Remove only proven dead or unconditional clone work at the emit seam:

```text
unused `_dbg_fn_name` / `_dbg_region_id` computation
current function-name clone when no diagnostic path needs it
instruction clone for non-Phi instructions, if post-append observation is
  represented by a small precomputed observation rather than a second owner
```

Keep the sole physical append point. Preserve receiver materialization, PHI
completion/origin, predecessor updates, metadata, and error diagnostics.
Acceptance requires MIR output parity, focused emit tests, the compile-time
baseline, and a guard that prevents reintroducing the unconditional clone.

### Selected execution — `MIR-BUILDER-EMIT-CLONE-SHAPE-P0` (2026-09-10)

This fast slice is limited to two proven dead computations and the append
ownership shape in the existing `MirBuilder::emit_instruction` owner:

```text
remove `_dbg_fn_name` and `_dbg_region_id` (no production reader)
capture only the post-materialization Phi observation when the instruction is Phi
move every non-Phi instruction into the existing append point
```

The existing `MirInstruction` append remains the sole physical mutation. The
source/semantic authority, receiver materialization, strict checks, metadata
recording, predecessor updates, Phi completion/origin, and diagnostic order are
unchanged. No new helper, receipt, route, fallback, retry, or backend consumer
is permitted. The exclusive deletion set is the two dead locals and the
non-Phi `instruction.clone()` allocation; Phi observation keeps its existing
post-append behavior through a bounded local snapshot.

Acceptance is the existing Builder/Phi focused suite plus the compile-cost
baseline protocol, `builder_emit.rs` below 760/800 lines, no remaining dead
local anchors, one append call, `git diff --check`, and the current-state
pointer guard. No speedup or whole-library green claim follows from this row.

### P0 closeout — `9808923785` (2026-09-10)

The selected slice is closed. `MirBuilder::emit_instruction` now removes the
two unread debug metadata computations, moves the instruction through the
existing sole append point, and retains only the bounded `(dst, inputs)`
snapshot needed for the existing post-append Phi observation. Receiver
materialization, metadata, predecessor updates, Phi completion/origin, and
diagnostic ordering are unchanged. No semantic, route, fallback, retry, or
backend surface changed.

Evidence:

```text
CARGO_BUILD_JOBS=4 cargo check --profile quick -p nyash-rust --lib       # passed
CARGO_BUILD_JOBS=4 cargo test --profile quick --lib phi_type_publication  # 14 passed
CARGO_BUILD_JOBS=4 cargo test --profile quick --lib method_call_terminal  # 8 passed
python3 tools/perf/mir_compile_scaling.py --bin target/quick/hakorune \
  --profile-label quick --warmup-runs 1 --repeat-runs 5                 # all retained runs rc=0
bash tools/checks/current_state_pointer_guard.sh                         # passed
git diff --check                                                          # passed
```

The compile-cost observation remained a baseline only: the retained
`static_methods` medians were 14 ms (50), 65 ms (100), and 227 ms (250), with
shadow parity mismatches at zero. The touched `builder_emit.rs` is 548 lines,
has one `append_instruction_core` call, and has no `_dbg_fn_name`,
`_dbg_region_id`, or unconditional `instruction.clone()` anchor. This does
not claim a measured speedup or whole-library health.

The next performance row is the ordered
`MIR-EMIT-DEBUG-POLICY-SNAPSHOT-I0`; its D0 design is accepted using the
existing invocation/session owner and both production ingress forms. Lazy
payload construction is later in the queue. The unrelated C
invocation-driver arity, generic compatibility-positive fixture, and
path-aware LocalSSA guard findings remain separate queue items and are not
silently counted as resolved by this emit slice.

### P3 — `MIR-BUILDER-DEBUG-EVENT-LAZY-ARGS-P0`

Move the debug decision to the debug hub boundary without moving authority:

```text
hub permit for (category, kind, config snapshot)
  -> only then candidate lookup / Vec construction / json! construction
  -> one event append
```

The default-off path must not allocate candidate vectors, JSON values, or
timestamps. Debug-on output and sampling semantics must remain unchanged.
Do not add unconditional `eprintln!` or a second debug gate in every caller.

### D1 — `MIR-SEMANTIC-REFRESH-WAVE-CENSUS-D0`

Count exact function visits and facts read/written in the refresh order before
attempting fusion. A pass may be fused only when:

```text
the same semantic owner remains sole issuer
read/write dependencies are explicit
intermediate observations are not externally visible
the final product is byte/field-equivalent on focused fixtures
```

“50–70 walks” and “35–50 after fusion” are not acceptance numbers until this
census exists. The task must not delete a post-fixpoint refresh merely because
its name resembles an earlier pass.

### D2 — `MIR-SOURCE-OBSERVATION-SINGLE-PASS-D0`

Audit the two Script shadow entrypoints, AST clone sites, and prelude reads as
one source-observation map. Merge only rows that are genuinely the same fact
under the same parser/source authority. A single traversal producing two
explicit products is preferred to two traversals, but it must not create a
combined “default” product or leak Builder/Recipe meaning into the shadow
owner.

### Future — `MIR-COMPILE-TIME-BUDGET-I0`

Open only after three baseline snapshots exist. Set a budget by workload class
(small, representative, selfhost-sized), record variance and machine/profile,
and require a regression gate. This is a compiler-time gate, separate from
generated-code runtime perf gates and from the existing phase-137x observe-only
runtime lane.

## Pass integration decision

Do not integrate passes as a single optimization wave now. The safe order is:

```text
baseline/census
  -> hot config snapshot (if measured)
  -> emit clone shape (if measured)
  -> lazy debug args (if measured)
  -> refresh fusion only with dependency proof
  -> source single-pass only with source-authority proof
```

Each row is one responsibility, one focused gate, and one reversible commit.
No row may change semantic acceptance, fallback, Resolver/Recipe ownership, or
the active callable ordinary-bridge design stop.

## NoSafeSlice conditions

Stop and return to design if any proposal requires:

```text
global OnceLock over env values that tests can mutate
moving a debug gate into a semantic caller and duplicating policy
combining refresh products without a dependency/co-seal proof
using runtime benchmark speed to claim compiler speed
changing source observation count without proving same authority/product
pass fusion that crosses the active canonical/legacy boundary
```

## Current status

The former Hako SameModuleInstance physical-ingress lane remains
`ParkedSealed__HakoIngressMissing`: no existing Hako LLVM-text consumer
borrows `PublishedMirBackendView`, and no family-exclusive caller/delete set
was found. `MIR-COMPILE-COST-BASELINE-P0` completed on 2026-09-02, and the
snapshot I0 plus its post-audit gate repairs are landed. B-S0 then added direct
negative evidence for the existing publication/view validators. The optional
loop-bound probes remain a separate route-coverage rejection and are not a
false green.

The lazy resolve payload slice and the observation-only postprocess walk
counter slice are now landed. Keep the Hako published-view ingress parked. The
debug-policy snapshot is already closed at `4ba9293900`; do not reopen its D0
wording or add a second Builder, adapter, fallback, or semantic receipt.

### `MIR-BUILDER-VARIABLE-READ-ACCESSOR-S0` selection

```text
Decision:
  Replace only the selected direct variable read in variable_read.rs with the
  existing VariableContext::lookup owner. Preserve behavior and keep the
  physical variable map unchanged.
Source authority + canonical issuer:
  Existing FunctionLoweringStateV1::variable_ctx / VariableContext::lookup.
Non-authority:
  PHI/Loop state, assignment writers, snapshots, map clones, and broad
  variable_map visibility cleanup.
Fail-fast boundary:
  __pin$ rejection, escape checks, debug observation, and undefined-variable
  diagnostics remain in build_variable_access with no fallback or repair.
Smallest next slice:
  One direct-read replacement plus focused variable-read parity and a census
  proving zero direct variable_map.get calls remain in variable_read.rs.
Non-claims:
  No PHI/Loop change, assignment change, snapshot change, whole-map
  privatization, or compiler-speed claim.
```

Closeout at `968f135fce`:

```text
CARGO_BUILD_JOBS=4 cargo check --profile quick -p nyash-rust --lib       # passed
CARGO_BUILD_JOBS=4 cargo test --profile quick --lib normal_callable_loop_handoff # 6 passed
rustfmt --edition 2021 --check src/mir/builder/variable_read.rs          # passed
rg -n "variable_map\.get" src/mir/builder/variable_read.rs              # zero matches
bash tools/checks/current_state_pointer_guard.sh + git diff --check      # passed
```

The selected direct read now uses the existing `VariableContext::lookup`
owner. `__pin$` rejection, escape checks, debug observation, and undefined
variable diagnostics remain in the same method. No PHI/Loop, assignment,
snapshot, whole-map, or compiler-speed claim is made.

### `MIR-CALL-EMIT-LOOKUP-FACADE-RETIRE-S0` design stop

The next row is not an immediate deletion. The worker census found that the
facade owns three distinct policy cases: unified-profile lookup present,
lookup-present legacy retry rejection, and lookup-absent compatibility. Its
real callers include the method-call terminal's global and standard paths plus
one header-port test; raw invocation ports can produce both `Some` and `None`.

```text
Decision:
  Audit whether one finite, contract-preserving delete-set exists. Keep the
  facade if deleting it would duplicate or bypass the existing policy.
Source authority + canonical issuer:
  MirBuilder::emit_unified_call_with_lookup owns lookup policy; the typed
  UnifiedCallEmitter consumes it.
Non-authority:
  caller frame count, a new port/receipt, name repair, fallback, retry, or
  semantic target re-resolution.
Fail-fast boundary:
  lookup-present must retain the legacy-retry rejection; lookup-absent may
  retain only the existing compatibility path. Any proposed deletion must
  preserve both terminals and their diagnostics.
Smallest next slice:
  read-only caller/consumer/policy census, then either one exclusive delete-set
  or an explicit ParkedSealed decision.
Non-claims:
  no code change, new port, receipt, backend route, or semantic Call change.
```

### Lookup-facade census closeout (2026-09-10)

The read-only census is complete. The policy facade has three distinct
contracts and no contract-preserving exclusive delete-set:

```text
lookup = Some:
  unified-call profile is selected and a legacy retry is rejected

lookup = None:
  the existing compatibility path remains permitted

caller surface:
  global and standard method terminals, plus one header-port test;
  raw invocation ports can produce either Some or None
```

`MirBuilder::emit_unified_call_with_lookup` remains the sole owner of these
policy decisions. `UnifiedCallEmitterBox` is the typed consumer. Deleting the
facade would either duplicate the profile/retry policy at callers or bypass
the lookup-present rejection. Direct test calls to the lower-level implementation
are not a production delete-set and do not justify changing the public seam.

Decision: `ParkedSealed__NoExclusiveDeleteSet`. Keep the facade unchanged until
a future caller census proves one finite delete-set that preserves both
`Some` and `None` terminals, diagnostics, recursion behavior, and compatibility
ownership. No code, new port, receipt, fallback, retry, or semantic Call route
was opened by this census.

The next bounded design stop is `MIR-PHI-ANALYSIS-BATCH-D0`; it is a separate
production-reachable PHI analysis concern and must name its mutation boundary
before any cache or repair deletion is considered.

### `MIR-PHI-ANALYSIS-BATCH-D0` design stop (2026-09-10)

```text
Decision:
  Reuse one immutable PHI analysis batch for each mutation-stable function
  phase; do not cache across a function mutation or change PHI semantics.
Source authority + canonical issuer:
  Existing PhiInputMaterializationAnalysis::new(func) owns CFG,
  predecessor/reachability, definition-block, and dominator facts. The
  materialization and finalization consumers borrow that batch.
Non-authority:
  for_pred, complete_missing_self_carried_phi_inputs, module lifecycle,
  individual PHI emitters, variable_map, and type hints. They may consume or
  update physical PHI inputs, but cannot reissue analysis facts.
Fail-fast boundary:
  add/remove/replace blocks or terminators, successor changes, parameter or
  entry changes, and add/remove/replace of an analyzed ValueId definition
  invalidate the batch. Existing PHI-input updates and append-only
  rematerialization are allowed only while the analyzed topology/definitions
  remain unchanged; a new definition requires rebuilding before use.
Smallest next slice:
  At materialize_all_phi_inputs, build the batch once after its preconditions,
  then pass the same batch to self-carry completion and grouped-edge
  materialization. Audit root finalize versus all-function finalize so one
  root is not repaired twice in the same mutation-stable phase.
Non-claims:
  no PHI meaning change, missing-input policy change, Binding SSA shape
  expansion, Loop production caller, backend change, or timing gate.
```

Observed production consumers are `materialize_all_phi_inputs` at JoinIR
application/module finalization and `for_pred` from builder emission, If join,
PHI lifecycle, and batch publication. The design is accepted as a bounded
read-only D0; implementation remains closed until the mutation boundary is
represented by existing owner state and focused stale-batch rejection is
defined. No new cache/receipt or second PHI authority is permitted.

### `MIR-PHI-ANALYSIS-BATCH-I0` implementation boundary

The design now permits one focused implementation cell. The batch is local to
one `materialize_all_phi_inputs` invocation and is borrowed by its helpers; no
function field, process cache, or cross-phase storage is added. `prune_unused`
must finish before batch creation. The batch may support only PHI-input
updates and append-only rematerialization, whose new values are consumed via
the existing per-predecessor memo. Any topology, successor, parameter/entry,
or pre-existing definition mutation remains outside the batch and must rebuild
before analysis is used.

Acceptance:

```text
- materialize_all_phi_inputs builds PhiInputMaterializationAnalysis once;
- self-carry completion and grouped-edge rematerialization consume that batch;
- direct test wrapper behavior remains unchanged;
- focused PHI repair tests and cargo check pass;
- no second analysis authority, cache, semantic PHI change, or Loop caller opens.
```

Closeout evidence for `MIR-PHI-ANALYSIS-BATCH-I0`:

```text
CARGO_BUILD_JOBS=4 cargo check --profile quick -p nyash-rust --lib       # passed
CARGO_BUILD_JOBS=4 cargo test --profile quick --lib phi_input_materializer # 17 passed
rustfmt --edition 2021 --check src/mir/builder/ssa/phi_input_materializer/edge_rematerialization.rs src/mir/builder/ssa/phi_input_materializer/function_repair.rs # passed
bash tools/checks/current_state_pointer_guard.sh + git diff --check      # passed
```

The production materializer now builds one invocation-local analysis after
unused-PHI pruning and shares it with self-carry completion and grouped-edge
rematerialization. The direct completion test wrapper keeps its existing API
and creates its own local batch. No persistent cache, topology mutation, PHI
meaning change, or Loop production caller was added.

## `MIR-LOCAL-SSA-PREPARED-OPERAND-D0` design stop (2026-09-10)

The next bounded row is a design stop, not permission to add a persistent
definition index.  A read-only owner audit found that the current prepared
request carries destination, target, and arguments but does not prove the SSA
definition location or use block.  Typed Calls still pass through
`finalize_call_operands`, and the common writer can invoke LocalSSA receiver
materialization again; `MirInstruction::Call` alone cannot select a prepared
path.

```text
Decision:
  Fix the prepared/legacy boundary and enumerate every FunctionLoweringState
  mutation before selecting an I0 definition index or fast path.
Source authority + canonical issuer:
  Existing typed Call owner selects the target. A future explicit prepared
  operand issuer must own receiver/argument definition and block evidence; the
  physical append owner remains the sole emission commit.
Non-authority:
  MirInstruction::Call shape, variable_map, MirType, the field-only
  ExactDefinitionIndexV1, and LocalSSA repair state cannot issue prepared facts.
Fail-fast boundary:
  Prepared receiver/arguments with owner, block, or definition drift reject
  before append. They never repair by name or fall back to LocalSSA; legacy
  Calls retain the current repair owner.
Smallest next slice:
  Census the typed Call issuer, sole append entry, and all mutation paths:
  instruction append, PHI/edge repair, Loop/CFG/SSA, JoinIR rewrite,
  lifecycle/exit emission, and transaction capture/restore.
Non-claims:
  no whole-Call scan deletion, variable-name interning, PHI/type/optimizer/
  backend change, or Call semantic change.
```

Observed surface: `builder_emit.rs` is 548 lines, `builder_emit_core.rs` 229,
`ssa/local.rs` 152, and `ssa/local/materialize.rs` 632.  The existing
`ExactDefinitionIndexV1` is rebuilt inside field-receiver provenance validation
and is not a `FunctionLoweringStateV1` authority.  A future index may update
only after a successful append and must reject stale use after every enumerated
mutation; N/2N scan probes are advisory evidence, not a timing gate.

The caller census identifies the existing typed issuers as
`method_call_terminal.rs::emit_canonical_instance_value_terminal_v1` and the
selected static/standard `*_with_receipt_v1` terminals.  Their physical path
converges on `physical_terminal::emit_finalized_generic_call_v1`, then
`MirBuilder::emit_instruction`, and the shared
`builder_emit_core::append_instruction_core` mutation point.  The ordinary
`emit_unified_call`/`emit_unified_call_with_lookup` path remains the legacy
owner of `finalize_call_operands` and LocalSSA repair; Print's typed
no-destination terminal is outside prepared operand selection.

The definition-index boundary must account for existing direct/scheduled
instruction insertion and removal, PHI/edge repair, CFG/block and terminator
changes, Loop/If/exception/lifecycle block generation, JoinIR rewrite/remap,
function-session install/take/replace, transaction capture/restore,
parameter/signature changes, and ValueId remap.  A `current_block` change
invalidates prepared use-context; clearing `local_ssa_map` or
`schedule_mat_map` invalidates operands derived from those caches.  A reserved
destination is not a definition until append succeeds.

### `MIR-LOCAL-SSA-SELF-CACHE-REUSE-I0` implementation boundary

This is the only implementation slice opened from the prepared-operand design
stop.  It keeps the existing `local_ssa_map` as the sole cache owner and
records `(current_block, returned_value, kind) -> returned_value` only after
`materialize_local_v1` succeeds.  A later request for that already-local value
therefore exits through the existing cache path without another definition
scan.  The first materialization still follows the legacy LocalSSA repair
owner, and the strict stale-cache check remains unchanged.

Acceptance:

- repeated receiver/argument requests for a successful local result return the
  same `ValueId` and emit no second Copy/rematerialization;
- failed materialization never publishes the self-cache entry;
- existing cache clears and transaction capture/restore remain authoritative;
- LocalSSA temporal and Call focused tests plus the quick library check pass;
- no PHI, Loop, Call, type, optimizer, backend, or compatibility behavior changes.

This slice adds no persistent definition index, prepared semantic receipt, new
Call route, or mutation policy.  The broader prepared-operand path remains a
separate design concern until its explicit typed issuer proof is available.

Closeout evidence for `MIR-LOCAL-SSA-SELF-CACHE-REUSE-I0` (`6383fe6a96`):

```text
CARGO_BUILD_JOBS=4 cargo test --profile quick --lib temporal_witness  # 17 passed
CARGO_BUILD_JOBS=4 cargo check --profile quick -p nyash-rust --lib    # passed
rustfmt --edition 2021 --check src/mir/builder/ssa/local.rs \
  src/mir/builder/calls/unified_emitter/temporal_witness_tests.rs      # passed
bash tools/checks/current_state_pointer_guard.sh + git diff --check   # passed
```

The successful materialization result is now self-cached under its existing
block/value/kind key, and the focused repeated-receiver test observes no new
entry.  The cache is written only from `Ok`, so failure paths do not publish a
false result.  No persistent definition index, prepared semantic receipt,
Call route, PHI/Loop meaning, or timing gate was added.

## All-worker surface audit (2026-09-03)

Six read-only workers audited the remaining MirBuilder surface after the
compile-cost baseline. Their reports agree on the following disposition:

`CURRENT_STATE.toml` plus this pointed active card are the current-row SSOT for
this design stop. Do not create a second design card. If the implementation
row changes a durable compiler contract, promote only the settled rule to the
existing perf SSOT and the affected module README/reference during closeout.

```text
Decision:
  The semantic spine is substantially clean for the landed StaticBoxMethod,
  FreeStatic/FreeFunction, Print, and root-lexical DeclaredInstance cohorts.
  Remaining dirt is split between a bounded hot-path policy seam, physical
  namespace/port surface, and verification/cleanup mass. Do not start a second
  Builder or a repository-wide purge.
Source authority + canonical issuer:
  Existing config/env parsers issue invocation values through
  BuilderInvocationConfigV1. Existing invocation sessions install them into
  the compilation-owned CompilationContext; emit code is a borrow-only user.
Non-authority:
  per-instruction env reads, process-global OnceLock, DebugHub/backend callers,
  MIR EffectMask or FunctionSignature inference, LOC/frame counts, test names,
  guard inactivity, and direct-storage/VM benchmark claims.
Fail-fast boundary:
  The selected snapshot must be installed before the first instruction for
  every included production invocation and remain unchanged after an ambient
  environment flip. A canonical session that bypasses the config seam, an
  unclassified hot key, or a live emit path outside the capture boundary keeps
  this row in design_stop.
Smallest next slice:
  None is selected. The snapshot row and its gate repairs are landed; the
  Hako physical-ingress census found no existing borrow-only consumer. Reopen
  only with a finite source -> published-view -> backend caller/delete set.
Non-claims:
  no DebugHub lazy payload change, global cache, Call/backend semantic change,
  direct-storage default change, VM decision, barrel split, variable_map API
  rewrite, guard deletion, or broad test/docs purge.
```

### Finite census boundary

```text
start: BuilderInvocationConfigV1 / canonical session construction
  -> install into the candidate CompilationContext
  -> selected builder_emit, call, copy, SSA, and receiver debug readers
end: selected MIR Builder emit decision before physical append
includes: normal/default and canonical production session entrypoints
excludes: DebugHub consumers outside Builder, backend method_router, VM,
  semantic Call routes, guard registry, docs/archive, and process-global caches
```

### What is still dirty, and its disposition

| surface | observed issue | disposition |
| --- | --- | --- |
| invocation policy | selected emit code rereads environment on every instruction; measured static counts are ordinary `7`, Copy `8`, Call `9` | current D0, then one context-owned snapshot I0 |
| canonical ingress | `CanonicalModuleLoweringSessionV1::open` currently copies `quiet_internal_logs` directly and does not visibly consume `BuilderInvocationConfigV1` | blocker to close in D0 map; use the existing canonical snapshot constructor or explicitly exclude the path; no silent bypass |
| debug events | candidate lookup/`Vec`/JSON payload is built before the Hub gate; Hub is also used outside Builder | separate `MIR-BUILDER-DEBUG-EVENT-LAZY-ARGS-P0`; not part of snapshot I0 |
| emit clones | unconditional/dead name and instruction clones remain candidates | `MIR-BUILDER-EMIT-CLONE-SHAPE-P0`, only after snapshot and a measured owner |
| namespace/ports | `builder.rs` is a large registry surface; context and port fields are widely visible; `variable_map` has direct consumers | defer broad physical thinning until the active Call boundary closes; no field growth now |
| guards | the finite guard audit found no new caller-zero/equal-successor deletion family | retain/park; do not delete by inactivity or filename |
| tests | one exact duplicate mutable-accumulator test has a stronger same-file successor | future `MIR-TEST-MUTABLE-ACCUMULATOR-DUPLICATE-RETIRE-R0`; no broad purge |
| direct storage | direct-slot/array defaults and lifetime/lease guarantees are not established | separate `MIR-C-SPEED-EXACT-MODE-CONTRACT-D0`; no default flip |
| VM/backend | LLVM/EXE/AOT is the product path; VM and nonselected backend are reference/compatibility lanes | no parity or retirement work in this row |

### Ordered task queue

The queue is intentionally finite; a report or local green result does not
open the next item.

1. `MIR-EMIT-DEBUG-POLICY-SNAPSHOT-D0` — current design stop. Freeze the
   exact keys, aliases/defaults, normal/raw capture, canonical capture,
   `CompilationContext` storage, session lifetime, and `src/test_support.rs`
   override boundary. No code change.
2. `MIR-EMIT-DEBUG-POLICY-SNAPSHOT-I0` — one BoxShape change after D0 is
   accepted. Reuse `BuilderInvocationConfigV1` and the existing session
   install seam; keep `MirBuilder` and `RawInvocationChildPortV1` unchanged.
   Prove A/B snapshots survive an ambient C flip, preserve the current flag
   matrix, and drive the selected emit readers to zero process-env reads.
3. `MIR-BUILDER-EMIT-CLONE-SHAPE-P0` — remove only measured dead/unconditional
   clone work at the sole append point; require MIR parity and the same
   compile-cost protocol.
4. `MIR-BUILDER-DEBUG-EVENT-LAZY-ARGS-P0` — separate Hub-owned permit from
   caller payload construction; default-off must not allocate debug payloads.
5. `MIR-CALL-EMIT-LOOKUP-FACADE-RETIRE-S0` and the
   `MIR-BUILDER-VARIABLE-READ-ACCESSOR-S0` family — defer until Call R7 and a
   finite caller/consumer/delete set are closed; these are physical thinning,
   not current semantic work.
6. `MIR-TEST-MUTABLE-ACCUMULATOR-DUPLICATE-RETIRE-R0` — one test-only
   deletion window, with the parent failure-name set unchanged; never mix it
   into a Builder semantic row.
7. `MIR-C-SPEED-EXACT-MODE-CONTRACT-D0` — separate design stop for direct
   storage defaults, generation/lease/lifetime, and exact-lane failure.
8. Historical docs/guard archive work remains in its existing cleanup lane;
   no new per-row guard or archive copy is created here.

The first row is accepted only when the two existing production ingress forms
are explicitly covered by the same snapshot vocabulary. If that cannot be
shown without a new capability axis, the exact outcome is
`NoSafeSlice__InvocationCaptureBoundaryMissing`; do not add a receipt,
adapter, fallback, or another D0.

## D0 decision and I0 contract (accepted 2026-09-03)

The design stop is closed with one existing configuration authority and two
already-existing invocation constructors. This is one semantic snapshot type,
not two policy owners:

```text
normal/raw ingress:
  BuilderInvocationConfigV1::snapshot_for_raw[_with_imports]
  -> ModuleBuilderInvocationSessionV1::open_with_identity
  -> config.install_into(candidate)

canonical ingress:
  CanonicalModuleLoweringSessionV1::open
  -> BuilderInvocationConfigV1::snapshot_for_canonical
  -> the same config.install_into(candidate)

both:
  candidate.comp_ctx owns one immutable Debug/Strict snapshot
  selected emit code borrows a Copy view; it never reads process env
```

The canonical `open` path is therefore included in I0; it may not keep the
current `quiet_internal_logs`-only copy as a silent bypass. Direct test-only
`MirBuilder::new()` fixtures outside an invocation session are not a production
hot-path claim and must use the existing explicit test configuration helpers.

### I0 finite key and reader boundary

I0 captures only the keys already read by the selected Builder emit/call
boundary. The existing parser functions remain the vocabulary authority and
their alias/default behavior is unchanged:

```text
joinir_dev::debug_enabled()
joinir_dev::strict_enabled()
joinir_dev::planner_required_enabled()
builder_local_ssa_trace()
builder_trace_recv()
builder_debug_enabled()
builder_static_call_trace()
builder_static_method_trace()
builder_call_resolve_trace()
```

The snapshot keeps the three JoinIR inputs separately and derives the existing
strict+planner+debug predicate; it does not collapse aliases into a guessed
single flag. The selected reader set is:

```text
builder_emit.rs
calls/unified_emitter.rs
receiver.rs
```

The free `emission/copy_emitter.rs`, `utils::builder_debug_log`, DebugHub's
`NYASH_DEBUG_*` gate/payload, router/observe OnceLock caches, and other
non-selected readers remain outside I0 and are named follow-up owners. I0 must
not claim that all environment reads in the repository are gone.

### I0 acceptance

```text
1. no new MirBuilder field, RawInvocationChildPort field, dispatch supertrait,
   forwarding parameter, semantic receipt, or global OnceLock;
2. one snapshot field below CompilationContext and one install owner;
3. normal/raw and canonical sessions each capture before their first MIR
   instruction and retain independent A/B values after an ambient C flip;
4. selected builder_emit/call/receiver readers use the installed snapshot and
   contain zero direct process-env calls;
5. defaults, aliases, debug-off behavior, strict/planner behavior, MIR bytes,
   and debug output remain unchanged under the existing test matrix;
6. existing pointer/active-surface guards are reused; no new per-row shell
   guard is introduced.
```

Any production emitter outside the finite reader boundary, a changed alias or
default, a need for live environment rereads, or a request to include DebugHub
or backend consumers returns the work to design_stop instead of widening I0.

### I0 closeout evidence (2026-09-03, revalidated 2026-09-10)

The one `CompilationContext` snapshot is installed by both invocation forms;
the normal/raw and canonical session tests each preserve independent policy
values across an ambient environment flip. The selected reader census remains
free of direct process-environment reads, and the Builder module README records
the ownership boundary. The implementation is recorded at `4ba9293900` and
was revalidated on the current branch with:

```text
CARGO_BUILD_JOBS=4 cargo test --profile quick --lib module_invocation_session_p0  # 16 passed
CARGO_BUILD_JOBS=4 cargo test --profile quick --lib module_session_borrow_p0_tests # 3 passed
python3 tools/checks/lib/mir_call_d1b_active_surface_guard.py .                    # passed
python3 tools/checks/lib/mir_call_d1b_active_surface_dispatch.py .                 # passed
selected builder_emit/calls/receiver process-env scan                             # 0 direct reads
git diff --check + current_state_pointer_guard.sh                                   # passed
```

The two post-audit gate repairs below are included in the original closeout so
the next family is not opened on a broken feature build or an inactive stable
dispatch gate. Whole-library health is still a separately classified known-red
baseline, not a green claim. DebugHub payload construction, observer OnceLock
state, and other non-selected environment readers remain outside I0.

### Lazy resolve payload P0 closeout (2026-09-10)

Status: **landed** at `21e85270ac`. The existing `DebugHub` remains the sole
master/category/sample/sink gate and JSONL writer. `resolve.try` and
`resolve.choose` now pass a synchronous private callback through that gate;
candidate lookup, receiver/function/region strings, timestamps, and JSON are
constructed only after acceptance. `resolve.choose` records its Known KPI from
the typed `TypeCertainty` value, so no observer JSON is parsed back into
meaning.

Focused evidence:

```text
CARGO_BUILD_JOBS=4 cargo check --profile quick -p nyash-rust --lib  # passed
CARGO_BUILD_JOBS=4 cargo test --profile quick --lib debug::hub::tests::lazy_event -- --nocapture  # 2 passed
CARGO_BUILD_JOBS=4 cargo test --profile quick --lib method_call_terminal -- --nocapture  # 8 passed
rustfmt --edition 2021 --check src/debug/hub.rs src/mir/builder/observe/resolve.rs src/mir/builder/calls/unified_emitter.rs  # passed
git diff --check  # passed
```

The negative test proves that a disabled master gate does not invoke the
payload callback; the enabled test checks the existing `try` JSON shape. The
selected emitter remains below the 760/800-line boundary. No method target,
MIR, KPI meaning, non-resolve observer, backend, or semantic route changed.
No compiler speedup or whole-library green claim follows from this row.

## Post-audit follow-up queue (2026-09-03)

The following findings were independently checked at the current snapshot I0
boundary. They are taskized here so that the next family cannot silently
inherit a broken feature build or an inactive proof gate. This queue does not
change I0's implementation permission and does not claim that the whole
library is green.

### Confirmed P0 — before opening another semantic family

#### `MIR-BUILDER-VM-REFERENCE-CALLEE-EXHAUSTIVENESS-P0`

Status: **landed**. The VM reference consumer now has explicit typed arms
for `SameModuleInstance`: tracing records the key/receiver, while execution
returns the existing typed unsupported result because this lane has no exact
same-module definition owner. No name or registry recovery was added.

Observed failure: `cargo check --profile quick --features vm-reference --lib`
fails with E0004 in
`src/backend/mir_interpreter/handlers/calls/mod.rs` because the existing
`Callee::SameModuleInstance { .. }` variant has no match arm at the callee
classification and execution sites. The normal library build does not cover
this feature, so the failure is not a known-green claim.

```text
source authority + canonical issuer:
  the existing typed Callee::SameModuleInstance key/receiver product;
  the VM is a reference consumer only.

allowed:
  add the explicit VM arm(s), focused vm-reference evidence, and one feature
  guard. If the VM has no exact execution owner, the arm must be a typed
  Unsupported/terminal result rather than a name or registry recovery.

forbidden:
  wildcard match, target reconstruction, args[0] receiver repair, VM-to-C/
  JSON fallback, new semantic Callee variant, or changing the product backend.

acceptance:
  vm-reference quick check/build is green; SameModuleInstance is handled by
  an explicit typed arm; no fallback/retry or semantic authority moves into
  the VM.

observed evidence:
  `CARGO_BUILD_JOBS=4 cargo check --profile quick --features vm-reference --lib`
  exits 0 after the two explicit arms landed.
```

#### `MIR-GUARD-ACTIVE-ROW-DISPATCH-REPAIR-P0`

Status: **landed**. The stable D1B guard keeps its fixed Call-card authority;
the selected performance snapshot and the landed B-S0 evidence row are
explicit, exact delegations to the existing `current-state-pointer` owner. No
wildcard, unconditional skip, or second shell guard was introduced.

Observed failure: the stable
`mir_call_d1b_cataloged_affine_loan_lifecycle_guard.sh` dispatches through
`mir_call_d1b_active_surface_guard.py`, but the current
`MIR-EMIT-DEBUG-POLICY-SNAPSHOT-I0` row is absent from the declarative dispatch
table and the guard exits rc=1 with `unsupported current row`. The generic
pointer guard passing is not a substitute for this lane guard.

```text
source authority + canonical issuer:
  the existing guard registry and one stable dispatch owner.

allowed:
  register the current performance row through the same manifest-driven
  dispatch path, or explicitly retarget the stable guard to its owning lane;
  validate the actual active-card/pointer and keep the guard green at HEAD.

forbidden:
  unconditional skip, wildcard acceptance, a second shell guard, or a fake
  pass that ignores current_execution_row.

acceptance:
  the stable guard returns 0 for the selected row, rejects a drifted row in a
  negative fixture, and keeps the existing Call-row handlers unchanged.

observed evidence:
  `python3 tools/checks/lib/mir_call_d1b_active_surface_guard.py .` and
  `bash tools/checks/mir_call_d1b_cataloged_affine_loan_lifecycle_guard.sh`
  both exit 0 for `MIR-EMIT-DEBUG-POLICY-SNAPSHOT-I0`; a non-performance row
  still follows the fixed D1B dispatch and remains fail-closed. The direct
  negative delegation probe rejects a performance row paired with the fixed
  D1B card path.
```

The two P0s are gate repairs, not a reason to widen the snapshot semantic
slice. They must be closed (or explicitly ParkedSealed with a recorded owner
and reopen trigger) before the next Call family is opened.

### P1 — bounded published-transport hardening

#### `MIR-CALL-PUBLISHED-TRANSPORT-DEDUP-P1`

The selected-C published-call dispatch currently repeats the same Print and
FreeFunction/static-kind handling in
`lang/c-abi/shims/hako_llvmc_ffi_mir_call_dispatch.inc` and
`lang/c-abi/shims/hako_llvmc_ffi_same_module_body_emit.inc`. A second
production emit edge also remains in `harness_executor.rs`. The current
read-only census found that these are not yet one closed delete set: the C
sites may be alternative entry paths and the generic JSON branch still has
other-family callers. Therefore this task is `NoSafeSlice` until a finite
caller/consumer/delete relation is named; do not extract a helper or add a
fallback while that relation is open.

#### `MIR-CALL-PUBLISHED-VIEW-NEGATIVE-COVERAGE-P1`

Add the smallest direct negative set for the already-published view: join/view
reject variants, phi-at-head coordinate invariant, duplicate definition
publication, and non-scalar argument-lane mismatch. Each case must reject
before object emission and must not re-enter JSON/name/registry repair. Do not
inflate the positive fixture matrix or add a new receipt family.

#### `MIR-CALL-PUBLISHED-VIEW-NEGATIVE-COVERAGE-B-S0`

This is the next selected fast row after the snapshot closeout. It is a
BoxShape-only evidence slice, not a new semantic family. Reuse the existing
`MirModule::add_cataloged_box_method` / `preflight_cataloged_box_method`
publication validators and `PublishedMirBackendView::try_new` error owner.
Add only direct Rust tests for already-issued errors that can be produced
without a new source-backed authority: duplicate key/symbol publication,
missing or mismatched definition, arity/legacy-carrier rejection, and the
builtin-print row shape checks. Do not include phi coordinate or non-scalar
lane semantics here; those need a separate authority decision. Do not touch
the C shim, runner, JSON transport, Call schema, or selected-C admission.

```text
Decision: select B-S0 as the smallest closed negative-evidence slice; keep
the broader transport dedup task parked as NoSafeSlice.
Source authority + canonical issuer: existing cataloged publication
validators and PublishedMirBackendView validator; no new issuer.
Non-authority: JSON/name/registry/header, physical symbol text, args[0], C
success, generic add_function, and local fixture identity.
Fail-fast boundary: the named validator rejects before object admission;
there is no JSON/name/registry re-entry or backend fallback.
Smallest next slice: direct Rust tests for the finite existing error rows,
then one reusable guard/evidence receipt.
Non-claims: no phi-head contract, non-scalar lane support, C/runner changes,
JSON deletion, Call-schema cutover, or new receipt family.
Census boundary: cataloged publication entry and view constructor -> their
existing error terminals; includes duplicate/missing/mismatch/arity/carrier
and builtin-print rows; excludes C transport, phi layout, and lane typing.
```

### B-S0 closeout evidence (2026-09-03)

Status: **landed** as a direct evidence-only slice. The focused module suite
now runs 18 tests with 18 passed and 0 failed, including explicit witnesses
for duplicate cataloged publication, key/symbol/arity drift, missing published
definitions, static/free call arity and result drift, legacy function carriers,
and all existing builtin-print shape errors. The tests exercise the existing
`MirModule` publication owner and `PublishedMirBackendView` validator; no
production route, C transport, runner, JSON schema, or semantic receipt was
added. The source test file remains 581 lines, below the 760/800-line boundary.

Evidence command:

```text
CARGO_BUILD_JOBS=4 cargo test --profile quick --lib
  'mir::function::published_backend_view::tests'
-> 18 passed; 0 failed
```

The broader transport-dedup row remains `NoSafeSlice`; phi coordinate and
non-scalar lane coverage remain separate design questions rather than silent
test omissions. The known whole-library red baseline is unchanged and is not
claimed green by this row.

These P1 tasks remain separate from the next DeclaredInstance or other
semantic-family cutover. Their evidence must distinguish changed-test green
from the known whole-library red baseline.

### Post-B-S0 Hako ingress census (2026-09-03)

The read-only physical-ingress audit found no safe existing route from
`PublishedMirBackendView` into the Hako LLVM-text emitter. Rust has published
StaticBoxMethod/FreeFunction definitions and a typed selected-C path, but Hako
currently consumes `RecipeFacts`/JSON and rejects non-print Global rows. Its
Method route still relies on names, registries, and receiver repair, so it is
not a canonical Instance key consumer. The selected-C entry is shared by
Static/Free/Print families; no family-exclusive deletion boundary exists.

```text
task: MIR-CALL-HAKO-SAME-MODULE-INSTANCE-PHYSICAL-INGRESS-D0
status: ParkedSealed__HakoIngressMissing
reopen: borrowed published-view ingress + one exact family + named caller
         + finite old-edge delete set + no JSON/name/registry/args[0] repair
non-claims: no Hako extension, selected-C retirement, Call-schema change,
            or new semantic/physical adapter in this closeout
```

### P2 — bounded cleanup after the gates are healthy

* `MIR-CALL-ME-DECLARED-INSTANCE-QUARANTINE-R0`: retain `me.method` in the
  language, but keep unsupported arbitrary UserBox selected-C routes as one
  typed quarantine until a published descriptor consumer exists.
* `MIR-CALL-PUBLISHED-TRANSPORT-STALE-SURFACE-R0`: remove stale SSOT wording,
  dead variants, and the old second cleanup value only after their caller-zero
  evidence is recorded. This is physical cleanup, not a new semantic owner.
* The existing duplicate-test retirement
  `MIR-TEST-MUTABLE-ACCUMULATOR-DUPLICATE-RETIRE-R0` remains a separate
  test-only row; it is not folded into a Builder or transport row.

### Gate rule

Until both P0 rows have a green, observable owner, no new Call family is
opened. A focused snapshot test passing is useful evidence for I0 only; it
does not waive the vm-reference feature build, stable dispatch guard, or the
known-red baseline comparison.

The two P0 rows and B-S0 are now landed. A remains parked until its
alternative-entry caller census closes; the next semantic family is not
selected by this evidence-only closeout.

### Verification refresh after B-S0 (2026-09-03)

The seven B-S0 and snapshot tests are now included in the executable baseline:
7578 total / 7411 passed / 138 failed / 29 ignored, inventory SHA-256
`db572fea583c934661886b020801b325408c7ed47bf8025a1e2895077c17c1f1`, and the
unchanged 138-name failure SHA-256
`29569949bacd86b39af4f122dad137ae4d476185363d667722a0b87cf56d4ba1`. The
baseline comparator passes with this classified known-red result.

`CARGO_BUILD_JOBS=4 cargo build --profile quick --features vm-reference --lib`
also completes successfully (the `SameModuleInstance` consumer has explicit
typed trace and unsupported arms). The stable D1B guard now dispatches the
current B-S0 row through its named performance-card delegation; both that
guard and `current_state_pointer_guard.sh` pass. No new compiler, Call, backend,
or semantic route is claimed by this verification repair.

#### `MIR-TEST-MUTABLE-ACCUMULATOR-DUPLICATE-RETIRE-R0`

Status: **landed**. This was one production-neutral test-surface
reduction, selected only after the baseline comparator and both stable guards
returned green. Delete exactly `test_string_accumulator_spec` from
`src/mir/loop_route_detection/support/locals/mutable_accumulator.rs`; retain
`test_mutable_accumulator_spec_simple`. Their AST input, analyzer call, and
assertions are identical after comments and function names are removed; the
candidate has no production reference.

Source authority + canonical issuer: the existing mutable-accumulator test
module and its retained successor; the baseline inventory/comparator is the
only count and failure-receipt owner. Non-authority is the test name alone,
raw counts, local green, historical phase prose, or worker silence.

Fail-fast boundary: before deletion, both exact test names occur once, the
normalized bodies match, the candidate occurs only in `cfg(test)` source and
the inventory, and the baseline is exactly `7578/7411/138/29` with inventory
SHA `db572fea583c934661886b020801b325408c7ed47bf8025a1e2895077c17c1f1` and
failure SHA `29569949bacd86b39af4f122dad137ae4d476185363d667722a0b87cf56d4ba1`.
Any mismatch aborts without editing.

Ordered tasks:

```text
A. run the exact module focused suite and the stable/pointer guards;
B. remove only the candidate body and its one sorted inventory line;
C. refresh the existing baseline to 7577/7410/138/29, preserve the failure
   names/SHA, run the exact retained successor and comparator, then close the
   row with status=landed and no archive/stub/new guard.
```

Acceptance after closeout: the candidate and inventory row are absent, the
successor remains and passes, baseline is `7577/7410/138/29` with the same
138-name failure SHA, the active/pointer guards pass, and no production source
path changed. `test_string_accumulator_spec`,
`test_mutable_accumulator_spec_simple`, and `cargo_lib_red_baseline.py` are
the required evidence anchors for this row.

Implementation evidence: the candidate body and its single sorted inventory
row were removed; the retained successor ran `1 passed; 0 failed`; the
baseline comparator reports `7577 total / 7410 passed / 138 failed / 29
ignored` with inventory SHA-256
`c87404eb91f1436274b93f95d60921273f72f487f314773bffb2efa0a1f324fb` and the
unchanged 138-name failure SHA-256
`29569949bacd86b39af4f122dad137ae4d476185363d667722a0b87cf56d4ba1`.
The stable active-surface and current-state pointer guards pass, and no
production source path changed.

#### `MIR-CALL-PUBLISHED-C-DUAL-CONSUMER-PREPARE-BOXSHAPE-S0`

Status: **landed** (2026-09-03). This was the bounded BoxShape slice selected for
the already-landed published-C transport. It is not a new MIR semantic family
and it does not reopen the Hako ingress or selected-C UserBox design stop.

Decision: share only the exact-site published-row take and the common Global
shape admission used by the two existing C consumers. Keep LLVM text emission,
argument formatting, tracing, and generic compatibility routes local to each
consumer. The existing published-row owner remains the sole issuer.

Source authority + canonical issuer: the existing
`hako_llvmc_published_static_method_rows_begin` / `row_for_site` /
`rows_finish` owner in
`lang/c-abi/shims/published_mir/hako_llvmc_ffi_published_static_method.inc`;
the Rust published view and typed row are only inputs to that existing owner.
The two finite consumers are
`hako_llvmc_ffi_mir_call_dispatch.inc` and
`hako_llvmc_ffi_same_module_body_emit.inc`.

Non-authority: JSON/name/registry lookup, physical symbol parsing, `args[0]`,
`ValueId(0)`, `EMIT`/`set_type`/`append_i64_arg_ref`, local route traces, or
backend success. No new semantic receipt, target identity, fallback, or retry
may be introduced.

Fail-fast boundary: a typed row is taken exactly once at its function/block/
instruction site. The shared admission checks the Global shape, destination,
arity, and numeric argument-row shape before either local emitter runs.
`Absent` leaves the existing non-published route untouched; `Ready` returns a
borrowed row for one local emission; `Malformed` returns `-1` with no fallback;
`Residual` is rejected by the existing `rows_finish` owner. Duplicate sites
remain rejected at `rows_begin`.

Census boundary: start at the two existing `row_for_site` callsites and end at
their local typed static/Free/Print emission or the existing residual terminal;
includes StaticBoxMethod, Builtin Print, and FreeFunction rows; excludes
generic LLVM emission, Hako ingress, JSON transport removal, Method(Some),
non-scalar lane semantics, and Call schema changes.

Smallest next slice: add one helper in the existing published-row `.inc`,
replace the two direct lookups with that helper, and keep both local emitters.
The helper performs `take_once` at the existing exact site.
The helper must not require new state or a callback port. The 800-line
`same_module_body_emit.inc` must shrink below the hard boundary; the other two
owners remain below 760 lines.

Acceptance: direct `row_for_site` calls are `2 -> 0` outside the helper; the
existing Static/Print/Free source-to-exe probes remain equivalent; one
non-entry published call is covered; malformed shape and residual rows fail
before object emission; `fallback/retry = 0`; and no semantic/MIR/JSON/backend
authority changes.

Implementation evidence: the existing published-row `.inc` now owns one
`take_i64_global_row_v1` admission helper. It performs the exact-site
`row_for_site` take once, validates `Global`, destination, arity, and numeric
argument shape, and returns distinct `Absent`/`Ready`/`Malformed` statuses;
the existing `rows_finish` owner still rejects `Residual`, and duplicate sites
remain rejected by `rows_begin`. Both C consumers call this helper while
retaining their own LLVM emission, argument formatting, and traces. No
callback/state port, receipt, fallback, retry, or semantic/MIR/JSON change was
added. The measured files are 177 lines (published-row owner), 383 lines
(`mir_call_dispatch.inc`), and 797 lines (`same_module_body_emit.inc`), with
consumer-side direct `row_for_site` calls reduced from 2 to 0.

Evidence commands/results:

```text
bash tools/build_hako_llvmc_ffi.sh
  -> shared library built successfully
CARGO_BUILD_JOBS=4 cargo build --profile quick --bin hakorune -q
  -> success (existing baseline warnings only)
CARGO_BUILD_JOBS=4 cargo test --profile quick --lib
  'mir::function::published_backend_view::tests' -- --nocapture
  -> 18 passed; 0 failed
RUST_MIN_STACK=16777216 CARGO_BUILD_JOBS=4 CARGO_INCREMENTAL=0
python3 tools/checks/lib/cargo_lib_red_baseline.py --root .
  -> 7577 total / 7410 passed / 138 failed / 29 ignored;
     unchanged 138-name failure SHA-256
```

The real Print source probe still emits and runs with result `42`. A direct C
ingress probe against
`apps/tests/mir_shape_guard/lowering_plan_stage1_emit_program_json_runtime_helper_same_module_min_v1.mir.json`
also consumed a non-entry helper row at `b1.i2` and returned `rc=0` with an
object. The same typed C entry rejects malformed shape, an unconsumed
residual coordinate, and duplicate rows with `rc=-1`; the Static and Free
physical branches accept their typed rows. These are transport/shape proofs;
the already-landed Static/Free source proofs remain the source authority, and
the current row does not claim a new source semantic route. The active-surface
guard, current-state pointer guard, and `git diff --check` all pass.

NoSafeSlice: stop if the helper needs nested local state/callback plumbing,
changes a return-code or trace contract, mixes a third consumer, or requires
sharing LLVM emission/argument formatting. In that case keep the broader
`MIR-CALL-PUBLISHED-TRANSPORT-DEDUP-P1` parked and do not add another adapter.

#### `MIR-CALL-BUILTIN-PRINT-PRODUCER-COVERAGE-S0`

Status: **landed**. This is a producer-evidence row only; it does not
reopen the landed Print publication cutover or widen the semantic family.

```text
Decision: prove that the existing source-backed print producer emits exactly
one typed builtin Print call and that the already-published view accepts it.
Reuse the existing normal-default-root lifecycle seam and the existing
PublishedMirBackendView validator; add no new semantic issuer or transport.
Source authority + canonical issuer: the existing source-backed lifecycle
and MirBuilder builtin Print publisher. The test observes the published
module; it does not issue or reconstruct a target.
Non-authority: test name, JSON/name/registry lookup, C success, backend
fallback, fixture identity, or a guessed call shape.
Fail-fast boundary: source-backed `print(42)` must publish one Global builtin
Print call with `func == ValueId::INVALID`, no destination, one source
argument, and a CanonicalTyped published view. Any missing/duplicate/wrong
shape is a test failure; no alternate route is accepted.
Smallest next slice: one test in
`src/mir/builder/normal_default_root_catalog_lifecycle_tests.rs`, reusing the
existing `session()` and `callable_source()` helpers.
Non-claims: no C shim/runner/JSON/VM/Call-schema change, no Hako ingress, no
new guard executable, no fallback/retry change, and no whole-library green
claim.
```

Allowed files are limited to the existing lifecycle test, this performance
card, `CURRENT_STATE.toml`, the rolling workstream, and the existing stable
D1B dispatch module. The focused test must be named
`source_backed_print_producer_publishes_typed_builtin_row`. The existing
stable guard may explicitly dispatch this row, but no standalone guard or
new guard registry entry is allowed.

Acceptance: the exact test passes; it observes one source-backed Print call,
`Callee::Global(CanonicalGlobalTargetV1::builtin_print())`, `dst == None`,
`func == ValueId::INVALID`, one source argument, and
`PublishedStaticMethodRouteV1::CanonicalTyped` with one builtin Print row and
zero static/free rows. The existing lifecycle test was renamed and expanded,
so the `#[test]` count did not increase; the baseline inventory contains the
new name in the same one-row position and the pre-existing failure-name set
is unchanged. Pointer and active-surface guards pass, and the row closes
without changing production semantics.

NoSafeSlice: stop and return to closeout if the existing lifecycle cannot
expose the published module, if the assertion needs a new source/backend
owner, if a second Print producer or route is found, or if a new guard,
fallback, retry, JSON, C, VM, or Call-schema change becomes necessary.

Implementation evidence: (2026-09-03) the existing
`source_backed_lifecycle_facade_consumes_program_runtime_and_app_once` test
was renamed to
`source_backed_print_producer_publishes_typed_builtin_row` and now checks the
source-backed `print(42)` module before retaining its existing Program/App
lifecycle assertions. It observes exactly one call with
`Callee::Global(CanonicalGlobalTargetV1::builtin_print())`, no destination,
`ValueId::INVALID`, one source argument, and one builtin row in a
`CanonicalTyped` `PublishedMirBackendView`; static/free rows are empty. No
new `#[test]` was added. The exact full-path test ran `1 passed; 0 failed`.
The known-red comparator accepts `7577 total / 7410 passed / 138 failed /
29 ignored` with inventory SHA-256
`30764d77ec4ae277300c58b400c7080d0a840ee76065d6395f84fe3d33637739` and the
unchanged failure SHA-256
`29569949bacd86b39af4f122dad137ae4d476185363d667722a0b87cf56d4ba1`.
The stable active-surface guard, current-state pointer guard, and
`git diff --check` pass. The existing lifecycle module still has four
pre-existing known-red tests; they were observed but not reclassified by this
evidence-only row.

Closeout: complete; keep the published Print producer proof in the existing
lifecycle owner and return to the next bounded semantic/backend decision.

#### `MIR-CALL-PUBLISHED-BACKEND-VIEW-C-TRANSPORT-BOXSHAPE-S0`

Status: **landed** (2026-09-03). This is a behavior-neutral owner split
before future Call schema growth; it does not reopen semantic publication.

`implementation_permission = true` applies only to this physical split.

```text
Decision: move only the C/FFI transport structs from PublishedMirBackendView
to src/mir/function/published_backend_view_c_transport.rs. Keep validation,
borrowed view, route decisions, and all existing public paths unchanged.
Source authority + canonical issuer: existing PublishedMirBackendView and
published-row owner; this row issues no semantic target or receipt.
Non-authority: C layout, JSON/name lookup, physical symbol, backend success,
fallback/retry, and file placement.
Fail-fast boundary: none is changed; from_view, NUL checks, arity checks, and
route errors remain owned by the current view/transport contract.
Smallest next slice: add #[path = "published_backend_view_c_transport.rs"]
and re-export the three existing C types (`PublishedCallKindV1`,
`PublishedStaticMethodCallCRowV1`, `PublishedStaticMethodCFrameV1`); move
their definitions verbatim.
Acceptance: parent and child are <800 lines, public re-export paths remain
unchanged, behavior unchanged, new semantic authority = 0, new receipt = 0,
new guard = 0, fallback/retry = 0, and existing 18 view tests remain green.
NoSafeSlice: stop if a private-field accessor, error-type split, API change,
test-path change, semantic validation change, or another consumer is needed.

Implementation evidence: the existing `PublishedCallKindV1`,
`PublishedStaticMethodCallCRowV1`, and `PublishedStaticMethodCFrameV1`
definitions now live in the private sibling
`src/mir/function/published_backend_view_c_transport.rs`; the parent keeps the
historical `pub(crate)` re-exports and all view validation unchanged. The
parent is 583 lines and the sibling is 155 lines. `cargo check --profile quick
--lib`, the exact 18 published-view tests, the current-state pointer guard,
the active-surface guard, and `git diff --check` passed. No semantic authority,
receipt, guard, route, fallback, or retry was added.

Closeout: complete. Return the pointer to the existing R6 post-Group-B census
closeout; the next executable row must be selected only from the already
closed family-local scheduler and must not reopen the WASM Global/Extern/Method
reader stops, which are already landed.
```

Allowed files are the parent view, the named private sibling, this existing
performance card, `CURRENT_STATE.toml`, and the existing active-surface guard
and dispatcher. No new guard or receipt family is allowed. Implementation is
branch/worktree scoped and must return to the C0 closeout pointer after the
row is landed.
