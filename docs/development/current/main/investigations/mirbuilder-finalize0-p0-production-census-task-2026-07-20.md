---
Status: FINALIZE0-CENSUS0-P0a-S0b and VERIFY-SPLIT0-S0/P0/I0/FUNCTION-G0-D0/S0/P0/G0 and PHI-SPLIT0-D0/S0/M0/P0/I0-SELECT/MODULETX0-S0 are closed; PHI-SPLIT0-REMATFACT0-D0 is next
Date: 2026-07-21
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

FINALIZE0-CENSUS0-P0a-S0b-CARGO0-S0    # closed
  neutral Cargo metadata snapshot vocabulary
  pure declared-unit package/target/feature seal
  cargo_metadata process consumers = 0

FINALIZE0-CENSUS0-P0a-S0b-CARGO0-M0    # closed
  cargo_metadata locked/offline process adapter
  sealed rustc cfg probe and Cargo-config fingerprints

FINALIZE0-CENSUS0-P0a-S0b-CARGO0-P0    # closed
  dependency-free synthetic workspace proof
  root six-profile declared-unit observation

FINALIZE0-CENSUS0-P0a-S0b-CARGO0-G0    # closed
  exact evidence and stop-line guards

FINALIZE0-CENSUS0-P0a-S0b-MODULE0      # closed
  inline/ordinary/literal-path module instances
  profile-gated traversal

FINALIZE0-CENSUS0-P0a-S0b-INCLUDE0     # closed
  literal item/module-position include instances
  include-chain identity and cycle rejection

FINALIZE0-CENSUS0-P0a-S0b-CFGSTREAM0-S0   # sole next
  one source-order cfg/cfg_attr stream decision
  no eager evaluation after the first excluding row

FINALIZE0-CENSUS0-P0a-S0b-CFGSTREAM0-P0
FINALIZE0-CENSUS0-P0a-S0b-CFGSTREAM0-I0
FINALIZE0-CENSUS0-P0a-S0b-CFGSTREAM0-G0

FINALIZE0-CENSUS0-P0a-S0b-CONTENTCFG0-S0
  root-or-module-edge content-candidate gate vocabulary

FINALIZE0-CENSUS0-P0a-S0b-CONTENTCFG0-R0
  private parse/classify typestate
  excluded content exposes no direct items

FINALIZE0-CENSUS0-P0a-S0b-CONTENTCFG0-P0
FINALIZE0-CENSUS0-P0a-S0b-CONTENTCFG0-I0
FINALIZE0-CENSUS0-P0a-S0b-CONTENTCFG0-G0

FINALIZE0-CENSUS0-P0a-S0b-INCLUDE-SCOPE0-S0
FINALIZE0-CENSUS0-P0a-S0b-INCLUDE-SCOPE0-P0
FINALIZE0-CENSUS0-P0a-S0b-INCLUDE-SCOPE0-I0
FINALIZE0-CENSUS0-P0a-S0b-INCLUDE-SCOPE0-G0
  module-local path imports and inherited textual macros are distinct

FINALIZE0-CENSUS0-P0a-S0b-P0
  synthetic workspace/profile/module/include parity
  root nyash-rust exact-profile observation

FINALIZE0-CENSUS0-P0a-S0b-G0
  deterministic/atomic report
  typed Unknown and unresolved topology closure
```

#### INCLUDE0 authority lock

`include!` is a same-module source-occurrence edge. It never creates a module
instance or a synthetic module path segment.

```text
logical module identity:
  exact surrounding module instance

include path base:
  lexical parent directory of the source occurrence containing the invocation

module directory while traversing included items:
  included file parent with relative owner = none
```

The last two directories are deliberately different from the surrounding
module's ordinary-module lookup directory. An included file may itself contain
ordinary, inline, or literal-path modules; their parent module identity remains
the surrounding module, while their filesystem lookup starts at the included
file. This is the bounded rustc directory law and is not reconstructed from a
module name.

One ordered source scan owns both declarations:

```text
ModulePositionItemV1 =
  Module(ModuleDeclarationV1)
  | Include(IncludeDeclarationV1)
```

Adding a second include pass is forbidden because it would lose the expansion
position relative to sibling module declarations.

The durable topology product gains one occurrence edge:

```text
DeclaredIncludeEdgeV1 {
  include_edge_id
  owning_module_instance_id
  parent_source_observation_id
  parent_include_edge_id
  invocation_range
  cfg_decision
  literal_path?
  selected_source_path_workspace_relative?
  child_source_observation_id?
}
```

Every included source observation keeps the same `module_instance_id` as the
owning edge and points back through `parent_include_edge_id`. The chain is
derived from edge/observation references; a duplicate textual include-chain
field or a canonical-file global dedup table is forbidden. Two sibling
includes of the same physical file remain two edges and two observations.

The existing canonical ancestry stack is shared by module and include loads.
Only an ancestor canonical re-entry is a cycle. A completed sibling reuse is
legal. Successful product invariants are:

```text
module instances = 1 + Included module edges
source observations = defining module observations + Included include edges
Included include edge -> exactly one child observation
Excluded include edge -> zero child observation and zero path probe/read
include child module identity = owning surrounding module identity
partial report on any error = 0
```

The first admission accepts only unqualified module-position `include!` with
one literal string and an optional trailing comma. It accepts crate/module
items, inline-module items, nested includes, and included-file module items.
It rejects expression, statement/block, impl, trait, foreign, generated, and
nonliteral include forms before topology publication. Unknown cfg stops before
path interpretation or filesystem access; excluded cfg does neither.

Unqualified spelling is not by itself a semantic built-in-macro proof. An
observed local `include` macro definition/import, wildcard macro import, or
other unsupported macro-identity ambiguity is a typed unresolved stop. The
topology extractor does not guess macro resolution or add a name-based
fallback.

INCLUDE0 may claim bounded source-occurrence closure only. It may not claim
general macro expansion, expression-fragment inclusion, semantic item/call
resolution, production callsite counts, FINALIZE0 route policy, or compiler
behavior.

INCLUDE0 closed evidence:

```text
ordered ModulePositionItemV1 owners = 1
DeclaredIncludeEdgeV1 owners = 1
include path resolution owners = 1
same-module included source occurrences = exact
included-file child-module directory law = exact
nested include parent chain = exact
sibling same-file include occurrences = independent
module/include shared ancestor-cycle rejection = exact
Excluded include path/token probes = 0
Unknown include acceptance = 0
macro-identity guessing = 0
project CLI / FINALIZE0 policy consumers = 0

focused INCLUDE0 tests = 7
MODULE0 focused tests = 7
manifest-backed INCLUDE0 guards = 1
source/check files >= 800 lines = 0
```

Commands:

```bash
cargo test --manifest-path tools/checks/rust_source_topology/Cargo.toml
tools/checks/run_row_guard.sh --only rust-source-topology-module-traversal
tools/checks/run_row_guard.sh --only rust-source-topology-include-traversal
cargo check -q
```

The root profile still reaches source files with file-level inner `cfg` and
`cfg_attr`. INCLUDE0 does not reinterpret them or skip their contents. The next
row therefore selects one content-gate authority before S0b-P0; block-local
module identity remains a different widening and may not be smuggled into that
decision.

#### CFGSTREAM0 / CONTENTCFG0 / INCLUDE-SCOPE0 decision lock

The source-order audit selects three separate rows. They must not be collapsed
into one implementation change:

```text
CFGSTREAM0:
  shared ordered cfg/cfg_attr decision semantics

CONTENTCFG0:
  apply that decision to one root-or-module-edge content candidate

INCLUDE-SCOPE0:
  prove that an unqualified include! still denotes the builtin macro
```

This split is required by the observed Rust laws. Inner attributes are
processed in source order. The first excluding `cfg` removes the attached
crate/module content and later attributes are not evaluated. An inactive
`cfg_attr` does not evaluate its nested attributes. Conversely, the complete
source must still parse before stripping. Reusing the current eager
all-row evaluator unchanged would reject malformed or unknown rows which rustc
never reaches.

Primary specification anchors:

- [conditional compilation](https://doc.rust-lang.org/reference/conditional-compilation.html)
- [inner attributes](https://doc.rust-lang.org/reference/attributes.html#inner-attributes)
- [attributes on modules](https://doc.rust-lang.org/reference/items/modules.html#attributes-on-modules)
- [`macro_rules!` scope](https://doc.rust-lang.org/reference/macros-by-example.html#scoping-exporting-and-importing)
- [ordinary item scopes](https://doc.rust-lang.org/reference/names/scopes.html)

##### `CFGSTREAM0` — sole next code-facing prerequisite

One pure owner decides an ordered attribute stream:

```text
CfgAttributeStreamDecisionV1
  rows[]
  final_state = Included | Excluded | Unknown
  decisive_row_ordinal?
```

Each row retains its exact source ordinal/range/syntax and one disposition:

```text
Evaluated
TopologyNeutral
NotReachedAfterExclusion
```

`cfg_attr` expansion remains nested evidence under its source row; it is not a
second flat authority. `Unknown` is never converted to false and never guessed
from a filename, feature spelling, build success, or rustc output. The existing
outer module/include cfg consumers become thin users of this same stream owner.
CONTENTCFG0 may not introduce a second predicate engine.

Required fixtures:

```text
false cfg before malformed/unknown row:
  Excluded; later row NotReached

malformed/unknown row before false cfg:
  typed failure / Unknown

inactive cfg_attr with malformed nested cfg:
  Included; nested attribute not evaluated

active nested cfg_attr:
  exact recursive decision

empty stream:
  Included
```

Task order:

```text
CFGSTREAM0-S0  # closed: pure vocabulary/decision, production consumers = 0
CFGSTREAM0-P0  # closed: source-derived inner-topology attribute/profile matrix
CFGSTREAM0-I0  # sole next: replace the existing eager cfg-row owner once
CFGSTREAM0-G0  decision owners = 1; eager all-row owners = 0
```

##### `CFGSTREAM0-S0` closeout

`CfgAttributeStreamDecisionV1` now owns a disconnected, source-ordered
cfg/cfg_attr decision. Each source row retains its ordinal, exact range, and
syntax. The decision reuses the existing one-row predicate evaluator; it adds
no second cfg predicate engine and has no production consumer.

The stream law is now explicit:

```text
Excluded row:
  preserves all later source rows as NotReachedAfterExclusion without parsing
  or evaluating them

