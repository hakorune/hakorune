---
Status: P0a-S0b architecture fixed; S0b-PROFILE0-S0 is next
Date: 2026-07-20
Scope: measured FINALIZE0 production topology and repair observation
Parent: docs/development/current/main/investigations/mirbuilder-finalize0-census-task-2026-07-20.md
Decision: docs/development/current/main/investigations/mirbuilder-finalize0-p0-production-census-consultation-2026-07-20.md
---

# FINALIZE0-CENSUS0-P0: measured source topology and repair observation

## Decision

Candidate A-prime is selected with normalized evidence ownership.

```text
Rust source topology
  -> production direct-callsite census
  -> entry-family route graph
  -> facade child coverage

selected test routes
  -> child entered observation
  -> operation-specific delta observation
```

Static source topology and selected runtime observation are different proof
products. Neither may substitute for the other.

## Evidence ownership

### Entry family owns callsite and route evidence

One `Finalize0EntryFamilyCensusV1` owns:

```text
entry_family_id
entry_def_paths[]
production_direct_call_sites[]
production_direct_callsite_count  # derived len only
authority_root_reachability{}
boundary_root_reachability{}
build_profile_reachability{}
reachability_witnesses[]
```

Semantic operation rows reference `entry_family_id`. P0a removes these
repeated manual fields from every operation row:

```text
production_invocation_count
route_reachability
canonical_repair_reachable
```

No aggregate such as the old `declared_production_invocations=93` is an
authority. It multiplied one physical family callsite set by its child-row
count.

### Count law

```text
production_direct_callsite_count =
  len(distinct repository production direct-call source sites
      resolving to the entry family under one declared build profile)
```

`DirectCallSite` admits both:

```text
ExprCall
ExprMethodCall
```

Runtime execution multiplicity, loop count, function count, and repository-
external public API callers are not represented by this scalar.

## Product 1: `RustSourceTopologyV1`

This is a generic check-only neutral product.

Recommended physical home:

```text
tools/checks/rust_source_topology/
  Cargo.toml
  README.md
  src/
    main.rs
    cargo_topology.rs
    cfg_profiles.rs
    source_sites.rs
    direct_calls.rs
    resolver.rs
    report.rs
```

It is a standalone check tool, not part of compiler/runtime crates and not a
FINALIZE0 policy owner. Existing app-specific
`apps/rust-subset-to-hako/tools/syn_adapter` is not reused as the authority;
that adapter emits a different application vocabulary.

The first implementation may use:

```text
cargo_metadata
syn
cfg-expr
serde / serde_json
```

The exact dependency versions are locked by the standalone tool's Cargo.lock.
The root compiler workspace dependency surface does not change.

### Neutral output

```text
cargo_targets
module_graph
items
cfg_expressions
active_profiles
direct_call_sites
unresolved_call_sites
```

Final neutral callsite fields after Cargo topology and conservative resolution:

```text
path
module_def_path
enclosing_item_def_path
expression_kind
source_range
normalized_callee
resolved_callee_def_path | unresolved_reason
cfg_expression
active_profiles
```

S0a does not publish semantic module/item def-paths. Its single-file rows use
`module_syntax_path`, `enclosing_item_syntax_path`, and report-local syntax
IDs. S0b/S0c may add semantic def-path projections only from their Cargo graph
and supported resolver evidence. Byte range and normalized syntax are
diagnostic observations, not cross-reorder stable source identity.

Substring ordinal may remain diagnostic-only. It is not identity authority.
The extractor performs no FINALIZE0 classification.

### Supported resolver boundary

First admission:

```text
fully-qualified path call
unique non-glob use path call
self.method() inside an exact impl type
method call on a parameter/local with an explicit supported type
```

Typed unresolved result, never guessing:

```text
wildcard import
macro-generated watched call
trait dispatch
function pointer
closure invocation
receiver requiring general type inference
ambiguous alias
unparsed include! source
unsupported cfg expression
unsupported guard
```

If general Rust semantic resolution becomes necessary, stop and select a
rust-analyzer-class semantic resolver. Do not grow spelling heuristics.

## Build/cfg profiles

P0a records the exact checked profile set. Minimum vocabulary:

```text
host-default
host + vm-reference
host + llvm-harness
target wasm32
test
```

This is not a claim over every feature combination. An unclassified profile
or three-valued cfg result remains explicit `Unknown`.

`cfg(test)` is derived from Cargo target/module/item inclusion, not filename
or path spelling. Inline `#[cfg(test)] mod tests` must be excluded even when
the containing filename looks production-like.

