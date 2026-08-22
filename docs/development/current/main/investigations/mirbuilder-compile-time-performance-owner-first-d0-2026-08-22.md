---
Status: parked; design_stop
Task: MIR-COMPILE-TIME-PERF-OWNER-FIRST-D0
Date: 2026-08-22
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
same target/quick compiler binary digest
same backend and env snapshot
warm process and cold process reported separately
>= 9 samples per case
median + p95, not one best sample
correctness/probe result recorded beside timing
```

Use a small source, a representative MIR-heavy source, and the existing
merged callable probe. External `/usr/bin/time` is acceptable for the first
observation; phase counters may be test-only or debug-off observation fields.
Do not use generated executable runtime time as compile-time evidence.

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

This card is intentionally parked. The active lane remains
`MIR-CALLABLE-LOOP-ORDINARY-BRIDGE-D0`; performance is not a production
blocker or an implementation permission. The next allowed performance action
is the baseline/census P0 after the active design stop is selected for work.