Unknown row:
  is terminal and preserves only the consumed prefix
  later rows are neither parsed nor allowed to overwrite Unknown with Excluded

inactive cfg_attr:
  retains nested token evidence as NotEvaluatedInactiveCfgAttr without parsing
  the nested attributes

active nested cfg_attr:
  applies the same exclusion short-circuit recursively and records later
  parsed nested attributes as NotReachedAfterExclusion
```

The disconnected fixtures cover the false-before-malformed/unknown,
malformed-before-false, Unknown-before-false, inactive nested malformed,
active recursive nested, active nested short-circuit, malformed active nested
separator, strict source ordinal, direct and recursively nested cfg_attr
path-unknown, and empty-stream boundaries. `cargo test
--manifest-path tools/checks/rust_source_topology/Cargo.toml`, the existing
MODULE0/INCLUDE0 guards, and the current-state pointer guard are green. P0 now
owns source-derived/profile matrix evidence; it must not connect the existing
eager traversal consumer.

##### `CFGSTREAM0-P0` closeout

`FileInnerTopologyAttributeSurfaceV1` is now the disconnected source product
for one complete Rust file. It owns a workspace-relative source path, exact
source digest, and one file-local ordered stream of only inner `cfg`,
`cfg_attr`, and `path` attributes. Each stream row preserves the parsed meta
range and its exact source slice; token-display reconstruction is not used.
Inner documentation comments and other non-topology attributes are purposely
outside this product because their comment-shaped source is not a second syntax
authority. File-local rows are never merged across files.

The existing Cargo/rustc evidence conversion was moved to the neutral
`cfg_environment_from_declared_unit_evidence_v1` owner. Both the still-legacy
module traversal and this disconnected proof use that one conversion; P0 does
not assemble feature, target, test, or debug facts from a profile label.

The root `src/**/*.rs` static inventory now proves:

```text
inner cfg rows      = 17
inner cfg_attr rows = 0
inner path rows     = 0
```

Against the six sealed Cargo/rustc profiles, direct raw-file stream decisions
are deterministic and contain no `Unknown` result. The raw source matrix has
eight Included rows only for `host-vm-reference-dev`; each of the other five
profiles has zero Included rows. This is not a module-content reachability
claim: the later CONTENTCFG0-P0 owns the distinct outer-candidate matrix of
eleven host-test rows, all Excluded.

P0 remains disconnected from module/include traversal and emits no module
instance, content gate, path selection, or compiler fact. `CFGSTREAM0-I0` is
now the sole next row and may make the existing eager module/include cfg users
thin consumers of the selected stream owner.

##### `CFGSTREAM0-I0` decision lock

I0 has one bounded source-order cutover. The stream decision gains one ordered
`CfgAttributeActivePathEffectV1` projection, produced during the same outer or
nested `cfg_attr` evaluation. A direct path effect keeps its exact outer row;
a nested effect keeps the exact enclosing outer row plus its nested index path
and selected path syntax. Effects are present only when the complete stream is
Included. They do not perform literal parsing, module lookup, or filesystem
selection.

`ModuleDeclarationV1` and `IncludeDeclarationV1` retain one exact outer
topology input stream (`cfg`, `cfg_attr`, and direct `path`) while source spans
and source slices are available. The module layer calls the ordered stream
decision once per declaration. It consumes only its final state, same-owner
path effects, and nested decision dispositions: literal-path parsing and
path-resolution stay in the module layer; active nested shape validation reads
the decision evidence but does not evaluate predicates. Inactive and
not-reached nested rows are never parsed again.

`DeclaredModuleEdgeV1` and `DeclaredIncludeEdgeV1` publish the exact stream
decision rather than the lossy eager `CfgDecisionV1`; the declared module
topology observation schema therefore advances to V2. This is a check-only
observation-schema change, not a compiler behavior change. `decide_cfg_rows_v1`,
`outer_cfg_syntax`, `collect_cfg_attr_paths`, and
`validate_cfg_attr_contents` retire in this row. The direct profile fixtures
migrate to stream inputs. I0 must preserve Unknown-before-path failure,
inactive nested path no-op, active nested literal path selection, duplicate
active-path rejection, and active unsupported attribute rejection without a
second predicate evaluator.

##### `CONTENTCFG0` — module-content candidate authority

The gate is not owned by a non-root child `ModuleInstance`. Rust removes an
inline or external module item when its inner `cfg` is false. The gate must
therefore precede child-instance issuance:

```text
ModuleContentCandidateIdV1 =
  Root
  | ModuleEdge(edge_id)

DeclaredModuleContentGateV1 {
  candidate_id
  defining_surface
  inner_cfg_sites[]
  cfg_decision
}
```

`defining_surface` is bounded evidence, not a semantic module instance:

```text
SourceFile {
  workspace_relative_path
  content_digest
}

InlineBody {
  parent_source_observation_id
  body_range
}
```

Outer declaration cfg and inner content cfg remain separate. They have
different filesystem timing and may not be collapsed into one `Excluded`
boolean:

```text
root:
  root instance = 1
  root content gate = 1
  gate Excluded keeps crate identity but exposes zero contents

outer declaration Excluded:
  content gate = 0
  path/read/parse = 0
  child instance = 0

outer declaration Included + inner Excluded:
  defining source is resolved/read/parsed
  content gate = 1
  selected external source path may be recorded
  child instance = 0
  active S0a source observation = 0
  descendant module/include probes = 0

outer declaration Included + inner Included:
  content gate = 1
  child instance = 1
  active source observation = 1
  direct-item traversal enabled

inner Unknown:
  typed failure
  returned partial topology = 0
```

The central invariant is:

```text
child_instance_id.is_some()
  iff outer_cfg == Included
  and content_cfg == Included
```

`include!` adds another source occurrence to the already-Included surrounding
module. It creates no content gate of its own. A top-level inner attribute in
an included item fragment remains rejected even when its `cfg_attr` condition
is inactive. An inline or external module declared by that fragment receives
its own normal module-edge content candidate.

The parser/traversal boundary becomes a private typestate:

```text
ParsedModuleContentDraftV1
  -> classify with CfgAttributeStreamDecisionV1
  -> Included { gate, direct_items }
     | Excluded { gate }
```

Only `Included` exposes direct items. This prevents excluded content from
triggering block-module, include-identity, missing-child, path, or descendant
validation. It does not admit block-local modules in active content.

Task order:

```text
CONTENTCFG0-S0
  candidate/gate/defining-surface vocabulary
  production consumers = 0

CONTENTCFG0-R0
  one-level private parse draft
  eager inline descendant collection = 0
  active accepted shapes delta = 0

CONTENTCFG0-P0
  pure gate and synthetic transaction matrix

CONTENTCFG0-I0
  root/external/inline traversal connection
  successful topology published only after complete traversal

CONTENTCFG0-G0
  content gate owners = 1
  Unknown gates in successful products = 0
  excluded-content descendant probes = 0
```

Required fixtures:

```text
root: no attrs / true / false / Unknown
external: true / false / Unknown
inline: true / false / Unknown
outer false + missing file: no probe
inner false + syntactically invalid source: parse error
inner false + missing grandchild: no grandchild probe
false-before-malformed attr vs malformed-before-false
inactive vs active nested cfg_attr
active inner path: typed parked stop
inner path after excluding cfg: NotReached
included fragment top-level inner attr: reject
included fragment inline child inner cfg: normal child gate
active block-local module: unchanged typed rejection
```

The exact repository census for the six declared library profiles is:

```text
file-level inner cfg rows in repository = 17
file-level inner cfg_attr rows = 0
file-level inner path rows = 0

reachable rows:
  host-test-unit-default = 11, all Excluded
  other five profiles = 0
```

The eleven reachable files are five resolved-lowering tests, three compiler
activation tests, two interpreter-legacy tests, and `plugin_hygiene.rs`.
CONTENTCFG0-P0 must pin this matrix without treating current absence of inner
`cfg_attr`/`path` as permission to omit their typed laws.

##### `INCLUDE-SCOPE0` — bounded macro-identity correction

The closed INCLUDE0 structure currently has one discovered scope drift. Its
single `include_macro_ambiguity` boolean conflates two different Rust scopes,
scans cfg-excluded items, ignores source order, and propagates parent `use`
imports into external children. In the actual root this makes the wasm-only
`use wasm_bindgen::prelude::*` poison host child modules.

The replacement has two ephemeral lanes:

```text
module-local path/import ambiguity:
  active direct use/rename/glob only
  order-independent inside the current module
  reset at every inline/external child module

textual macro_rules include state:
  source-order
  active only after its definition
  inherited into inline/external child modules
  threaded through same-module included files