## Product 2: `Finalize0EntryFamilyCensusV1`

The Python FINALIZE0 policy checker consumes neutral topology JSON and a small
root/guard fixture. It owns FINALIZE0 entry-family membership and route policy.

Recommended files:

```text
tools/checks/lib/mirbuilder_finalize0_production_census.py
tools/checks/fixtures/mirbuilder_finalize0_route_roots_v1.json
```

Route evidence has three independent axes.

### Authority roots

```text
LegacyBuilder
CanonicalAPlus
CanonicalBindingSsa
JoinIrBridge
```

Labels are summaries only. Exact root symbols are retained. The first
`CanonicalBindingSsa` proof includes distinct roots for:

```text
MirCompiler::compile_resolved + TrivialBindingSsa guard
MirCompiler::compile_resolved_callable_module
MirCompiler::compile_resolved_recursive_callable_module
```

### Execution-boundary roots

```text
Verifier
MirJsonExport
VmExecution
BackendPreflight
ToolDirectVerify
HostProvider
NyLlvmc
```

Boundary roots are never lowering authority roots.

### Build profiles

```text
reachable_build_profiles
```

Every route witness retains the exact build/cfg profile under which its edge
exists.

## Three-valued reachability

```text
Reachable
Unreachable
Unknown
```

```text
Reachable:
  supported direct edge and guard witness produce a path

Unreachable:
  supported roots and guard vocabulary prove no path

Unknown:
  source/cfg/resolution/guard evidence is insufficient
```

Any watched FINALIZE0 operation with `Unknown` fails P0a.

Initial guard vocabulary remains bounded:

```text
cfg(...)
enum variant match
bool literal / equality
matches!(enum, Variant)
if let Variant = value
fixed enum argument passed through a direct wrapper
```

General symbolic execution is forbidden.

## Product 3: `Finalize0FacadeChildCoverageV1`

Two surfaces are recorded separately:

```text
operation-site reverse census:
  inline MIR/fact mutation
  metadata publication
  verifier logic

project-local call-edge reverse census:
  child function call
  nested facade call
  helper call
```

Every project-local direct child edge is exactly one of:

```text
inventory_operation
semantic_facade
pure_orchestration_helper
diagnostic_helper
reason_allowlisted_compatibility_helper
```

Standard-library calls may be grouped only when the neutral extractor resolves
their crate as `std`, `core`, or `alloc`.

Project-local helper allowance is callsite-specific and retains:

```text
source site
resolved callee def-path
reason
owner
allowed mutation surface
expiry / retirement owner
```

Function-name-only allowlists are forbidden. One unclassified child blocks
P0a closure and sends the operation back to schema correction.

Expected schema-expansion candidates:

```text
optimizer child schedules
contract validation and carrier-summary children
semantic all-functions/layout/fixpoint children
callsite canonicalization rewrite children
extern route/result-fact children
```

## Product 4: `Finalize0RepairObservationMatrixV1`

P0b is test-only and session-scoped.

```text
fixture_id
authority_root_id
boundary_root_id?
operation_id
entered
changed
before_digest
after_digest
observed_surface
```

`entered` and `changed` are independent. `changed=false` does not prove the
child was skipped.

Mutation digests are operation-specific and not limited to a pass's existing
`usize` counter.

```text
Type repair:
  transient value_types / kinds / semantic origins

PHI repair:
  MIR / CFG / next_value_id / instruction spans
  transient facts / function metadata

Call/Await repair:
  type facts / semantic origin facts

Metadata publication:
  exact owned metadata fields
```

Observation state is installed and removed inside one test session. New
production state, logs, environment variables, persistent counters, or retry
routes are forbidden.

## Static versus dynamic claims

P0a may claim:

```text
production module/cfg inclusion is measured
direct callsites are source-derived
entry-family callsite sets are complete
static potential reachability is measured
unclassified facade children = 0
unresolved watched calls = 0
unknown watched reachability = 0
```

P0b may claim:

```text
selected exact root fixture entered a child or did not
selected exact root fixture changed the declared mutation surface or did not
```

Neither may claim:

```text
all canonical inputs have zero delta
static reachable code is not a semantic consumer
repository-wide canonical consumer count = 0
zero observed delta proves static unreachability
```

Later quarantine/CUT0 requires structural canonical disconnection or an
equivalent guard proof.

## Task order

### `FINALIZE0-CENSUS0-P0a-S0a` — closed

```text
production behavior delta = 0
FINALIZE0 policy consumers = 0
```

