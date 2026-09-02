---
Status: baseline observation complete; implementation successors remain gated
Task: MIR-COMPILE-TIME-PERF-OWNER-FIRST-D0
Date: 2026-09-02
Priority: measure compiler-time fixed costs before changing the canonical MIR spine
Parent: MIRBUILDER-FINAL-PIPELINE-v1
NextCard: MIR-COMPILE-TIME-PERF-BASELINE-P0
---

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

The former Hako SameModuleInstance physical-ingress lane is ParkedSealed, so
the owner-first compile-cost train is selected without reopening that semantic
family. `MIR-COMPILE-COST-BASELINE-P0` completed on 2026-09-02: the existing
runner now has an immutable binary/source/environment identity, one warmup,
five retained samples, median/p95 output, and strict shadow-contract evidence
for the accepted static-method probes. The optional loop-bound probes remain a
separate route-coverage rejection and are not a false green.

The next row is design-stop gated:
`MIR-EMIT-DEBUG-POLICY-SNAPSHOT-D0`. It may open only after the recorded
baseline is accepted by the current pointer and the exact invocation-owned
configuration seam is named. No Builder, Call/backend semantic, environment
policy, clone, forwarding-layer, direct-storage-default, or VM-selection
change is authorized by the completed baseline itself.
