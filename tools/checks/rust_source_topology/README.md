# Rust Source Topology Check

Check-only Rust source parser for neutral, single-file syntax topology facts.

```text
one Rust source file
  -> syn parser
  -> items + ExprCall + ExprMethodCall
  -> neutral RustSourceTopologyV1 JSON
```

## Boundary

This crate owns syntax observation only. It does not own:

```text
FINALIZE0 entry families
semantic-operation classification
route or boundary policy
Cargo module inclusion
cfg evaluation
Rust name/type resolution
runtime observation
compiler/runtime behavior
```

Every call is therefore emitted with a typed unresolved reason in S0a.
Resolution, Cargo topology, and cfg profiles are later rows. Unsupported call
shapes must remain explicit; spelling heuristics are forbidden. Macro bodies
and external modules remain typed opaque S0a sites so an unparsed surface
cannot become a false zero-call result. The disconnected project layer may
additionally traverse the bounded literal module-position `include!` profile
described below; this does not change the S0a observation schema.

The tool is a standalone workspace so its parser dependencies do not enter the
root compiler workspace.

## Usage

```bash
cargo run --manifest-path tools/checks/rust_source_topology/Cargo.toml -- \
  single-file src/mir/compiler/mod.rs \
  --module-syntax-path hakorune::mir::compiler
```

Output is written to stdout. The syntax path, half-open byte range, source
slice, FNV-1a diagnostic digest, and callee syntax are neutral observations.
They are not semantic identity or resolution authority. Source reorder may
change report-local IDs and ranges.

## S0a guarantees

```text
parser-backed single-file item inventory
ExprCall / ExprMethodCall distinction
enclosing item syntax path/id
half-open byte range + source-slice digest
enclosing item and call-local cfg/cfg_attr syntax
typed unresolved projection for every observed call
typed opaque rows for macros, include!, and external modules
deterministic source-order JSON
```

## Disconnected project profile layer

`project` now owns the first disconnected S0b prerequisite:

```text
explicit profile request JSON
  -> structurally validated, deterministically ordered profile inputs
  -> pure three-valued cfg / cfg_attr decisions
```

The six initial inputs distinguish host dev, VM-reference, LLVM-harness,
wasm32 dev, unit-test library, and host release. Their expected activated
features are input assertions only. CARGO0 must compare them with Cargo's
actual feature closure before they become compile-unit evidence.

The cfg decision consumes an explicit environment and never reads Cargo,
rustc, process environment, source filenames, or FINALIZE0 policy. Unsupported
custom flags and unsealed target features return `Unknown`; malformed syntax
returns a typed error. The target matcher currently uses cfg-expr's built-in
target database. CARGO0 must add exact rustc/config fingerprints before any
repository inclusion claim.

`project::cargo` now owns the disconnected CARGO0-S0 seal:

```text
neutral Cargo metadata snapshot
  + exact selected manifest
  + validated profile request
  -> declared package/target/root-feature evidence
```

Selection is manifest-first and workspace-member exact; package names and
opaque Cargo PackageIds are never durable identity. Raw target kinds and crate
types remain in the evidence, while one bounded semantic-kind check prevents
library, binary, integration-test, example, build-script, and proc-macro roots
from being mixed. The Cargo resolve-node feature set is compared exactly,
including the literal `default` feature, and target `required-features` must
already be active.

This seal proves declarations and one metadata-run feature closure only. It
does not prove that Cargo compiled the target, that a build succeeded, or that
profile codegen, module inclusion, call resolution, or FINALIZE0 reachability
is active. The cargo_metadata process adapter, sealed rustc cfg probe, and
Cargo/config fingerprints are now owned by the disconnected CARGO0-M0 layer.

CARGO0-M0 runs `cargo metadata` with exact manifest, target filter,
package-qualified requested features, `--locked`, and `--offline`. It removes
cfg-affecting ambient Cargo/Rust flags and records a Cargo version digest. A
separate direct `rustc --print cfg` probe receives explicit target, test mode,
debug-assertion, panic, and Cargo-resolved feature arguments; its output and
rustc version are digested independently. Neither process is called a Cargo
build or actual unit-graph proof.