```

Each potentially relevant import/definition is first filtered through the
shared cfg-stream authority. Excluded rows have no scope effect; Unknown rows
are typed unresolved. General glob-content resolution, `macro_use`, proc
macros, trait resolution, and exported-macro resolution remain outside this
bounded proof.

Task order:

```text
INCLUDE-SCOPE0-S0  private two-lane scope vocabulary, consumers = 0
INCLUDE-SCOPE0-P0  cfg/order/module-boundary/include-threading matrix
INCLUDE-SCOPE0-I0  remove the blanket ambiguity boolean and connect once
INCLUDE-SCOPE0-G0  scope owners = 1; parent-use child propagation = 0
```

Required fixtures:

```text
cfg-excluded glob + same-module include: accepted
active/Unknown same-module glob: typed unresolved
parent use glob does not poison external/inline child include
textual macro definition before child does poison child include
definition after child does not retroactively poison child
included-source textual scope returns to following sibling items
excluded module content performs zero scope scan
```

The root six-profile S0b-P0 proof remains forbidden until all three rows are
green. CONTENTCFG0 does not absorb the scope correction, and INCLUDE-SCOPE0
does not infer content visibility independently.

##### Claims and stop conditions

After the three G0 rows, implementation may claim only:

```text
cfg/cfg_attr streams are evaluated once in rustc source order
root and module-edge inner content gates decide instance issuance exactly
excluded content publishes no active call/item/module/include facts
unqualified literal include identity uses bounded correct module/textual scope
the six declared profiles reach no Unknown topology in this bounded surface
```

It must not claim general item-level cfg projection, general macro expansion,
block-local module identity, complete Rust name resolution, production direct
callsite census, FINALIZE0 route reachability, repair quarantine, or CUT0
readiness.

Stop if any row requires:

1. a second cfg/cfg_attr evaluator;
2. evaluation after an excluding row;
3. active facts from excluded raw S0a content;
4. a child module instance for non-root inner `cfg(false)`;
5. inner `path` normalization rather than a typed stop;
6. included-fragment top-level inner attributes;
7. general glob/`macro_use`/proc-macro resolution;
8. persistent macro catalogs or filename/name special cases;
9. block-local module widening;
10. compiler/runtime/backend behavior changes;
11. any source/check file reaching 800 lines.

Before CONTENTCFG0-I0, split parser/content-gate and traversal state into
separate modules. `traversal.rs` is already near the line budget; growing one
more recursive state machine there is forbidden.

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

### `FINALIZE0-CENSUS0-P0a-S0b-CARGO0` — authority lock

`cargo metadata` is not an actual Cargo-unit graph or build-success proof.
CARGO0 therefore names its durable product declared-unit evidence.

```text
Cargo manifest / metadata
  package declaration
  target declaration
  metadata-invocation root feature closure

explicit compile-mode request
  test-harness disposition only

sealed rustc cfg probe
  exact cfg rows for the explicit probe argv

Cargo/config fingerprints
  exact observed inputs
  not a reimplementation of Cargo profile inheritance
```

S0 accepts one neutral metadata snapshot and one already-validated profile.
It selects a workspace package by exact manifest identity, never by package
name alone. It then selects one exact target by name and semantic target kind,
retains raw Cargo target kinds/crate types, and checks the Cargo resolve-node
feature set without reimplementing feature dependency direction.

The Cargo resolve-node feature `default` remains an exact feature fact. When
default features are enabled and the package declares `default`, the expected
feature set must contain the literal `default`; no comparison projection may
silently remove it. Consequently `cfg(feature = "default")` remains true in
the disconnected cfg decision environment when Cargo proves it active.

The S0 product may claim only:

```text
exact selected workspace manifest/package declaration
exact declared Cargo target root
exact metadata-run root feature closure
request/package/target/feature agreement
target required-feature eligibility
```

It may not claim:

```text
the target was compiled
Cargo internal unit-graph completeness
build success
Cargo-derived debug_assertions or panic strategy
build-script/proc-macro custom cfg
module/include inclusion
semantic call resolution
production or FINALIZE0 reachability
```

M0 must keep evidence owners separate. `cargo_metadata` owns package, target,
and feature resolution. A sealed `rustc --print cfg` probe owns only its exact
argv/result. Repository and ambient Cargo configuration are fingerprinted and
validated separately. If an external config, wrapper, build-script cfg, or
profile setting is needed to claim the actual Cargo invocation, CARGO0 stops
rather than reconstructing Cargo policy.

CARGO0 stop conditions:

```text
package selection by name or opaque Cargo PackageId
absolute path as durable identity
dropping the default feature from Cargo closure
guessing a target kind or required-feature eligibility
claiming actual compilation from metadata
reimplementing Cargo profile inheritance
unsealed global/ancestor Cargo config or cfg-affecting environment
module/include traversal before MODULE0/INCLUDE0
FINALIZE0 policy or compiler behavior connection
source/check file >= 800 lines
```

CARGO0-S0 closed evidence:

```text
neutral snapshot vocabulary owners = 1
declared-unit seal owners = 1
manifest-first workspace package selection = exact
target name + semantic kind selection = exact
raw Cargo kind / crate-type preservation = exact
opaque Cargo PackageId durable identity uses = 0
serialized absolute-path observations = 0
literal default feature projection drops = 0
cargo_metadata process consumers = 0
rustc/config consumers = 0
module/include consumers = 0
FINALIZE0 policy consumers = 0
```

Seven disconnected declared-unit fixtures cover exact success, stable
workspace-relative identity, opaque-ID non-publication, missing/foreign
package/target rejection, exact `default` retention, expected-feature drift,
requested/required feature failure, no-default success, asymmetric LLVM alias
direction, required-feature success, and unit-test-library compile mode.

CARGO0-M0 closed evidence:

```text
cargo_metadata process owners = 1
metadata flags = --locked + --offline + exact --filter-platform
requested feature spelling = exact package-qualified feature
metadata ambient cfg environment = sanitized

rustc cfg probe owners = 1
rustc target / test / debug / panic / feature argv = explicit
rustc cfg/version digests = exact observation

workspace input fingerprint owners = 1
manifest / Cargo.lock / repository config = exact digest
ancestor / Cargo-home configs = enumerated digest
cfg-affecting repository/external rustflags = reject

cargo executable drift acceptance = 0
rustc executable drift acceptance = 0
workspace input drift acceptance = 0
serialized absolute path / opaque PackageId = 0
actual Cargo build or unit-graph claims = 0
module/include/FINALIZE0 consumers = 0
```

Focused M0 fixtures run the root default metadata path twice and require
byte-identical durable evidence, verify host release, wasm32, and unit-test
rustc cfg probes, accept the repository's linker-only rustflags, and reject a
synthetic `--cfg` injection. The first-profile process remains disconnected
from the project CLI and from FINALIZE0 policy.

CARGO0-P0 closed evidence:

```text
dependency-free synthetic profiles = 6
root nyash-rust exact profiles = 6

synthetic target roots:
  library
  required-feature binary
  integration-test target

synthetic feature rows:
  default exact
  no-default exact
  llvm-harness does not activate llvm alias
  required feature exact

synthetic target rows:
  host
  wasm32
  integration cfg(test)

root two complete runs:
  durable JSON byte equality = exact
  absolute workspace path occurrences = 0
  expected feature rows = Cargo resolve rows for all six profiles
  wasm-backend activation from wasm target = 0
```

The synthetic fixture is a separate dependency-free Cargo workspace with its
own lockfile. It proves lib/bin/test target selection and profile/feature
separation without registry or root-workspace dependency behavior. The root
proof executes all six declared profiles twice and retains exact host dev,
host release, VM-reference, LLVM-harness, unit-test, and wasm32 evidence.

Focused command:

```bash
cargo test --locked --offline \
  --manifest-path tools/checks/rust_source_topology/Cargo.toml
```

CARGO0-G0 closed evidence:

```text
declared-unit seal owners = 1
cargo_metadata process owners = 1
rustc cfg probe owners = 1
workspace input fingerprint owners = 1

metadata command:
  literal cargo executable = 1
  --locked = 1
  --offline = 1
  --filter-platform = 1
  package-qualified feature projection = 1
  dependency-pruned metadata = 0

durable process evidence raw snapshot fields = 0
durable process evidence absolute-path fields = 0
project CLI Cargo evidence consumers = 0
Cargo evidence FINALIZE0/MirBuilder policy consumers = 0

disconnected proof tests:
  declared unit = 7
  process/fingerprint = 3
  profile matrix = 2
```

Stable guard:

```bash
tools/checks/run_row_guard.sh \
  --only rust-source-topology-cargo-evidence
```

This guard closes only the Cargo/rustc evidence boundary. MODULE0 remains the
first owner allowed to add profile-gated module traversal, and the project
CLI/report remains disconnected until the full S0b-G0.

### `FINALIZE0-CENSUS0-P0a-S0b-MODULE0` — authority lock

MODULE0 consumes one already-sealed CARGO0 process evidence row plus an exact
workspace root. It revalidates the CARGO0 workspace fingerprints before and
after traversal, then emits only an explicit-`ItemMod` module-instance graph.
The workspace root is an execution capability and is never serialized.

```text
CARGO0 declared unit:
  exact profile / package / target / root source
  exact Cargo-resolved features
  exact rustc cfg flags and key/value rows

MODULE0:
  explicit inline module instances
  explicit ordinary external module instances
  explicit literal-path module instances
  three-valued cfg-gated edges
  workspace-relative source observations

not MODULE0:
  include! expansion
  macro-generated modules
  semantic def-path or call resolution
  entry-family / FINALIZE0 policy
  project CLI/report publication
```

The single-file S0a schema remains unchanged. MODULE0 owns a separate
parser-backed declaration tree because S0a deliberately does not publish the
literal path selector or a traversal policy. Each loaded source occurrence is
still passed through the existing S0a extractor with its exact module syntax
root; one physical file used by different module paths or compile units is not
deduplicated into one semantic instance.

#### Exact directory law

MODULE0 models the same bounded directory state as rustc.

```text
ModuleDirectoryOwnershipV1:
  Owned { relative = None | Some(module_segment) }
  UnownedViaBlock
```

For ordinary `mod x;`, `relative=None` probes exactly `x.rs` and
`x/mod.rs`; `relative=Some(parent)` probes exactly `parent/x.rs` and
`parent/x/mod.rs`. Zero candidates reject as missing and two candidates reject
as ambiguous, even when both canonicalize to the same file. An `x.rs` child
retains `relative=Some(x)` while an `x/mod.rs` child retains `relative=None`.
Cargo roots always start with `relative=None`, regardless of root filename.

Inline modules push any pending relative owner and then their own semantic
segment. A literal `#[path]` on an inline module denotes its child directory.
A literal `#[path]` on an external module denotes its exact source file. Every
external file loaded through `#[path]` starts its children with
`relative=None`, matching rustc's historical mod.rs-equivalent law. Raw Rust
identifiers retain their source spelling for diagnostics but use the unraw
segment for logical and filesystem identity.

#### Exact cfg and filesystem order