Create the standalone check-only crate, README boundary, neutral JSON schema,
and parser-backed single-file item/direct-call extraction. It must distinguish
`ExprCall` and `ExprMethodCall`, retain enclosing item/range/cfg syntax, and
emit typed unresolved rows. No Cargo module graph or route policy is claimed
yet.

Closed evidence:

```text
standalone crate:
  tools/checks/rust_source_topology

schema:
  rust-source-topology-v1

single-file fixture:
  items = 13
  direct calls = 13
  typed unresolved calls = 13
  typed opaque syntax sites = 4

real-file observation:
  src/mir/compiler/mod.rs
  items = 52
  direct calls = 150
  typed unresolved calls = 150
  typed opaque syntax sites = 27
```

S0a deliberately names module/item values as syntax paths, not semantic Rust
def-paths. It preserves direct and inherited `cfg`/`cfg_attr` syntax without
evaluating inclusion. Half-open byte ranges round-trip to exact source slices;
macro invocation, `include!`, ordinary external `mod`, and `#[path] mod` are
typed opaque rows rather than false empty call inventories. Parse failure is a
typed nonzero tool failure. Deterministic fixture rerun, call-kind separation,
closure/async lexical context, Unicode range roundtrip, and all files below
800 lines are green.

Commands:

```bash
cargo fmt --manifest-path tools/checks/rust_source_topology/Cargo.toml -- --check
cargo test --manifest-path tools/checks/rust_source_topology/Cargo.toml
cargo run --quiet --manifest-path tools/checks/rust_source_topology/Cargo.toml -- \
  single-file src/mir/compiler/mod.rs \
  --module-syntax-path hakorune::mir::compiler
```

S0a does not claim Cargo target/module inclusion, active cfg profiles,
production callsites, semantic resolution, repository completeness, route
reachability, facade coverage, runtime observation, or FINALIZE0 policy.

### `FINALIZE0-CENSUS0-P0a-S0b` — architecture fixed

S0b adds a second product without changing the S0a single-file schema.

```text
RustSourceTopologyV1
  syntax facts for one physical source file

RustCargoTopologyV1
  explicit compile-unit profiles
  Cargo target roots
  module/include instances
  three-valued inclusion evidence
```

Physical files are not module identity. The same file may appear under
different Cargo targets, module syntax paths, include chains, and profiles.
Each occurrence remains a distinct instance. `include!` is a same-module
source inclusion edge, never a child-module edge.

#### Exact cfg boundary

S0b uses Kleene three-valued evaluation.

```text
not(Unknown) = Unknown
all(...) = Excluded if any Excluded
           Included if all Included
           Unknown otherwise
any(...) = Included if any Included
           Excluded if all Excluded
           Unknown otherwise
```

Known evidence is limited to one explicit compile unit:

```text
Cargo-resolved root-package features
explicit test-harness disposition
Cargo profile debug_assertions / panic disposition
rustc --print cfg --target <exact triple>
sealed ambient RUSTFLAGS / CARGO_ENCODED_RUSTFLAGS policy
```

Build-script cfg, undeclared custom cfg, proc-macro-generated topology,
unsupported attribute macros, unsealed target-feature flags, and nonliteral or
generated includes remain typed `Unknown`. S0b never converts unknown evidence
to false. `cfg!(...)` is a value expression, not source inclusion; both source
branches remain in the topology.

`cfg_attr` may affect inclusion only through recursively supported nested
`cfg` and literal `path`. Unknown topology-affecting conditions remain
`Unknown`. Non-topology attributes such as `allow`, `inline`, and `no_mangle`
do not alter inclusion.

#### Initial exact profiles

Profile labels are summaries; every row retains package, target name/kind,
target triple, Cargo profile, compile mode, requested/default/resolved
features, rustc cfg/version digest, repository Cargo-config digest, and
ambient rustflags policy.

```text
host-default-dev
host-vm-reference-dev
host-llvm-harness-dev
wasm32-default-dev
host-test-unit-default
host-default-release
```

The release twin is mandatory because FINALIZE0 currently has known
`debug_assertions`-dependent behavior. `wasm32` target selection does not
activate the `wasm-backend` feature. Requesting `llvm-harness` does not
activate the compatibility alias `llvm`; feature dependency direction remains
the Cargo-declared direction. A library unit-test compile unit and an
integration-test target are distinct; the latter's library dependency does
not gain `cfg(test)`.

#### Module and include boundary

Supported module traversal:

```text
inline mod
ordinary external x.rs xor x/mod.rs
literal #[path = "..."] mod
literal item/module-position include!("...")
```