Manifest, Cargo.lock, repository Cargo config, and every discovered ancestor
or Cargo-home config are read and fingerprinted. Repository/external
rustflags are admitted only when the bounded classifier proves them
linker-only; cfg/profile/target-affecting settings reject. Cargo/rustc versions
and workspace fingerprints are checked before and after the observation so a
drifting tool or input cannot produce a sealed result. The final process
evidence serializes only workspace-relative paths and digests, never neutral
snapshot absolute paths or opaque Cargo PackageIds.

CARGO0-P0 owns two proof matrices. A dependency-free nested fixture workspace
seals default/no-default and asymmetric feature aliases across library,
required-feature binary, integration-test, host, and wasm32 targets. The root
matrix executes all six declared nyash-rust profiles twice and requires
byte-identical durable evidence with no absolute workspace path. These are
evidence tests only. CARGO0-G0 freezes this disconnected evidence boundary;
the project report/CLI remains disconnected until the full S0b-G0.

## Disconnected explicit-module traversal

MODULE0 consumes one sealed CARGO0 process-evidence row plus its exact
workspace capability and emits a separate `DeclaredModuleTopologyV1`.

```text
sealed Cargo package / target / features / rustc cfg
  -> explicit ItemMod declarations
  -> cfg gate before filesystem access
  -> inline / ordinary / literal-path module instances
  -> one S0a observation per loaded source occurrence
```

The traversal follows rustc's bounded directory ownership law. A custom Cargo
root starts at its physical parent; an ordinary `x.rs` puts later ordinary
children under `x/`, while `x/mod.rs` does not add another relative segment.
Inline modules advance the directory owner, and path-loaded external files are
treated as mod.rs-equivalent for their children. Literal path selection,
ordinary candidate selection, lexical containment, canonical containment, and
ancestor-cycle checks each have one owner.

`Excluded` declarations are recorded without probing a child. `Unknown`
declarations stop before probing and never appear in a successful product.
The graph is returned only after source and Cargo fingerprints are rechecked;
failure returns no partial graph. Sibling declarations may load the same
canonical file as distinct module instances, while an ancestor reuse rejects
as a cycle.

The first MODULE0 profile deliberately rejects reachable source-level inner
topology attributes and block-local modules. Their content-gate and block
identity laws require separate widening rows. The project CLI, semantic
resolution, entry-family policy, and compiler/runtime behavior remain
disconnected.

## Disconnected literal include traversal

INCLUDE0 extends the same project traversal with one ordered module-position
item vocabulary. An include edge adds a source occurrence to the exact
surrounding module; it never creates a module instance or a synthetic module
path.

```text
including source occurrence lexical parent
  -> literal include path
  -> included source occurrence in the same module
  -> included-file parent as the temporary child-module directory
```

This separation matters for inline modules: `include!("parts/items.rs")` is
resolved relative to the physical file containing the invocation, not the
inline module's ordinary-module directory. Conversely, `mod child;` inside the
included file starts from that included file's parent directory. Nested
includes repeat the same rule using their immediate source occurrence.

Each include edge owns its parent observation, optional parent include edge,
exact invocation range, cfg decision, literal/selected path, and optional child
observation. Included and excluded edges are explicit. Unknown cfg stops before
path interpretation; excluded cfg performs no path probe or read. Module and
include loads share one canonical ancestry stack, so ancestor reuse rejects
while sibling includes of the same file remain distinct occurrences.

The bounded profile accepts only an unqualified item/module-position
`include!` with one literal string and optional trailing comma. Expression,
statement/block, impl, trait, foreign, generated, nonliteral, absolute,
workspace-escaping, and ambiguous macro-identity forms fail with typed errors.
Included shebang/inner-attribute preambles also stop rather than being guessed.
The original S0a opaque include row remains available for source-site
correspondence; INCLUDE0 does not turn it into semantic call resolution.

## Stop lines

```text
no FINALIZE0 names or policy in this crate
no filename-based cfg classification
no alias/method/type inference
no general macro expansion
no guessed resolved def-path
no claim that syntax paths are Rust semantic def-paths
no active/excluded cfg or production classification
no Cargo/profile CLI consumer before CARGO0
no project CLI/report publication before S0b-G0
no generated, expression-context, or macro-ambiguous include expansion
no inner-cfg or block-local module guessing
no root workspace dependency change
no source/check file at or above 800 lines
```