The cfg environment is derived only from the sealed CARGO0 feature and rustc
probe evidence. It never returns to profile expected-feature assertions.

```text
parse parent source
  -> extract explicit module declaration
  -> evaluate cfg / topology-affecting cfg_attr
  -> Excluded: record edge, filesystem operations = 0
  -> Unknown: typed stop, filesystem operations = 0
  -> Included: select literal path or ordinary candidates
  -> canonicalize and enforce workspace containment
  -> reject ancestor-stack cycle
  -> read / parse child
```

Known `cfg_attr(..., path = "...")` conditions may select one literal path.
Unknown path selection, a nonliteral path, or multiple active path attributes
rejects before probing a child. Non-topology attributes remain passive, while
an unknown attribute macro on a module is unsupported rather than guessed.

Lexical and canonical workspace containment are separate checks. A symlink
that escapes the workspace rejects. Cycle identity is the current canonical
source ancestry stack, not a global visited set; sibling uses of one canonical
file remain distinct valid instances. No partial graph is returned on error.

#### MODULE0 durable product

```text
DeclaredModuleTopologyV1
  profile_id
  package_key
  target_key
  root_instance_id
  module_instances[]
  module_edges[]
  source_observations[]

ModuleInstanceV1
  instance_id / parent_edge_id
  exact module syntax path
  root | inline | ordinary-file | ordinary-mod-file | literal-path
  workspace-relative source path
  optional inline body range

ModuleEdgeV1
  parent instance / declaration range
  source ident spelling / unraw semantic segment
  inline | ordinary | literal-path
  Included | Excluded decision
  optional included child instance
```

Successful invariants:

```text
root instances = 1
unknown edges = 0
excluded-edge filesystem operations = 0
module instances = 1 + included edges
each included edge has exactly one child instance
each excluded edge has zero child instances
all serialized paths are workspace-relative
```

MODULE0 fixtures must cover root/custom root, inline and nested inline,
ordinary flat and mod.rs forms, non-mod-rs relative ownership, direct and
known-cfg_attr literal paths, excluded missing non-read, Unknown non-probe,
test/debug/release/feature/host/wasm gates, missing/dual candidates,
workspace/symlink escape, ancestor cycle, sibling same-file reuse, child parse
failure, raw identifiers, deterministic output, and an opaque unchanged
`include!` row. The synthetic fixture owns these focused laws; root six-profile
counts remain for the later S0b-P0 row.

MODULE0 stop conditions:

```text
filename or module-name guessing beyond the exact rustc directory law
reading/probing an Excluded or Unknown child
first-match selection of ambiguous candidates or path attributes
global canonical-file deduplication
absolute path serialization
include! or proc-macro expansion
semantic resolution or FINALIZE0 policy
project CLI/report publication
root workspace dependency or compiler behavior change
source/check file >= 800 lines
```

MODULE0 closed evidence:

```text
durable topology owners = 1
module declaration parser owners = 1
sealed CARGO0 cfg-environment owners = 1
external path-resolution owners = 1
S0a observation calls = 1

focused fixtures = 7
fixture profiles = 6

custom Cargo root = exact physical-parent ownership
ordinary x.rs / x/mod.rs = exact relative ownership
inline / path-inline = exact child-directory ownership
path-loaded external child = mod.rs-equivalent sibling ownership
raw identifier spelling / semantic segment = separate

Excluded child probes = 0
Unknown child acceptance = 0
ancestor canonical cycles = rejected
sibling same-file instances = preserved independently
workspace lexical/canonical escape = rejected
partial graph publication on error = 0

project CLI consumers = 0
include expansion consumers = 0
semantic resolution / FINALIZE0 policy consumers = 0
compiler/runtime/backend behavior delta = 0
```

The first profile explicitly stops on reachable source-level inner topology
attributes and block-local modules. Root source files contain examples of the
former, so S0b-P0 may not silently count the whole repository until a separate
content-gate widening row closes them. MODULE0 does not infer either surface.

Commands:

```bash
cargo test --manifest-path tools/checks/rust_source_topology/Cargo.toml
tools/checks/run_row_guard.sh --only rust-source-topology-module-traversal
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
> canonical consumer zero, quarantine, or CUT0 readiness. PROFILE0-S0 and the
> complete CARGO0-S0/M0/P0/G0 chain, MODULE0, and INCLUDE0 are closed. The
> continuation is fixed as `CFGSTREAM0 -> CONTENTCFG0 -> INCLUDE-SCOPE0 ->
> S0b-P0`. `CFGSTREAM0-S0` is closed: one ordered, short-circuiting cfg/cfg_attr
> decision owner now exists with zero production consumers. `CFGSTREAM0-P0` is
> the sole next code-facing row. CONTENTCFG0 then gates
> each Root or outer-Included ModuleEdge candidate before non-root module
> instance issuance. INCLUDE-SCOPE0 separately repairs the discovered
> module-local-import versus inherited-textual-macro drift. Block-local module
> identity, general semantic/macro resolution, FINALIZE0 policy, repair
> quarantine, and CUT0 remain forbidden.

##### `CFGSTREAM0-I0` closeout

Module and include declarations now retain one exact outer topology stream
while source ranges and slices are available. Their traversal calls
`decide_cfg_attribute_stream_v1` exactly once, then consumes only the final
state, same-owner path effects, and nested dispositions. The module layer
does literal parsing and filesystem selection; it no longer evaluates a cfg
predicate or rebuilds a cfg_attr path list.

`CfgAttributeActivePathEffectV1` keeps the enclosing outer row and, for a
nested `cfg_attr`, the exact nested index path. Effects exist only for a fully
Included stream. `DeclaredModuleEdgeV1` and `DeclaredIncludeEdgeV1` now carry
the full stream result, so the declared-topology observation schema is V2.
The lossy eager row product, `outer_cfg_syntax`, `collect_cfg_attr_paths`, and
`validate_cfg_attr_contents` are retired.

Focused fixtures prove inactive and outer-excluded nonliteral paths are never
parsed, active nested literal paths select once, unknown remains terminal, and
active unsupported attributes still fail. The standalone topology suite,
MODULE0/INCLUDE0 guards, current-state pointer guard, and root `cargo check`
are green. The subsequent `CFGSTREAM0-G0` freezes the one stream owner and
the absence of eager/reconstructing consumers before CONTENTCFG0 begins.

##### `CFGSTREAM0-G0` closeout

`rust-source-topology-cfg-stream` is the registered pilot guard for this
cutover. It fixes exactly one ordered stream owner and one one-row predicate
evaluator, two module/include declaration consumers, two V2 edge publications,
and zero eager row owners or cfg_attr path reconstruction helpers. It also
requires active-effect, inactive-path, and excluded-include fixtures and keeps
the guarded implementation/test files below 800 lines.

`CONTENTCFG0-S0` is now the sole next row. It must define only the inner
content-candidate vocabulary before any child-module instance is issued;
inner cfg evaluation, source-content pruning, or compiler behavior remains
for its later rows.

##### `CONTENTCFG0-S0` closeout

The disconnected content-gate vocabulary now keeps `Root` and `ModuleEdge`
candidate identity distinct from `SourceFile` and `InlineBody` defining-surface
evidence. `DeclaredModuleContentGateV1` holds the exact inner stream rows and
the completed stream decision, but no traversal builds or consumes it yet.
Focused tests pin both root/file and edge/inline pairings. The next row,
`CONTENTCFG0-R0`, may introduce only the private parsed-content draft and must
not alter accepted active shapes or connect traversal.

##### `CONTENTCFG0-R0` closeout

One private complete-file draft now parses source before deciding the inner
stream. Its shared inner-surface helper preserves the same source slices used
by CFGSTREAM0. An Included classification alone exposes one-level direct
items; an Excluded classification keeps only its gate, so it cannot trigger
descendant validation in a future I0 consumer. Unknown and active inner `path`
are typed stops. No production traversal calls this draft yet. `CONTENTCFG0-P0`
is next and owns the root/external/inline proof matrix.

##### `CONTENTCFG0-P0` closeout

One `#[cfg(test)]` candidate observer now measures the content gate before
module-instance issuance without connecting the production traversal. It reuses
the selected outer cfg stream, module path resolver, and R0 content draft; it
creates no topology edge, module instance, or compiler fact. Only an
outer-Included declaration is observed as an external or inline content
candidate, and an inner Excluded draft stops before descendant parsing.

The disconnected root/external/inline matrix fixes Included, Excluded, and
Unknown gates; preserves the same candidate and defining-surface evidence;
keeps parse-before-inner-false behavior; keeps a missing descendant behind an
inner false gate; preserves source-order short-circuit before malformed/path
rows; and retains active/inactive nested `cfg_attr` behavior. Existing
MODULE0/INCLUDE0 fixtures continue to pin outer-false no-probe, included
fragment inner-attribute rejection, and active block-local-module rejection.

The exact six-profile repository census is now source-derived:

```text
host-test-unit-default = 11 content candidates with inner rows, all Excluded
host-default-dev / host-default-release / host-llvm-harness-dev /
host-vm-reference-dev / wasm32-default-dev = 0
```

The eleven paths are the five resolved-lowering tests, three compiler
activation tests, two interpreter-legacy tests, and `plugin_hygiene.rs` in
deterministic source traversal order. The topology suite, registered cfg-stream
guard, current-state pointer guard, and root `cargo check` are green. The next
row, `CONTENTCFG0-I0`, alone may connect this already-proved gate to root,
external, and inline traversal; CUT0 remains forbidden.

##### `CONTENTCFG0-I0` closeout

The declared-topology observation advances to V3. It owns one root content
gate, while each module edge owns `content_gate: Option<_>`: absence means its
outer stream excluded the declaration, and an owned gate records the exact
Root or same-edge candidate. A root instance remains when its inner content is
Excluded but has no active source observation. A non-root child instance exists
exactly when both the outer declaration and its content gate are Included.

`content_issuance.rs` owns the single post-outer-cfg transition. It resolves
and parses external source before its inner gate, records an Included-outer /
Excluded-inner edge without a child or active observation, and issues a child
only from the Included direct-item product. Inline bodies retain raw syntax and
their exact body range, then classify before direct module or include
declarations issue. `traversal.rs` remains the outer-cfg/include owner at 574
lines; content issuance is a 275-line sibling.