Ordinary module lookup accepts exactly one candidate. Missing or dual
`x.rs`/`x/mod.rs` candidates are typed failures. Excluded edges are not read.
Unknown edges are not guessed or followed. Literal path/include files must
remain inside the workspace, exist, and avoid canonical-path cycles.

Unsupported first-profile surfaces remain explicit:

```text
cfg_attr-controlled path with Unknown condition
nonliteral include / concat! / env! / OUT_DIR
expression, statement, impl, or trait fragment include
macro-generated module or call
external workspace source escape
custom cfg that controls a watched module edge
```

#### S0b task order

```text
FINALIZE0-CENSUS0-P0a-S0b-D0
  this architecture lock

FINALIZE0-CENSUS0-P0a-S0b-PROFILE0-S0  # closed
  disconnected project/profile schema
  explicit profile input validation
  pure three-valued cfg/cfg_attr decision
  Cargo/module consumers = 0

FINALIZE0-CENSUS0-P0a-S0b-CARGO0       # sole next
  cargo_metadata target/package adapter
  exact compile-unit feature/rustc/config evidence

FINALIZE0-CENSUS0-P0a-S0b-MODULE0
  inline/ordinary/literal-path module instances
  profile-gated traversal

FINALIZE0-CENSUS0-P0a-S0b-INCLUDE0
  literal item/module-position include instances
  include-chain identity and cycle rejection

FINALIZE0-CENSUS0-P0a-S0b-P0
  synthetic workspace/profile/module/include parity
  root nyash-rust exact-profile observation

FINALIZE0-CENSUS0-P0a-S0b-G0
  deterministic/atomic report
  typed Unknown and unresolved topology closure
```

PROFILE0-S0 closed evidence:

```text
schema:
  rust-cargo-topology-profile-schema-v1

validated initial inputs:
  host-default-dev
  host-vm-reference-dev
  host-llvm-harness-dev
  wasm32-default-dev
  host-test-unit-default
  host-default-release

focused decisions:
  feature / target / test / debug-release separation
  llvm-harness does not activate llvm alias
  wasm32 does not activate wasm-backend
  cfg_attr conditional cfg implication
  unknown custom path selector remains Unknown
  unsealed target_feature remains Unknown
  malformed/profile-drift errors remain typed
```

`ValidatedBuildProfileInputV1` deliberately stores
`expected_activated_root_features`; it is not a Cargo feature proof. One pure
`CfgEvaluationEnvironmentV1` may use those expectations in disconnected
fixtures, while CARGO0 must replace that assumption with Cargo-derived closure
and exact rustc/config evidence. Production CLI consumers, Cargo metadata
calls, module/include traversal, semantic resolution, FINALIZE0 policy, and
compiler behavior remain zero through PROFILE0-S0.

Focused command:

```bash
cargo test --locked --offline \
  --manifest-path tools/checks/rust_source_topology/Cargo.toml
```

Every implementation file is split by responsibility before reaching 800
lines. `extract.rs` receives no S0b traversal policy.

#### S0b stop conditions

Stop before widening if any selected watched edge requires:

1. build-script execution output or arbitrary custom cfg inference;
2. proc-macro expansion or rustc-internal name resolution;
3. nonliteral/generated or arbitrary-context include expansion;
4. choosing one ambiguous module candidate;
5. filename/path heuristics for test or production classification;
6. semantic call/item resolution, FINALIZE0 policy, or route ownership;
7. root compiler dependency or compiler/runtime behavior changes;
8. a source/check file reaching 800 lines.

S0b may claim exact bounded inclusion only for declared compile units and
supported syntax. It may not claim production callsite counts, repository
completeness, semantic def-paths, route reachability, facade coverage, or
FINALIZE0 repair ownership.

### `FINALIZE0-CENSUS0-P0a-S0c`

Add the conservative resolver admission above. Unsupported/ambiguous watched
calls become typed unresolved rows. Generic extractor G0 requires all neutral
fixtures and file/line guards green.

### `FINALIZE0-CENSUS0-P0a-P0`

Add FINALIZE0 entry-family normalization, direct-call census, three route
axes, three-valued reachability, and facade child reverse coverage. Remove the
three repeated manual fields from operation rows.

Required initial corrections include:

```text
canonical post-transform enclosing symbol:
  finish_built_canonical_module

finalize_module authority reachability:
  includes CanonicalBindingSsa
```

### `FINALIZE0-CENSUS0-P0a-G0`