Root/external/inline Included, Excluded, and Unknown fixtures prove the central
issuance law, parse-before-inner-false behavior, outer-false no-probe, active
inner-path typed stop, included-fragment rejection, and no descendant probe
behind excluded content. The standalone topology suite, CFGSTREAM0 guard,
pointer guard, and root `cargo check` are green. `CONTENTCFG0-G0` is next and
alone may freeze the one production gate/issuance owner and its negative
boundaries. CUT0 remains forbidden.

##### `CONTENTCFG0-G0` closeout

The registered `rust-source-topology-cfg-stream` guard is now the shared
CFGSTREAM0/CONTENTCFG0 boundary guard. It fixes one content-gate vocabulary
owner, one three-way content classifier, one post-outer-cfg issuance owner,
and exactly the root plus external/inline issuance consumers. The only absent
edge gate is the outer-Excluded publication; every outer-Included edge retains
its gate. The guard also fixes two `Excluded -> None` direct-item transitions,
two early returns before child issuance, the typed Unknown stop, and the sole
read-only edge-candidate identity verifier. The test-only candidate observer
remains behind `#[cfg(test)]`.

Focused root/external/inline tests prove Included, Excluded, and Unknown
boundaries; the private draft tests prove that excluded content exposes no
direct items and cannot probe descendants. The registered guard, standalone
topology suite, content-draft suite, checker/root `cargo check`, pointer guard,
and diff check are green. `INCLUDE-SCOPE0-S0` is next. CONTENTCFG0 makes no
claim about module-local import scope, textual macro scope, block-local module
identity, general name/macro resolution, compiler behavior, or CUT0.

##### `INCLUDE-SCOPE0-S0` closeout

One private, disconnected `IncludeScopeLanesV1` now separates the two states
that the legacy `include_macro_ambiguity` bit had conflated. The
order-independent module-local lane records whether the builtin name may be
shadowed within one module. The source-order textual lane records whether a
`macro_rules! include` definition is visible. `child_module_entry` resets only
the module-local lane and preserves the textual lane, which is the required
future inline/external child boundary; same-module included text will instead
continue with the same product in P0/I0.

This row has no production consumer: it does not scan AST items, evaluate cfg,
issue a topology declaration, change the legacy boolean, or alter any typed
error. Two unit tests fix root initialization and the child-boundary law. The
registered reusable INCLUDE0 guard now checks the one vocabulary owner, both
lanes, the child-boundary operation, zero production consumers, the focused
INCLUDE0 suite, and the 800-line limit. `INCLUDE-SCOPE0-P0` is next and alone
owns the cfg/order/module-boundary/include-threading matrix. CUT0 remains
forbidden.

##### `INCLUDE-SCOPE0-P0` closeout

One test-only shared-authority observer now proves the bounded scope matrix
without attaching a production consumer. It reuses the existing root content
draft/parser, the sole ordered cfg-stream gate, the included-fragment parser
facade, and the declaration-owned direct-item range projector. It owns no
second parser, cfg evaluator, source-range calculation, declaration issuance,
or legacy-boolean read.

Five focused fixtures fix the active/excluded/Unknown glob law; module-local
reset for both inline and external child entries; source-ordered textual macro
inheritance; same-module included-text continuation into the following root
sibling; and zero scope scans for excluded root content. The observer stays
behind `#[cfg(test)]`; the registered INCLUDE0 guard freezes the five proof
fixtures, one test-only observer pair, one parser facade, and zero production
scope consumers. The standalone topology suite, focused observer suite,
checker/root `cargo check`, pointer guard, and diff check are green.

`INCLUDE-SCOPE0-I0` is next and alone may replace the existing blanket
`include_macro_ambiguity` connection in traversal. P0 does not claim general
macro resolution, block-local module identity, production direct-callsite
census, compiler behavior, or CUT0 readiness.

##### `INCLUDE-SCOPE0-I0` closeout

One new sibling, `include_scope_traversal.rs`, now owns the sole production
scope stream. `declarations.rs` retains ordered direct `use` and exact
`macro_rules! include` events with existing cfg rows and source ranges, but no
longer manufactures or transports a blanket ambiguity boolean. The traversal
first folds Included direct module-local events over the entire current module;
that lane is deliberately order-independent. It then applies Included textual
macro events in source order, rejects either non-builtin lane at literal
include issuance, and leaves Excluded events inert while keeping Unknown typed.

An inline or external child receives `child_module_entry()`: module-local
state resets and textual state inherits; the child result is discarded. A
same-module included source receives the current scope by value and returns
its final scope to the following parent sibling. `content_issuance.rs` only
threads this child product through the pre-existing content-gate path and
does not classify scope or evaluate cfg. The old blanket state, its parser
parameter, and all production reads are zero.

The focused topology suite expands to eleven tests: excluded and Unknown scope
events, order-independent direct imports, source-ordered textual macros,
inline/external child boundaries, child non-leakage, and included-source
continuation are all fixed alongside the prior include/path/cycle laws. The
checker/root `cargo check`, private proof suite, topology suite, registered
guard, pointer guard, and diff check are green. `INCLUDE-SCOPE0-G0` is next
and owns only final owner/consumer/retirement guard closure; CUT0 remains
forbidden.

##### `INCLUDE-SCOPE0-G0` closeout

The existing reusable INCLUDE0 guard now closes the I0 ownership boundary. It
requires one production scope-stream module, one module-local preparation
owner, one include continuation owner, one writer for each lane, and one child
boundary consumer. It forbids lane writing, child-boundary construction, or
scope preparation in declarations, the legacy traversal shell, and
CONTENTCFG0 issuance; `content_issuance.rs` may only receive its prepared
child scope. The retired blanket boolean is absent from every production
surface, while the test-only P0 proof remains explicitly separated.

The 11 focused topology fixtures and five private proof fixtures are green,
as are the checker/root `cargo check`, registered guard, pointer guard, and
line/diff checks. The root six-profile `FINALIZE0-CENSUS0-P0a-S0b-P0` proof is
now authorized. INCLUDE-SCOPE0 makes no claim about general macro or name
resolution, block-local identity, compiler behavior, repair quarantine, or
CUT0 readiness.

##### `S0b-P0 / G0` closeout

The existing synthetic Cargo/profile, explicit-module, and literal-include
fixtures remain the bounded parity substrate. One root-only proof consumes the
same six sealed CARGO0 evidence rows and calls the existing V3 traversal twice.
It fixes the exact profile order and topology matrix:

```text
host-default-dev:         instances=2282 edges=3128 includes=3 observations=2260
host-default-release:     instances=2281 edges=3126 includes=3 observations=2259
host-llvm-harness-dev:    instances=2282 edges=3128 includes=3 observations=2260
host-test-unit-default:   instances=3538 edges=3633 includes=3 observations=2921
host-vm-reference-dev:    instances=2359 edges=3227 includes=3 observations=2336
wasm32-default-dev:       instances=2255 edges=3096 includes=3 observations=2233
```

Every root content gate is Included; successful traversal is the typed closure
for supported Unknown/unresolved topology. Both V3 JSON results are byte-equal,
the topology profile/package/target keys agree with their sealed evidence, and
no serialized result contains the workspace absolute path. CARGO0 remains the
separate owner of repeated Cargo/rustc process-evidence determinism. The root
topology proof is an explicit milestone test (about three minutes), not a
quick-gate or daily-guard command.

The reusable module traversal guard now pins the one root proof consumer and
the six exact profile IDs; it also corrects two stale pre-CONTENTCFG0 literals
to the current direct-item projector and nine focused MODULE0 fixtures. No new
parser, cfg evaluator, traversal, CLI/report consumer, semantic resolution,
FINALIZE0 policy, repair quarantine, or CUT0 authority is introduced.

`FINALIZE0-VERIFY-SPLIT0-S0` is next.

## FINALIZE0-VERIFY-SPLIT0 — typed-value verifier / stale-row split

### Decision lock

`verify_typed_values_are_defined` currently combines two incompatible roles:

```text
completed-draft definition verification
transient unreferenced stale-row removal
```

The selected end state is:

```text
CompletedMirFunctionDraft
  -> read-only definition verification

FunctionLoweringSession transient facts
  -> explicit stale-row normalization
```

The verifier is correctness-bearing in every build mode once connected.  The
normalizer is a distinct lifecycle operation; strict/dev controls diagnostics
only and may not decide whether semantic validation or normalization occurs.

### Fixed row order

```text
FINALIZE0-VERIFY-SPLIT0-S0
  -> FINALIZE0-VERIFY-SPLIT0-P0
  -> FINALIZE0-VERIFY-SPLIT0-I0
  -> FINALIZE0-VERIFY-SPLIT0-FUNCTION-G0-D0
  -> FINALIZE0-VERIFY-SPLIT0-FUNCTION-G0-S0
  -> FINALIZE0-VERIFY-SPLIT0-FUNCTION-G0-P0
  -> FINALIZE0-VERIFY-SPLIT0-FUNCTION-G0-G0

parked terminal retirement:
  FINALIZE0-PHI-SPLIT0
  -> MODULE-FINALIZE-VERIFY-CUT0
  -> LOOP-LIVEFACT0
  -> MIRBUILDER-CLEAN0-VERIFY-MIXED-RET0-G0
```

### S0 — disconnected definition rows

The active row introduces only a Builder-free vocabulary for typed-value
definition rows, a read-only completed-draft verifier, and a private prepared
transient stale-row product.  It has zero production callers and makes no
fact-map writes.

It fixes these boundaries before any lifecycle rewiring:

```text
defined parameter / defined instruction       -> accepted
ValueId::INVALID                              -> ignored
referenced undefined typed value               -> verifier error
unreferenced undefined typed value             -> prepared stale candidate
pending PHI / pinned transient value           -> not a stale candidate
```

S0 must not change the legacy helper, its strict/dev gate, current finalizer
callers, fact snapshot timing, or error text.  It does not move implicit
Return, TypePropagationPipeline, Call/Await annotation, PHI repair, metadata
publication, diagnostics, or session-generation cleanup.

### P0 / I0 / G0 reservation

P0 proves the row matrix and prepared-then-commit failure boundary.  I0 alone
extracts transient stale-row normalization before sealed fact snapshots and
installs the read-only verifier after the final draft-publication mutation.
The first safe I0 consumer is only `finalize_function_draft`: its pipeline and
Call/Await annotation are complete before metadata publication, and it takes
the draft immediately after the verifier.  `finalize_module` remains legacy
because its PHI inference/materialization still mutates after the current
snapshot; intermediate loop lowering remains a separate diagnostic caller.
The initially reserved global G0 is not currently admissible: module
finalization and loop lowering still have distinct live/post-mutation
boundaries.  The next scoped function G0 may guard only the selected function
finalizer split.  Terminal mixed-helper retirement remains parked until every
remaining production caller has its own exact replacement.

No row in this series may repair a missing lowering fact, infer source
semantics, mutate completed MIR, or use final metadata as a lowering fact.

### S0 closeout

`value_lifecycle_definition.rs` now owns the disconnected, Builder-free
products.  It collects deterministic typed-without-definition rows from a
`MirFunction` plus its transient type snapshot, ignores `ValueId::INVALID`,
and provides a read-only completed-draft failure.  A separate non-Clone
prepared stale-row product can contain only unretained rows; referenced,
pending-PHI, and pinned rows reject before any commit surface exists.

The five focused fixtures cover parameter and instruction definitions, the
invalid sentinel, deterministic completed-draft failure, unretained stale-row
preparation, and each retained category.  There are no production callers and
no `MirBuilder`, map write, finalizer order, metadata, Return, type pipeline,
Call/Await, PHI, or strict/dev behavior changes.  The existing
`rust_lifecycle_mirbuilder_typed_value_verification_guard.sh` is intentionally
not an S0 proof: it still encodes the legacy mixed helper and currently has a
pre-existing stale `module_lifecycle` marker; P0/I0 owns its replacement or
retirement.

Focused library tests, formatting, pointer guard, diff check, and root
`cargo check` are green.  `FINALIZE0-VERIFY-SPLIT0-P0` is next.

### P0 closeout

The same disconnected product now proves the complete pure row matrix:

```text
function parameters and all instruction definitions, including unreachable blocks
  -> defined
ValueId::INVALID, even when present in retention inputs
  -> absent from rows
Missing Integer / Unknown / Void
  -> stable ValueId sort and exact type preservation
completed-draft verification with any residual row
  -> typed failure
unretained multi-row input
  -> one ordered prepared candidate set
referenced / pending-PHI / pinned missing row
  -> typed retention failure
overlapping retention
  -> Referenced > PendingPhi > Pinned
early stale plus later retained row
  -> no partial prepared product
```

The legacy helper and its three direct callers remain exactly as before.  The
loop caller remains an intermediate diagnostic boundary, not a final-draft
consumer.  The obsolete historical typed-value guard is quarantined rather
than repaired: it names pre-FSESSION storage and encodes the mixed strict-gated
helper, so it cannot be evidence for this split.

No production caller, map mutation, snapshot ordering, Return, type pipeline,
Call/Await, PHI, metadata, fact-session, build-mode, or legacy diagnostic text
changes in P0.  Eight focused product tests, formatting, diff check, and the
pointer guard are green; the new source file is 423 lines.  `VERIFY-SPLIT0-I0`
is the sole next row.

### I0 selected slice

`finalize_function_draft` alone adopts the split in this order:

```text
TypePropagationPipeline
-> Call/Await annotation
-> prepare + commit transient stale rows
-> existing metadata type/origin publication
-> read-only completed-draft verifier
-> current-function take
```

Both new operations are unconditional correctness operations.  Strict/dev may
add diagnostic detail later, but cannot choose whether the function-draft
normalizer or verifier runs.  A retained missing row fails before any transient
map or metadata mutation; a residual row after snapshot fails before draft
take.  No MIR or ValueId rollback is claimed.

`finalize_module` and loop lowering retain the legacy helper in this row.  I0
does not reorder or redesign Return, TypePropagationPipeline, Call/Await,
metadata contents/freshness, PHI inference/materialization, session generation,
or the historical guard.  The existing legacy helper is not retired until G0.

### I0 closeout

`finalize_function_draft` is now the sole production consumer of the split.
After its existing type pipeline and Call/Await annotation, it prepares and
commits only unretained transient stale rows before the existing metadata
snapshot.  It then runs the read-only completed-draft definition verifier after
metadata publication and before taking the draft.  Neither operation reads a
strict/dev gate.

The new function-finalizer witnesses prove that an unretained stale row is
removed from all three transient lanes (`value_types`, `value_kinds`, and
`value_origin_newbox`) before the snapshot; a pinned stale row fails with the
new typed retention error while those three rows and the draft remain present;
and an ordinary no-stale finalizer still completes.  A product-level witness
also proves that prepared commit removes exactly those three lanes and leaves a
defined row untouched.

This is not a whole-finalizer rollback claim: pre-existing Return/pipeline work
may already have run before a retained-row failure.  The new normalizer itself
does no MIR, ValueId, metadata, or cache mutation before its successful commit.
`finalize_module` remains legacy because PHI repair can still mutate after its
snapshot, and loop lowering remains an intermediate legacy diagnostic.  The
historical mixed-helper guard remains quarantined.  Focused tests, formatting,
diff check, pointer guard, and root `cargo check` are green.  `VERIFY-SPLIT0-G0`
must not make a repository-wide retirement claim.  `VERIFY-SPLIT0-FUNCTION-G0-D0`
is the sole next row.

### FUNCTION-G0 design lock — scoped closure, not mixed-helper retirement

The original `VERIFY-SPLIT0-G0` reservation promised mixed-helper retirement
after every caller had a replacement.  That condition is false today and must
not be weakened by a source-count guard.

```text
selected split boundary:
  MirBuilder::finalize_function_draft

legacy mixed-helper boundaries retained:
  finalize_module
  loop lowering intermediate diagnostic
```

`finalize_module` runs its legacy helper before type/call annotation, metadata
publication, return-PHI inference, and PHI input materialization.  The latter
can allocate ValueIds, insert predecessor instructions, rewrite PHI inputs, and
remove unused PHIs/instruction spans.  It therefore has no completed-draft
boundary compatible with the selected function verifier.  The loop caller runs
after loop-variable finalization but before enclosing lowering is complete; it
is an intermediate diagnostic rather than a completed draft.

#### `FINALIZE0-VERIFY-SPLIT0-FUNCTION-G0-D0` — next row

```text
code delta = 0

scope:
  function-finalizer split only

global mixed-helper retirement claim = 0
module conversion claim = 0
loop conversion claim = 0
```

It fixes the explicit partition and creates no guard yet:

```text
split prepare production consumer = 1
  finalize_function_draft

split completed-draft verifier production consumer = 1
  finalize_function_draft

legacy mixed-helper production consumers = 2
  finalize_module
  loop lowering
```

#### `FINALIZE0-VERIFY-SPLIT0-FUNCTION-G0-S0/P0/G0`

S0 adds one scoped guard, tentatively named
`rust_lifecycle_mirbuilder_finalize_function_typed_value_split_guard.sh`.  It
must inspect only the bounded `finalize_function_draft` body and the named
definition modules; it must not repair or green the historical
`rust_lifecycle_mirbuilder_typed_value_verification_guard.sh`, which remains a
quarantined module-finalizer/mixed-helper artifact.

The scoped guard locks this exact order:

```text
TypePropagationPipeline::run
-> Call/Await annotation
-> prepare_transient_stale_value_facts_v1
-> prepared commit
-> metadata.value_types snapshot
-> metadata.value_origin_callers publication
-> verify_completed_draft_typed_value_definitions_v1
-> current_function.take
```

It requires one prepare definition and consumer, one completed verifier
definition and consumer, one prepared commit consumer, and exactly one remove
from each stale transient lane.  The selected finalizer must contain none of
`strict_or_dev_planner_required`, `strict_enabled`, `joinir_dev_enabled`,
`planner_required_enabled`, or the legacy mixed-helper call.  P0 proves the
three existing function-finalizer witnesses plus the product commit witness in
the default build profile; G0 may claim only this scoped all-build boundary.

Its stable report must say:

```text
selected_function_finalizer_normalizer_consumers = 1
selected_function_finalizer_verifier_consumers = 1
legacy_mixed_helper_remaining_consumers = 2
module_legacy_conversion_claim = 0
loop_legacy_conversion_claim = 0
mixed_helper_retirement_claim = 0
```

#### Parked successor order

```text
FINALIZE0-PHI-SPLIT0
  read-only edge verifier
  + unused-PHI normalizer
  + full-candidate transactional missing-edge/rematerialization repair

FINALIZE0-DERIVED0 / producer closure
  metadata freshness and post-publication verification boundary

MODULE-FINALIZE-VERIFY-CUT0
  module finalizer conversion only after its post-mutation boundary exists

LOOP-LIVEFACT0
  intermediate diagnostic and live reservation/pending-producer law

MIRBUILDER-CLEAN0-VERIFY-MIXED-RET0-G0
  only then, zero legacy mixed-helper consumers and deletion
```

Stop the scoped row if it needs module PHI mutation/order changes, loop
diagnostic semantics, historical-guard repair, a global helper-zero claim, or
Return/type-pipeline/Call-Await/metadata/fact-session redesign.

#### D0 closeout

The production-callsite audit fixes the partition as one selected split
function-finalizer consumer and two intentionally legacy mixed-helper
consumers.  The old module finalizer has post-snapshot PHI mutation and the
loop caller is an intermediate diagnostic, so neither can be silently folded
into a completed-draft G0.  The historical guard is confirmed obsolete and
quarantined.  No code or check was changed by D0.  The sole next row is the
new scoped `VERIFY-SPLIT0-FUNCTION-G0-S0` guard; terminal mixed-helper
retirement remains parked.