```text
manual operation-row count/route/canonical-repair fields = 0
unresolved watched calls = 0
unknown watched reachability = 0
unclassified project-local facade children = 0
entry-family callsite count = derived len only
```

### `FINALIZE0-CENSUS0-P0b-S0`

Add one test-only scoped observer with entered bit and mutation-surface digest.
Production state/log/env behavior remains zero.

### `FINALIZE0-CENSUS0-P0b-P0`

Required exact-root fixtures:

```text
Legacy root
Canonical A+ root
Canonical BindingSSA trivial root
Canonical BindingSSA acyclic-module root
Canonical BindingSSA recursive-module root
relevant Verifier/VM/JSON/backend boundary roots
```

### `FINALIZE0-CENSUS0-P0-G0`

Static and dynamic products are complete and remain separate. Only census
completeness is claimed.

Then:

```text
FINALIZE0-VERIFY-SPLIT0
```

## Required neutral extractor fixtures

```text
fully-qualified ExprCall
unique imported ExprCall
self.method ExprMethodCall inside impl
explicitly typed receiver method call
inline cfg(test) exclusion
host feature/profile inclusion
wasm target exclusion/inclusion
same filename under different module cfg
wildcard import unresolved
ambiguous alias unresolved
trait dispatch unresolved
function pointer unresolved
closure call unresolved
macro watched call unresolved
include source unresolved
source reorder stable identity where range changes are expected diagnostics only
```

## Counters and guards

```text
RustSourceTopologyV1 owners = 1
Finalize0EntryFamilyCensusV1 owners = 1
Finalize0FacadeChildCoverageV1 owners = 1
Finalize0RepairObservationMatrixV1 owners = 1

generic extractor FINALIZE0 policy conditions = 0
operation rows storing manual production counts = 0 after P0a-P0
operation rows storing route arrays = 0 after P0a-P0
operation rows storing canonical-repair booleans = 0 after P0a-P0

watched unresolved call acceptance = 0
watched Unknown reachability acceptance = 0
unclassified project-local child acceptance = 0
filename-only cfg classification = 0
substring-ordinal identity authority = 0

production observer state = 0
production observer logs = 0
observer environment variables = 0
persistent compiler counters = 0

compiler behavior delta = 0
runtime/backend/ownership delta = 0
CUT0 consumers = 0
source/check files >= 800 lines = 0
```

## Implementation may claim after full P0

```text
production direct-callsite sets are parser/module/cfg derived
entry families own callsite and route evidence once
authority roots, boundary roots, and build profiles are independent
every project-local facade child has one disposition
static reachability is three-valued and watched Unknown is rejected
selected entered and changed observations are distinct
operation rows no longer repeat manual count/route/repair reachability facts
```

## Implementation must not claim

```text
runtime invocation multiplicity
all external callers are known
complete Rust semantic resolution
static reachability proves mutation
selected zero delta proves canonical disconnection
repository-wide canonical consumer zero
repair quarantine or CUT0 readiness
compiler/runtime/backend behavior changes
```

## Stop conditions

1. Generic extractor needs FINALIZE0-specific names or policy.
2. cfg inclusion requires filename/path heuristics.
3. Watched unresolved calls are guessed.
4. General Rust inference, trait dispatch, or macro expansion is required.
5. Authority, boundary, and build-profile roots must be merged again.
6. Facade child edges remain unclassified or use function-name-only allowance.
7. Three-valued reachability is collapsed before evidence is complete.
8. Static reachability is used as an actual delta claim.
9. Observation requires production state, logs, env, or persistent counters.
10. P0 needs compiler behavior or CUT0 changes.
11. Root workspace dependencies must change for the standalone check tool.
12. Any source/check file reaches 800 lines.

## Decision lock

> Candidate A-prime is selected with normalized evidence ownership. A generic,
> parser-backed, Cargo/module/cfg-aware check-only Rust extractor emits neutral
> three-valued source topology and direct-call facts. Entry families own each
> production direct-callsite set and its derived count exactly once, plus
> independent authority-root, boundary-root, and build-profile reachability.
> Semantic operation rows reference an entry family and retire their manual
> count, route array, and canonical-repair boolean. Every project-local facade
> child is operation-owned or callsite-reasoned. Static code reachability is
> closed by P0a, while test-only scoped entered/changed mutation-surface
> observations are closed by P0b. Neither product proves runtime multiplicity,
> canonical consumer zero, quarantine, or CUT0 readiness. The sole next
> code-facing row is `FINALIZE0-CENSUS0-P0a-S0b-PROFILE0-S0`.