#### S0 closeout

`tools/rust_lifecycle/mirbuilder_finalize_function_typed_value_split.py` and
its seven-line shell entry now form the one scoped guard.  Its brace scanner
handles comments, quoted/raw strings, character literals, and lifetime labels
before extracting only `finalize_function_draft`; it therefore cannot mistake a
format string or the function's `'search` label for a structural brace.  The
guard locks the selected eight-step order, the one prepare/verifier/commit
consumer counts, the one-per-lane commit removals, and absence of build-mode
tokens or the old helper in the selected body.  It separately pins the two
remaining legacy callers and reports zero module/loop conversion and zero
mixed-helper retirement claims.  The historical guard is untouched.

The new guard is listed in the check-script index, exits green on the current
source, and stays below 800 lines.  No Rust production behavior changes.  The
sole next row is `VERIFY-SPLIT0-FUNCTION-G0-P0` for guard-negative and
function-finalizer witness proof.

#### P0 closeout

The scoped guard's four in-memory drift probes reject removal of the prepared
commit, removal of the `value_kinds` stale lane, removal of the module legacy
partition, and insertion of a selected-finalizer build-mode gate.  Its parser
also observes the actual lifetime label and format strings in the selected
source body.  The existing default-profile Rust witnesses remain separate from
the static proof: `finalize_value_lifecycle_tests` is 3/3 and the definition
product suite is 9/9.  The scope remains one function finalizer, not a proof
that module or loop behavior has been converted.  The sole next row is
`VERIFY-SPLIT0-FUNCTION-G0-G0`.

#### FUNCTION-G0 closeout

The scoped guard is green with its drift probes, the function-finalizer
witnesses are 3/3, the definition-product witnesses are 9/9, the Python source
compiles, the pointer guard is green, and every new source/check file remains
below 800 lines.  It proves exactly one all-build prepare/commit/verifier seam
inside `finalize_function_draft`, ordered after the existing pipeline and
Call/Await annotation and before/after the existing metadata publication as
sealed by the guard.

The same report pins two legacy mixed-helper consumers and explicitly reports
zero module conversion, zero loop conversion, and zero mixed-helper retirement.
Therefore this is a function-scoped closure only.  The old historical guard
remains quarantined and the legacy helper remains live.  The next work is the
separate `FINALIZE0-PHI-SPLIT0-D0` design boundary; it must select a complete
candidate-state transaction and post-mutation verification law before any
module conversion or helper retirement can begin.

## FINALIZE0-PHI-SPLIT0 — legacy PHI repair containment

### D0 decision lock — Candidate A′ selected

`materialize_all_phi_inputs` is neither a verifier nor a permanent normalizer.
It combines unused-Phi deletion (instruction plus aligned span),
dominance-based missing predecessor-row completion, and edge rematerialization
that allocates ValueIds, inserts predecessor instructions, and rewrites Phi
inputs. A nested rematerialization can mutate one child before a later child
fails; its `HashMap` work traversal also has no canonical fresh-ID order.

Candidate A′ separates exactly three products:

```text
verify_phi_edges_v1(&MirFunction)
  all-build, read-only, terminator-derived CFG edge verifier

PreparedUnusedPhiNormalizationV1
  private non-Clone candidate plan; no live deletion without side-artifact closure

LegacyPhiRepairCandidateV1
  private non-Clone candidate-only quarantine for unused cleanup,
  missing-edge completion, and rematerialization; drop on failure
```

The edge verifier must not call `update_cfg`. It derives predecessor facts from
terminators and checks reachable Phi targets for duplicate, phantom, and
missing predecessor rows, incoming definitions, edge dominance, and stable
diagnostic order `(block, Phi ordinal, predecessor, value)`. Unreachable policy
is explicit.

The unused-Phi product retains exact instruction/span rows. It may commit only
with a positive side-artifact closure; otherwise it returns
`BlockedByArtifactReference` before mutation. Existing stale type/origin rows
are not such a closure. A function clone can prove helper-local rollback, but
is not a module-finalization transaction: module commit later must own all
candidate functions, module artifacts, relevant transient facts, and
site-indexed artifacts before one external commit.

### Explicit non-selection and caller partition

```text
split verifier + direct live repair
  rejected: partial MIR/ValueId mutation remains

classify every repair as NormalizeRepresentation
  rejected: missing-edge derivation/remat is semantic repair; unused deletion
  lacks artifact closure

type/origin inference or pipeline rerun for remat dsts
  rejected: producer closure owns those facts
```

Whole-function repair has exactly three production callers: two separate
`finalize_module` phases (after a type snapshot and after derived refresh) and
one intermediate JoinIR rewrite phase. Six live `for_pred` completion consumers
are excluded; they are not whole-function repair callers. No caller is
connected in S0/P0. Return, TypePipeline, Call/Await, phi type inference, fact
publication, source/name inference, backend policy, and JoinIR conversion
remain non-authorities.

### Fixed task order

```text
FINALIZE0-PHI-SPLIT0-D0
  this lock; code delta = 0
-> FINALIZE0-PHI-SPLIT0-S0
   disconnected edge verifier + prepared vocabulary; production consumers = 0
-> FINALIZE0-PHI-SPLIT0-M0
   candidate-state, positional-artifact, and caller-timing census
-> FINALIZE0-PHI-SPLIT0-P0
   verifier/plan/candidate-failure/determinism/freshness proof
-> FINALIZE0-PHI-SPLIT0-I0-SELECT
   select one integration only after module candidate ownership and
   post-publication verifier are proven
-> FINALIZE0-PHI-SPLIT0-G0
   scoped guard for the selected integration
```

No production I0 is authorized by D0. Current module/JoinIR callers remain
legacy and terminal mixed-helper retirement remains parked.

### Required proof and stop law

```text
verifier: valid loop; missing/phantom/duplicate predecessor; undefined or
non-dominating incoming; unreachable policy; terminator/cache drift; stable diagnostics

plan: used Phi untouched; exact instruction/span candidate; positional artifact
without closure rejects; no new ValueId

candidate: permitted remat families; cycle/non-rematerializable/missing-pred/
late-RHS failure leaves live MIR/cursor/spans/facts/metadata unchanged;
deterministic sorted output

module: function-N failure leaves all live functions/artifacts unchanged;
repair-before-derived-refresh exactly once; post-publication freshness check
```

Stop before I0 if it needs type/kind/origin inference, source/name/runtime-tag
reconstruction, direct live repair, partial module commit, unproven
instruction-index/site closure, HashMap-dependent output, JoinIR conversion,
or Return/Call-Await/metadata/fact-session redesign.

#### D0 closeout

Three independent audits establish the three direct callers, six excluded live
completion consumers, partial nested-remat mutation, and post-snapshot/
post-derived-refresh staleness. A′ is a containment architecture only; it
claims no production conversion. The following S0 was the sole first
code-facing row.

### S0 closeout — disconnected edge verifier and unused-Phi vocabulary

`src/mir/builder/ssa/phi_input_materializer/edge_verifier.rs` now owns the
two disconnected A′ products, with zero production consumers:

```text
verify_phi_edges_v1(&MirFunction)
  -> direct terminator successors
  -> direct reachable predecessor sets
  -> direct dominator sets
  -> deterministically sorted edge errors

PreparedUnusedPhiNormalizationV1
  -> exact unused Phi instruction/span rows
  -> non-Clone candidate only
  -> no commit surface
  -> BlockedByArtifactReference without a later closure owner
```

The verifier never calls `update_cfg`, never mutates the successor or
predecessor caches, and never writes MIR, facts, metadata, spans, or ValueId
state. Candidate collection rejects a missing instruction span rather than
fabricating one. Focused verifier tests and `cargo check -q` pass; existing
warnings are unchanged.

The following M0 inventories the complete candidate state, positional/site-
indexed artifact closure, direct caller timing, and deterministic
rematerialization order before P0 or any I0 selection. Module and JoinIR
callers remain legacy; no repair conversion, deletion, or retirement claim has
been made.

### M0 closeout — candidate scope, freshness, and deterministic repair census

The legacy helper remains unmodified. Its exact mutation sequence is:

```text
unused Phi instruction/span deletion
-> update_cfg plus missing predecessor-row completion
-> recursive edge rematerialization / fresh ValueId allocation / predecessor insertion
-> original Phi input-slot rewrite
```

It is neither candidate-safe nor deterministic today. `MirFunction.blocks` is
a `HashMap`; its traversal determines work order, fresh ValueId allocation,
predecessor insertion, and some missing-row input order. Recursive operand
materialization can insert/allocate a left child before a later right child,
argument, callee, or target-Phi lookup fails. Duplicate ValueId definitions
are also currently hidden by map overwrite and require a future preflight
reject, never a sorted first/last-wins rule.

The minimum physical function candidate is the complete `MirFunction`, not a
block subset: blocks and aligned spans, terminators, `next_value_id`, CFG
caches/effects/reachability/sealing/return environment, signature/params, and
all `FunctionMetadata`. It is still only helper-local rollback. A module
candidate must additionally own every function, `MirModule` metadata, affected
transient fact/origin-observation state, all positional or ValueId-indexed
derived artifacts, and a post-publication freshness verifier. Existing remat
does not publish type/kind/origin or other transient facts for its fresh dsts;
P0 must preserve that fact rather than repair it by inference.

The three direct production callers remain exactly:

```text
module finalizer after function metadata snapshot
module finalizer after record/typed-object/direct-state refresh
JoinIR apply after live rewritten-block insertion and before later boundary/context work
```

The two module sites prove a `MirFunction` clone cannot establish module
freshness. The JoinIR site proves any later JoinIR integration must own block
application, repair, and subsequent builder/context obligations in one
separately selected transaction. The six `for_pred` callers remain excluded as
producer-time PHI completion, not whole-function repair.

P0 must first prove a pure preflight and stable candidate schedule: sorted
target block/Phi site/predecessor/input rows before allocation; explicit
duplicate definitions, invalid edges, cycles, non-rematerializable values, and
allocator overflow reject before candidate mutation; operand traversal is the
only retained local order. Candidate execution may only delete in descending
per-block index order, rebuild cache from terminators, apply sorted rows,
verify edges, close artifacts, and then commit once. No candidate or I0 route
is selected by M0. `FINALIZE0-PHI-SPLIT0-P0` is the sole next row.

### P0 implementation lock — tolerant analysis before strict verification

`verify_phi_edges_v1` is intentionally **not** P0's input preflight. It must
reject missing predecessor rows and non-dominating incoming values, while the
legacy repair's only legitimate purpose is to complete or rematerialize those
two bounded cases. P0 therefore owns this disconnected sequence:

```text
immutable MirFunction
-> PhiRepairInputAnalysisV1 (terminator-derived, tolerant only at the two repair seams)
-> PreparedLegacyPhiRepairCandidateV1 (non-Clone; no fresh ValueId; no mutation)
-> owned MirFunction clone execution in one deterministic schedule
-> terminator-derived cache rebuild
-> verify_phi_edges_v1(candidate)
-> explicit artifact-closure fixture
-> drop candidate
```

Duplicate/phantom predecessor rows, undefined or duplicate definitions,
unrepairable missing rows, cycles, non-rematerializable definitions, allocator
cursor collision/overflow, and exception-region PHIs reject in input analysis.
Missing rows are limited to the existing self-carried or exactly-one dominating
incoming laws. Rematerialization remains limited to Const, Copy, BinOp,
Compare, UnaryOp, Select, and an explicitly pure bounded substring Call. The
plan uses plan-local IDs only; fresh `ValueId`s are assigned inside the moved
candidate after a checked allocation budget.

Unused-Phi deletion remains blocked in P0. Its candidate span row is observed,
but no real `FunctionMetadata` closure exists yet. P0 may prove a deliberately
declared fixture closure for paired instruction/span removal and fake
positional/value artifacts; that proves the closure API's rejection laws, not
coverage of real metadata or production deletion. The P0 candidate has no
`commit_to_live` API, Builder/module/fact references, or production consumers.
It may claim helper-local isolation only, never module freshness, transient
fact completeness, caller conversion, or repair retirement.

### P0 closeout — disconnected deterministic candidate

`legacy_candidate.rs` now implements the locked P0 sequence with zero
production callers:

```text
immutable MirFunction
-> tolerant terminator-derived CFG view
-> duplicate-definition / allocator / exception preflight
-> non-Clone clone-owned candidate schedule
-> deterministic candidate-only rematerialization
-> terminator-derived cache rebuild
-> strict edge verification
-> drop candidate
```

The only rematerialized definitions are Const, Copy, BinOp, Compare, UnaryOp,
Select, and `RuntimeDataBox`/`StringBox` `substring` calls whose effect mask is
pure. The exact rejection matrix covers missing rows without one dominating
source, duplicate definitions, undefined late operands, cycles,
non-rematerializable definitions, impure substring calls, allocator cursor
collisions/overflow, and Catch/Throw regions. Every rejection fixture snapshots
the live function and proves unchanged instructions, spans, terminators, CFG
caches, cursor, signature, parameters, and metadata.

The strict verifier's P0 cases separately pin missing, phantom, duplicate,
undefined, non-dominating, unreachable, cache-drift, and deterministic
diagnostic behavior. A test-only fixture proves only the declared paired
instruction/span and fake positional/value closure; real metadata closure is
still absent, so unused-Phi deletion remains blocked.

Focused `edge_verifier` (7) and `legacy_candidate` (12) tests, `cargo check
-q`, format, diff, pointer, and 800-line checks are green. The candidate is
not a module transaction and exposes no live commit. Exactly three legacy
whole-function repair callers remain unchanged. The sole next frontier is
`FINALIZE0-PHI-SPLIT0-I0-SELECT`: select a module-owned transaction only after
its full artifact/fact/freshness closure is designed. CUT0 and all direct
caller conversions remain forbidden.

### I0-SELECT decision lock — Candidate M-prime

Candidate M-prime is selected as the **only eventual production integration**.
No function-only replacement and no direct-live repair is admitted.

```text
completed function drafts
-> one unpublished module-completion candidate
-> all candidate PHI repairs exactly once
-> fact/session closure
-> metadata snapshots and derived artifacts
-> post-publication coherence verification
-> one lifecycle commit
```

The existing `PreparedLegacyPhiRepairCandidateV1` remains the private
function-local engine. It has no live commit API. The eventual
`PreparedModuleCompletionCandidateV1` must own the complete candidate module,
every function that the batch can repair, candidate transient fact generation,
candidate diagnostic-origin observations, positional/site-indexed artifacts,
derived-artifact invalidation, and final verification input. It is non-Clone
and single-use. A one-function commit API is forbidden.

#### One repair position, not two

The current module lifecycle is stale by construction:

```text
transient type snapshot
-> first whole-function repair
-> module derived refresh
-> second all-functions repair
```

M-prime replaces both module repair calls only when one batch contains every
function-producing completion output, runs before every function metadata
snapshot and module-derived refresh, and no later pass mutates MIR before the
post-publication verifier. `condition_fn` is currently produced after the
first function insertion; it must either become a pre-batch draft or retain a
separate explicitly excluded completion law. The first I0 must not simply
replace the early call while retaining the late all-functions repair.

The intended final order is:

```text
physical completion producers
-> existing transient normalization / TypePipeline / Call-Await / return work
   // retained legacy authority; no retirement claim here
-> assemble all pending module function drafts
-> sorted candidate PHI preflight and repair batch
-> strict edge verification for every repaired function
-> fresh-rematerialized-value fact closure
-> function metadata snapshots
-> module aggregation
-> declaration and derived-artifact publication
-> final published-module coherence verification
-> lifecycle-only context close and one external commit
```

JoinIR is excluded. Its apply stage performs live block insertion, repair,
boundary-copy insertion, and RewriteContext mutation in one currently
non-atomic chain; it needs a separate `JoinIrApplyCandidateV1` row and cannot
borrow the module-completion transaction.

#### Rematerialized fact prerequisite

M-prime does **not** authorize the proposed fresh-value disposition projection
yet. Current `TypeContext.value_types` is an unbranded mutable observation,
not a source-definition sealed receipt. `value_kinds`, `value_origin_newbox`,
string/map/record facts, final metadata, `TypePropagationPipeline`, and
`metadata::propagate` cannot supply this authority. In particular, generic
BinOp, UnaryOp, Select, Copy, and substring Call lack retained producer
receipts. P0 correctly remains fact-free.

`REMATFACT0-D0` therefore precedes every production I0. Its only acceptable
enabling shape is a producer-time, function-generation-branded exact type
receipt plus a non-Clone, candidate-local projection that co-seals the source
receipt, verified rematerialization node, and fresh destination after physical
candidate emission. It may publish only an exact type into the candidate fact
session; Missing and Unknown publish nothing, and kind/origin/literal/map/
record transfer are separate parked owners. This is a temporary legacy
quarantine, not a second persistent ValueId map or an inference engine.

#### Fixed task order

```text
FINALIZE0-PHI-SPLIT0-I0-SELECT
  Candidate M-prime decision lock
  code delta = 0

-> FINALIZE0-PHI-SPLIT0-MODULETX0-S0
   private module-completion candidate vocabulary and owned-state boundary
   production consumers = 0

-> FINALIZE0-PHI-SPLIT0-REMATFACT0-D0
   select producer-receipt and candidate-fact-session authority
   code delta = 0

-> FINALIZE0-PHI-SPLIT0-REMATFACT0-S0/P0/I0/G0
   close fresh exact-type projection before any module repair connection

-> FINALIZE0-PHI-SPLIT0-MODULETX0-P0
   batch success/failure/freshness proof over every owned candidate surface

-> FINALIZE0-PHI-SPLIT0-I0
   one module-completion batch connection only; absorb both legacy module
   repair calls or reject before commit

-> FINALIZE0-PHI-SPLIT0-MODULE-G0
   module repair batch consumers = 1
   direct live repair consumers = 0
   unused-Phi deletion consumers = 0

-> MODULE-FINALIZE-VERIFY-CUT0
```

#### Required proof and stop law

`MODULETX0-S0` creates no production candidate, no Builder snapshot/restore,
no fact write, no metadata snapshot, and no repair consumer. `REMATFACT0-D0`
stops if it needs source/type re-inference, raw map observation as a receipt,
final metadata, type pipeline, name/runtime-tag recovery, Unknown copying,
origin/kind copying, `metadata::propagate`, or a persistent ValueId fact map.

`MODULETX0-P0` must prove that a failure in any function candidate, candidate
fact projection, metadata snapshot, derived refresh, or final verifier leaves
live functions, six transient TypeContext maps, caller-origin observations,
module metadata, derived artifacts, spans, allocator state, and lifecycle
close/commit unchanged. It does not claim whole-Builder rollback or compiler
reuse until FACTSESSION0 / FunctionSession owns those laws.

Unused-Phi deletion stays disconnected. The P0 fixture closure is not real
FunctionMetadata coverage. The module I0 stops rather than deleting any unused
Phi until exact positional and ValueId artifact closure is independently
proven.

#### MODULETX0-S0 closeout

`PreparedModuleCompletionCandidateV1` is now the one private, non-Clone S0
ownership product. It moves an already assembled `MirModule` together with all
six current transient `TypeContext` lanes, the two current diagnostic-origin
observation lanes, and one derived-artifact invalidation ledger. The product
has no Builder reference, live-state constructor, commit operation, PHI repair
operation, metadata publication, fact projection, or derived refresh. Its
facts are explicitly observations, not source-producer receipts, so the
product cannot authorize a fresh rematerialized `ValueId` publication.

The only production module completion path remains unchanged. The focused
candidate test proves the eight lanes and a multi-function module move as one
non-Clone boundary; format, cargo check, pointer guard, diff check, and the
under-800-line limit are green. `REMATFACT0-D0` is now the required design
frontier: it must select a producer-time receipt and candidate-local exact
fresh-value projection without treating any mutable fact map as that receipt.
