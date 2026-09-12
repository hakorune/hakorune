---
Status: Active — R7 AOT direct-harness child-env I1 Windows proof
Date: 2026-09-12
Decision: MIR-CALL-AOT-DIRECT-HARNESS-CHILD-ENV-I1
Parent: docs/development/current/main/investigations/mir-call-legacy-target-census-d0-2026-08-20.md
ProductionCaller: strict/dev selfhost and stage1 ingress already stopped; release compatibility remains
ReplacementCell: existing `MIR-CALL-LEGACY-READER-STOP-R0` terminal (landed)
---

# MIR-CALL-COMPATIBILITY-RETIRE-R7-D0

## Six-line brief

Decision: repair direct-harness option transport at the process boundary before the queued published-row owner design.
Source authority + canonical issuer: the existing AOT invocation captures the effective HAKO/NYASH pair; the existing compile command remains the sole child-command issuer.
Non-authority: MIR/Recipe meaning, public ABI names, profile labels, provider reachability, and test-only callers issue no semantics.
Fail-fast boundary: argument validation -> local capture -> bounded command/env-block construction -> child launch -> existing object/error terminal.
Smallest next slice: run the independent `child-env` smoke on Windows and close only after empty `NAME=` is observed in the child.
Non-claims: no published-row implementation, public ABI deletion, provider retirement, concurrency guarantee, backend parity, or aggregate R7/MirBuilder completion.

## Finite boundary and state table

```text
canonical/compatibility MIR ingress + compile profile
  -> LegacyCallV0 constructors/reissuer/readers/egress
  -> selected artifact reject or explicit compatibility terminal
```

Included: the current manifest's 243 production lexical rows across 127 files,
four compile-environment route anchors across four families, and the known
compatibility ingress families. Excluded: canonical typed `Call` definition,
ordinary tests, unrelated runtime/startup environment, runtime hook registry,
future VM/WASM parity, and test-only Loop-PHI residue except as inventory.

| State | Issuer/owner | Pre-effect terminal | Allowed action |
| --- | --- | --- | --- |
| `SelectedNative` | canonical typed call owner | named admission reject on legacy mix | retain typed route |
| `ExplicitCompatibility` | existing compatibility ingress | existing compatibility terminal | Stop or Promote only with caller proof |
| `MechanicalReissuer` | existing ID remapper | preserve IDs or named failure | retain until all callers close |
| `ReaderProjection` | existing serializer/backend projection | explicit compatibility output | replace/delete only at caller-zero |
| `EnvironmentRoute` | existing invocation/profile owner | restore or named failure | move to invocation-owned state in its own row |
| `TestOnlyInventory` | manifest observer | no production terminal | separate test cleanup decision |
| `UnclassifiedOrDrifted` | no authority | design stop | do not infer or delete |

## Acceptance for the next owner-unit row

Before implementation, the selected owner must have one source-backed issuer,
one real production caller set, one named terminal/consumer, and one exact
old-edge delete-set. Several callers may retain a shared owner: migration moves
the selected caller to the invocation state first, then deletes only its old
edge after replacement behavior is proven. The delete-set need not cover the
whole shared file or public ABI. Positive evidence covers the supported caller;
a one-point mutation negative identifies the owner-specific reject.

R7 aggregate closure requires all in-scope production writers, reissuers,
readers, re-entry paths, and environment routes to be caller-zero, with no
unconsumed replacement product. Until then, `LegacyCallV0` and shared
compatibility carriers remain live. Reopen this design if the manifest drifts,
a new non-test constructor/reissuer appears, a selected native reader consumes
legacy input, or an external caller cannot be assigned to an owner.

### Candidate comparison and first bounded owner

The follow-up audit compared the two remaining concrete candidates without
reopening the census:

| Candidate | Real callers / terminal | Exclusive delete-set | Disposition |
| --- | --- | --- | --- |
| `src/runner/mir_json_v0/module.rs` `boxcall` arm | `json_artifact::mir_loader`, `selfhost::stage_a_route`, `stage_a_compat_bridge`, and `stage1_bridge::stub_emit::parse`; compatibility parser currently produces `LegacyCallV0`, while strict/dev has a named pre-effect stop | not yet exclusive: release/v0 compatibility and strict/dev ingress share the parser and the `LegacyCallV0` carrier | **Select first as an existing M7-S reader-stop owner.** Keep release compatibility; stop only the strict/dev outer ingress with its existing terminal. Promote is not allowed because JSON `receiver`/`box_name` is not a source-backed typed issuer. Delete waits for release caller-zero. |
| `src/mir/joinir_id_remapper.rs` Legacy arm | merge/rewriter and test/reference callers only; no production terminal | no production delete-set and no semantic consumer | **Park as test/reference cleanup.** It is not a safe first M7-S production owner. |

The strict/dev outer-ingress Stop landed at `4e1d6f92fb`:
`stage_a_route`, `stage_a_compat_bridge` and stage1 stub callers reach
`selfhost::json::parse_mir_json_v0_line`, which rejects legacy boxcall before
the v0 module parser or mutation. Named-reject/release tests cover that edge;
release/v0 compatibility remains live until its own caller-zero retirement.

The follow-up candidate audit found no second safe M7-S production owner:
`joinir_id_remapper.rs` is test/reference-only, while the remaining release
`boxcall` callers share the parser and have no exclusive delete-set. Keep the
aggregate R7 row in design stop with
`NoSafeSlice__NoRemainingUnsharedM7SOwner`; reopen when a caller can be
isolated or a supported replacement is selected.

### Closed link ownership series

The private direct-link seam landed at `fc19313028`: v1 compatibility resolves
its archive once and v2 keeps the caller's explicit archive; both use the
existing AOT link body without FFI save/set/restore. Public AOT ABI and dlsym
remain explicit compatibility owners. Link success/failure and unset/empty/
present environment preservation passed. Compile-options ownership is a
separate responsibility; historical consultation detail remains in Git.

### Closed physical-options design

The versioned physical-options contract and Boundary switch landed at
`454e49b755`; Static, Rust compatibility, AOT Generic, public Generic and named
harness adoption are recorded below. The public borrowed contract, private
owned copy, admission ordering and retain boundaries are owned by
`docs/reference/abi/nyrt_c_abi_v0.md` and `lang/c-abi/README.md`.
Historical proposal structs and unopened design states remain in Git, not as
current execution authority. Published-row globals and external ABI retirement
are separate unresolved responsibilities; no concurrency guarantee is inferred.

### R7-I1A closeout (2026-09-12)

Implementation landed at `454e49b755` and is pushed on
`codex/birth-definition-publication`. The selected Boundary pure-first CAPI
caller now resolves one Rust-owned request adapter, passes a versioned
default-visible contract to `hako_llvmc_compile_json_with_options_v1`, and
lets `HakoLlvmcInvocation` own the copied C strings through lowering. The
existing generic, exact-seed, and indexOf pattern terminals consume that same
invocation option pointer; explicit unset `llc_flags` is an empty value and
cannot inherit ambient flags. The old environment-mutating three-argument
path remains only for unselected compatibility callers.

Observed acceptance:

- `CC=cc bash tools/build_hako_llvmc_ffi.sh` passed.
- `bash tools/checks/llvm_compile_options_contract_smoke.sh` passed for the
  generic fixture and an exact-seed fixture, including fake opt/llc tool and
  flag propagation, ambient-state preservation, explicit unset-flags, and
  pre-effect revision rejection.
- `CARGO_BUILD_JOBS=1 cargo check -p nyash-rust --features plugins -j1`
  passed with the repository's existing warning baseline.
- `bash tools/checks/current_state_pointer_guard.sh` and `git diff --check`
  passed.
- `llvm_hako_aot_ffi_admission_smoke.sh` remains a named baseline red: the
  existing named direct compatibility call still returns only the host
  `Traceback` marker. It is not counted as I1A evidence or a compiler
  regression claim.

Scope is closed only for this Boundary pure-first compile-options edge.
Static V2, explicit harness, AOT compatibility, public ABI retirement,
published-row globals, concurrent compilation, and aggregate R7 caller-zero
remain open follow-up rows. The next selection must name one such row before
editing it; no whole-R7 completion is claimed.

### Static V2 open I0 closeout (2026-09-12)

The selected static owner landed at `0bae5fdd9c` and is pushed. The existing
published-MIR Rust caller now passes the accepted physical-options contract to
`hako_llvmc_static_open_v2_with_options`; the old Rust settings save/set/
restore seam is deleted. C copies the contract into the existing invocation
owner, while the public three-argument static entry remains compatibility-only.
The focused session test covers explicit open/query/compile/close with fake
tool/path/flag propagation, ambient recipe isolation, pre-effect revision/
size/profile/flags rejection, and the existing malformed/cleanup order. The
host's LLVM14 causes the legacy LLVM18 object portion to report a skip; this is
not runtime or executable evidence. `mir_call_static_v2_open_contract_guard.sh`,
the Boundary options smoke, C build, Rust check, and pointer guard pass.

### Post-static owner audit (2026-09-12)

One independent read-only audit rechecked the remaining production caller set
after static I0. It found no unshared next owner-unit: Rust/C explicit harness,
generic/v1 AOT compatibility, public AOT link/dlsym, and residual C/AOT ambient
option readers retain shared callers or externally observable compatibility.
The physical contract currently admits only the Boundary/Static profiles and
`compat_replay=none`, so switching one compatibility subset would leave the
other readers and public re-entry without an exclusive delete-set.

Decision: keep `NoSafeSlice__NoRemainingUnsharedM7SOwner` and
`work_mode = design_stop`. The next design slice is a co-sealed owner/retain/
delete matrix for those remaining callers, with pre-effect contract/replay/
tool rejection, environment preservation, and explicit non-claims. No code,
fixture, route switch, new receipt, or fallback is authorized by this audit.

### Explicit-harness follow-up audit (2026-09-12)

A second independent read-only audit confirms that the explicit harness is not
an isolated next I0. The Rust explicit-harness route reaches the existing
`mir_json_to_object_llvmlite` provider, which also serves the ambient llvmlite
compatibility route; selecting the harness therefore does not produce a
provider caller-zero/delete-set. The C/AOT harness, generic/v1 AOT entry, and
public AOT link/dlsym entries likewise have distinct external or public
callers, so they cannot be coalesced by extending the current physical profile
alone.

The current physical contract still admits only Boundary/Static profiles and
`compat_replay = none`. Before any I0, the design must co-seal the owner,
terminal, retained callers, exact delete-set, and pre-effect rejection for
contract/replay/tool/symbol/input errors while preserving the AOT FFI split and
ambient environment behavior. Explicit harnesses must not fall back to the
generic route. Decision remains
`NoSafeSlice__NoRemainingUnsharedM7SOwner`; no code, fixture, route switch,
new receipt, or fallback is authorized.

### R7 retained-caller matrix (2026-09-12)

The production census is now fixed to four retained boundary groups. This is a
design inventory, not an implementation permission; the paths below are the
caller/terminal evidence that prevents a profile-only cutover.

| group | current owner and terminal | named callers / re-entry | retain/delete disposition |
| --- | --- | --- | --- |
| Rust codegen ingress | `compat_codegen_receiver.rs:52,96` -> `route.rs:179,186` -> `provider_keep.rs:95,127`; ordinary Boundary and named harness both end at the existing provider/object terminal | backend extern dispatch, loader-cold, hostbridge, global/externals; the same llvmlite provider is also selected by ambient keep | retain shared provider; no provider delete-set until the named harness and ambient keep have separate owners or one explicit shared contract |
| C compile and AOT compile | `hako_llvmc_ffi_route.inc:440,459,488`; `hako_aot_shared_impl.inc:585,601`; generic, pure-first, and named harness terminals remain distinct | AOT dlsym at `hako_aot_shared_impl.inc:413,434`, public C callers, and direct harness callers | retain all public symbols and the AOT FFI split; delete no compile export while dlsym/public callers remain reachable |
| AOT link | `hako_aot_shared_impl.inc:725-768` dispatches compatibility v1 and explicit v2 into the existing link body | `hako_llvmc_ffi_route.inc:357,372`, public `hako_aot_link_obj`, public `hako_aot_link_obj_v2`, and FFI dlsym re-entry | retain v1/v2 ABI and shared dispatch; no wrapper/body deletion until archive authority and `HAKO_AOT_USE_FFI` callers are co-sealed |
| ambient physical selectors | C common/AOT readers (`hako_llvmc_ffi_common.inc:65,112`, `hako_aot_shared_impl.inc:119,529`); the Rust compatibility transport was retired at `7350421205` | generic compatibility, AOT, subprocess/tool selection, and legacy external entry points | delete only caller-specific reads/temporary mutation after explicit invocation state reaches every remaining caller; link-only env and public compatibility remain retained |

The matrix's six-line decision is:

```text
Decision: keep the four groups as separate owner candidates; do not promote a profile-only harness cutover.
Source authority + canonical issuer: each existing caller's accepted physical request; C exports/dlsym consume it.
Non-authority: symbol names, ambient env, provider reachability, and public ABI names do not issue new semantics.
Fail-fast boundary: reject profile/recipe/replay/tool/symbol/input conflicts before env mutation, dlsym, or artifact effect.
Smallest next slice: accept one group-specific owner/terminal/delete row only after its retained callers and exact delete-set are named.
Non-claims: no shared-provider retirement, public ABI removal, AOT route switch, concurrent isolation, or LegacyCallV0 deletion.
```

This closes the finite design census but leaves the implementation gate open:
the current matrix has no non-empty exclusive delete-set. `work_mode` therefore
remains `design_stop`; a future I0 must name one of these groups and its
caller-specific delete-set in the same card before editing code.

The owner-selection consultation also records that AOT link is already closed
by the prior direct-seam I0 and is not a next candidate. The single remaining
cross-group gap is an invocation-owned physical compatibility admission that
can be consumed by the harness, generic/v1 AOT, and ambient-selector callers;
the current contract has only Boundary/Static issuers. Until that authority is
co-sealed with retained public/external assets and a non-empty caller-specific
delete-set, the R7 decision remains unchanged.

### MIR-CALL-RUST-CAPI-AMBIENT-RETIRE-I0 (selected 2026-09-12)

The focused owner audit opens one behavior-neutral Delete row inside the Rust
transport only. `CodegenRouteRequestV1::LegacyAmbientKeep` has zero current
constructors; the only production constructors are Boundary or Explicit
Harness. Every CAPI route validates first, Boundary requires
`pure-first/none`, and Explicit Harness returns before the generic CAPI probe.
Therefore the non-explicit branch in `capi_transport.rs` is caller-zero in the
current repository.

```text
Decision: delete the caller-zero Rust non-explicit CAPI transport branch.
Source authority + canonical issuer: existing explicit Opts route request and its physical-options admission.
Non-authority: public C exports, AOT dlsym, llvmlite provider selection, and external ABI names are retained consumers.
Fail-fast boundary: route validation and explicit-options admission remain before dlsym, env mutation, and artifact effect.
Smallest next slice: remove the old Rust branch/selector and update its route census/guard in one I0.
Non-claims: no public C ABI removal, AOT or provider retirement, concurrent isolation, or aggregate R7 closure.
```

The exact old-edge delete-set is the three-argument dlsym plus environment
save/set/restore in `capi_transport.rs`, the `compile_symbol` plumbing used
only by that branch, `LegacyAmbientKeep`, the Rust-side
`compile_symbol_for_keep_recipe`/unused symbol constants and tests, and the
caller-zero Rust `ny-llvmc` provider helper. Retain
`compile_via_capi_with_options`, `hako_llvmc_compile_json_with_options_v1`,
the public generic/pure-first C exports, AOT dlsym, and the explicit llvmlite
provider helper. The separate `ny-llvmc` crate/driver remains the mainline
Boundary owner; this row removes only the old host-provider subprocess helper.
The existing route-identity guard and llvmlite production census must be
changed from expecting the old branch to proving its absence and classifying
the route as caller-zero.

I0 acceptance is: Boundary `pure-first/none` still reaches the explicit
options entry; Explicit Harness still bypasses CAPI and reaches the named
provider; invalid recipe/replay still rejects before effects; missing CAPI
still returns the existing unavailable terminal; the updated route guard,
census guard, source-size bound, `git diff --check`, and focused Rust route
tests pass. This row retains all C/AOT/public callers and does not claim R7
wide retirement.

### Rust ambient transport I0 closeout (2026-09-12)

The caller-zero Rust non-explicit CAPI branch is retired. The typed
`BoundaryPureFirst` request now reaches only
`compile_via_capi_with_options`; `ExplicitHarnessCompat` reaches the existing
llvmlite provider directly, and no Rust route selects the removed
`ny-llvmc` subprocess helper or mutates compile-recipe/replay environment
variables. Public C exports, AOT dlsym, the separate `ny-llvmc` crate/driver,
and the explicit llvmlite compatibility provider remain retained consumers.

Observed acceptance:

- focused Rust route tests: 5 passed, 0 failed;
- `cargo check -p nyash-rust --features plugins --profile quick -j1`: passed
  with the repository warning baseline and no new `provider_keep` warning;
- `llvm_codegen_route_identity_guard.sh`: passed;
- `llvm_llvmlite_production_census_guard.py`: passed (`rows=37`,
  `automatic_python_ingress=0`, `native_retry=0`, `keep_roots=26`);
- `current_state_pointer_guard.sh`, JSON validation, and `git diff --check`:
  passed.

The host preflight still reports Ubuntu 22.04 with LLVM14.0.0 only, so no
LLVM18 object/EXE runtime evidence is claimed. The follow-up environment task
`LLVM18-TOOLCHAIN-INSTALL-I0` is registered at
`docs/development/current/main/investigations/llvm18-toolchain-installation-task-2026-09-12.md`.
It uses the existing CI `apt.llvm.org` recipe and keeps LLVM14 side-by-side.

This closes only the Rust caller-zero transport row. The remaining C/AOT,
public, harness, provider, and aggregate R7 rows remain open by design.

### Post-Rust-I0 owner consultation (2026-09-12)

The read-only follow-up audit rechecked the remaining production callers after
the Rust transport closeout. It found no new standalone owner: C/AOT compile
and public dlsym callers share public compatibility surfaces, while explicit
harness and llvmlite/provider callers share retained provider/entry behavior.
Their exclusive delete-sets are empty under the current physical contract.

Decision remains `NoSafeSlice__NoRemainingUnsharedM7SOwner`; keep
`work_mode = design_stop`. The next design slice must co-seal one remaining
owner, terminal, retained callers, exact delete-set, and pre-effect rejection
for contract/replay/tool/symbol/input errors before any new implementation.
This audit did not edit code, add fixtures, add receipts, or run Cargo.

### Shared invocation-owned compatibility admission D0 (selected 2026-09-12)

The second independent read-only design consultation accepts one next design
slice: define a shared invocation-owned physical compatibility contract for
the remaining callers. This is a design stop, not an implementation grant.

```text
Decision: co-seal one invocation-owned physical compatibility contract before selecting another R7 implementation row.
Source authority + canonical issuer: each existing explicit physical entry request, normalized once by one admission adapter; C consumes a deep-copied HakoLlvmcInvocation.
Non-authority: MIR/Recipe meaning, profile names alone, ambient environment, provider reachability, public ABI names, and test-only emitters issue no new semantics.
Fail-fast boundary: entry/profile -> revision/size/flags -> recipe/replay/provider/tool/alias -> input/path/re-entry -> contract copy -> child/provider/lowering -> artifact publication.
Smallest next slice: fix a finite issuer -> terminal -> retained-caller/delete-set matrix and its pre-effect rejection obligations for every remaining compatibility group.
Non-claims: no code, route switch, fallback, public ABI deletion, concurrent guarantee, LLVM18 runtime evidence, semantic receipt, or R7 completion.
```

The design boundary is finite and keeps the existing terminals:

| issuer / caller group | current terminal | retained surface | design obligation |
| --- | --- | --- | --- |
| Rust `compat_codegen_receiver` and stage1/boundary compatibility | C options entry or explicit llvmlite provider | Rust route, `boundary_driver_ffi`, llvmlite keep | distinguish Boundary, explicit harness, and provider ownership without ambient retry |
| C generic/pure-first and named harness | `hako_llvmc_ffi_route.inc` terminals | public C compile exports and existing C lowering | carry explicit invocation options while retaining public symbols |
| generic/v1 AOT and AOT dlsym | `hako_aot_shared_impl.inc` compile/link terminals | AOT v1/v2, dlsym, runtime archive/link split | preserve public re-entry and classify link-only environment separately |
| C common/AOT ambient selectors | existing tool/provider child selection | compatibility subprocess/tool behavior | isolate invocation-owned options without parent-process mutation or fallback |

### Field-level source crosswalk (read-only, 2026-09-12)

The source audit fixes the missing design at field level; a profile name alone
does not establish ownership.

| field | generic public / Boundary | explicit harness / AOT / link | disposition for the next design |
| --- | --- | --- | --- |
| entry/profile | `hako_llvmc_ffi_route.inc:460-477` builds Generic; `capi_transport.rs:13-26` carries Boundary | `hako_llvmc_ffi_route.inc:508-529` requires named harness; AOT Generic uses profile 0 | entry-owned; no profile-only merge |
| recipe/replay | physical copy requires pure-first/none; ambient harness replay is rejected (`route.inc:439-458`) | named harness uses pure-first/harness; AOT checks before dlsym (`aot_shared_impl.inc:546-580`) | retain exact rejects; no ambient fallback |
| opt/tool/llc flags | Generic contract is assembled at `route.inc:419-436`; Boundary Rust captures its contract (`capi_transport.rs:38-72`) | AOT Generic captures effective values before its C call (`aot_generic_ffi_compile.inc:35+`) | invocation-owned copy; default-flag difference remains unresolved policy |
| `llvmc_path` | non-harness is NULL and rejected by physical copy (`physical_options.inc:72-87`) | named harness requires a non-empty path (`physical_options.inc:56-70`); AOT owns child/dlsym entry | keep harness boundary distinct |
| `HAKO_CAPI_TM` / dlsym | legacy TargetMachine probe is an explicit compatibility keep (`pure_compile_legacy_capi_emit.inc:1-7`) | public C, AOT v1/v2, and named-harness dlsym remain externally re-enterable | pre-effect validation plus external retention; no deletion claim |

This crosswalk creates no new authority. It proves that the missing design is
one owner/terminal/delete-set co-seal: until a row names a non-empty
caller-specific delete-set, remain `NoSafeSlice__NoRemainingUnsharedM7SOwner`
and keep `next_execution_card = none__R7NextOwner__DesignStop`.

### AOT direct-harness environment premise audit (pre-I1, 2026-09-12)

This subsection records the pre-fix I0 source; the I1 review repair below is
the current contract.

The exact-source check found a caller-specific old edge, but not yet a safe
deletion. `hako_aot_ensure_default_opt_env` is called only by
`hako_aot_compile_json_direct_harness` (`hako_aot_generic_ffi_compile.inc:8-15`,
`hako_aot_shared_impl.inc:517-544`), which launches the named child through
`system()` (`hako_aot_shared_impl.inc:139-156`). The child contract is not a
no-op: Rust `harness_driver::propagate_opt_level` gives `NYASH` precedence
(`harness_driver.rs:50-62`), while the Python harness defaults to O2 when both
values are absent (`src/llvm_py/build_opts.py:19-40`). The current helper
therefore changes both the child input and the parent process state.

| parent environment | current child selection | unresolved design obligation |
| --- | --- | --- |
| both unset | helper injects HAKO=0 and NYASH=0; child emits O0 | pass the same O0 to the child without parent mutation |
| HAKO only | helper adds NYASH=0; child selects O0 | preserve observed alias precedence or settle a new policy |
| NYASH only | helper adds HAKO=0; child selects NYASH | preserve empty-value behavior as well as nonempty values |
| both present | child selects NYASH, including its empty-value case | define conflict/empty handling before shell/process transport |

Decision: keep the R7 design stop. The non-empty delete-set is the helper and
its two parent `setenv` calls, but implementation requires one child-only
environment admission (including POSIX/Windows execution, pre-effect command
failure, nested invocation, and no parent-state mutation) plus an accepted
unset/empty/present policy. A helper deletion alone is not behavior-neutral;
the current AOT named-harness and child settings remain retained. The worker
consultation for this premise was `pending/cancelled` after the available
wait budget and produced no conclusion; that is not rejection evidence.

### MIR-CALL-AOT-DIRECT-HARNESS-CHILD-ENV-I1 (review repair, 2026-09-12)

The review found a real Windows defect in I0: `set "NAME="` deletes the
variable, so present-empty and unset were not distinct. I1 changes only the
process-launch boundary. The invocation captures `HAKO = inherited HAKO or
"0"` and `NYASH = inherited NYASH or "0"` once. POSIX retains the existing
quoted command prefix; Windows copies the inherited environment block,
replaces or inserts canonical `NAME=value` entries, and launches the existing
`cmd.exe /C` command with `CreateProcessA`. An empty value is therefore passed
as `NAME=` and is never expressed as a `set` command. MIR, Recipe, public AOT/C
ABI, provider ownership, and child semantics are unchanged.

The existing AOT admission smoke now runs the child-env probe first and accepts
`all` (default) or the independent `child-env` mode. It covers both-unset,
HAKO-only, NYASH-only, both-present, both-empty, parent preservation, and the
no-child command-construction negative. On this Linux host, the C build,
route guard, and standalone `child-env` mode pass. The default smoke reaches
the known named-direct compatibility red afterward because this host's Python
has no `llvmlite`; that is baseline environment debt. Windows runtime proof is
still pending on a Windows host, so I1 remains open until that command is run.

Separate informational census red: `mir_r7_legacy_census_manifest.py` currently
reports its pre-existing `capi_transport.rs:247` anchor drift (the source line
is now the `CStr::from_ptr` error conversion). It is outside I1 and was not
regenerated as part of this process-boundary repair.

No public ABI removal, harness/provider retirement, recipe/replay change,
process-wide concurrency guarantee, backend parity, LLVM18 evidence, or
aggregate R7 completion is claimed.

The canonical issuer is the existing physical request at each explicit entry;
the shared adapter may normalize its transport fields once, but it must not
issue a new MIR or Recipe product. C-side `HakoLlvmcInvocation` remains the
copy/lifetime owner. All public C ABI, AOT v1/v2, dlsym, `ny-llvmc --driver
harness`, llvmlite/provider keep, and existing C lowering remain retained until
a later row names a real caller-specific delete-set.

The counterexample blocking profile-only routing is fixed: Rust explicit
harness reaches the llvmlite provider, while
`hako_llvmc_compile_json_compat_harness` reaches `ny-llvmc --driver harness`
and AOT dlsym can re-enter a third owner. The shared `harness` label therefore
does not identify a physical terminal.

Design acceptance is a source-backed finite table covering every listed issuer
and terminal with `fallback=none`, explicit retained/delete disposition, and
pre-effect negatives for profile/replay/provider/tool/alias conflicts,
unset/empty/present environment preservation, nested invocation, and dlsym
re-entry. Only after that table contains a non-empty caller-specific delete-set
may `work_mode` leave `design_stop`. Until then the decision is
`NoSafeSlice__NoRemainingUnsharedM7SOwner`.

### MIR-CALL-NY-LLVMC-BOUNDARY-COMPILE-OPTIONS-I0 (selected 2026-09-12)

The focused source audit found one real production caller with an exclusive
delete-set: `ny-llvmc`'s default `DriverKind::Boundary` reaches
`boundary_driver_ffi.rs` through `boundary_driver.rs`, and no other crate
caller enters that Rust helper. The existing C
`hako_llvmc_compile_json_with_options_v1` ABI already owns the physical
options copy/lifetime, so this is an in-place transport replacement rather
than a new semantic or settings authority.

```text
Decision: switch the ny-llvmc Boundary compile caller to the existing versioned physical-options C entry.
Source authority + canonical issuer: the Boundary CLI's one captured physical request, encoded by a private repr(C) ABI mirror of the tracked C contract.
Non-authority: MIR/Recipe meaning, public C symbol names, AOT dlsym, explicit harness/native drivers, and environment mutation do not issue new semantics.
Fail-fast boundary: capture/validate recipe, replay, alias, level, and C layout before dlsym or lowering; C repeats contract, tool, input, and artifact checks.
Smallest next slice: remove Boundary Rust three-argument symbol selection/env override and pass the existing pure-first/none contract with explicit tool values.
Non-claims: no public C ABI removal, AOT/provider retirement, concurrent guarantee, LLVM18 runtime evidence, new receipt/settings layer, or R7 completion.
```

Exact delete-set: `CompileFn`, the three-argument `hako_llvmc_compile_json{,_pure_first}`
lookup, `boundary_compile_symbol` and its constants/tests, the Rust
`with_env_override` helper and its test, and the Boundary caller's temporary
recipe/replay environment mutation. Retain the existing public C generic,
pure-first, and harness exports; C/AOT dlsym; the C lowering; `link_obj_v2`;
and explicit `--driver harness/native` routes. The private Rust `repr(C)` row
must match the tracked `hako_llvmc_physical_contract_v1` layout exactly and
must not become a second authority.

Acceptance is: Boundary normal and dummy object emission pass through the
options symbol; pure-first/none is the only admitted Boundary pair; conflicting
recipe/replay or `HAKO_CAPI_PURE` rejects before library/symbol/lowering
effects; revision/size/profile/flags/tool/input failures retain the existing C
diagnostics; environment values remain unchanged; and public C/AOT/dlsym
callers remain source-covered. The implementation is one Boundary caller
switch plus old-edge retirement, with no fallback or retry.

### Boundary compile-options I0 closeout (2026-09-12)

Implementation landed in commit `36d7fc784f`. The production `ny-llvmc` Boundary caller now reaches the existing versioned
options entry with one invocation-owned Rust row. The old Rust three-argument
lookup, symbol selector, and process-environment save/set/restore path are
absent; public C generic/pure-first/harness exports, AOT dlsym, explicit
harness/native drivers, and `link_obj_v2` remain.

Observed acceptance:

- targeted Rust Boundary tests: 3 passed, 0 failed;
- `cargo check -p nyash-llvm-compiler --profile quick -j1`: passed;
- production Boundary normal and `--dummy` object emission passed with
  explicit fake opt/llc tools; the trace named
  `hako_llvmc_compile_json_with_options_v1` and both artifacts were written;
- Boundary recipe, replay, `HAKO_CAPI_PURE`, and opt-level alias conflicts
  rejected with no artifact;
- `llvm_compile_options_contract_smoke.sh`,
  `llvm_codegen_route_identity_guard.sh`,
  `llvm_llvmlite_production_census_guard.py`,
  `mir_r7_legacy_census_manifest.py`, and the current-state pointer guard:
  passed;
- the broader crate run was `50 passed, 2 failed`: one is the pre-existing
  LLVM14 opaque-pointer failure, and the other is the existing direct public
  pure-first fixture's `CheckedCallOut` metadata mismatch; neither enters the
  changed Rust Boundary caller and neither is claimed as an I0 regression;
- the workspace-wide format check still reports unrelated baseline drift;
  the two changed Rust sources pass targeted rustfmt checks.

This closes only `MIR-CALL-NY-LLVMC-BOUNDARY-COMPILE-OPTIONS-I0`. The next
R7 decision stop must select the remaining shared compatibility owner and a
non-empty caller-specific delete-set. At this pre-installation checkpoint,
LLVM18 was an environment-only task blocked by external sudo permission; no
LLVM18 object/EXE runtime evidence was claimed.

### LLVM18 environment follow-up (superseded)

The external sudo blocker is resolved. Current tool and witness evidence is
in `llvm18-toolchain-installation-task-2026-09-12.md` and the closeout below.

### MIR-CALL-AOT-GENERIC-COMPILE-OPTIONS-I0 (selected design 2026-09-12)

Two independent read-only worker audits selected the remaining AOT Generic
caller as the next bounded owner. The source-backed exclusive edge is
`hako_aot_try_ffi_compile` in `lang/c-abi/shims/hako_aot_shared_impl.inc`:
it alone dlsyms the old three-argument `hako_llvmc_compile_json`. The existing
versioned `hako_llvmc_compile_json_with_options_v1` contract and the
invocation-owned C options copy are the physical owner; no new ABI, receipt,
settings layer, or semantic product is introduced.

```text
Decision: route AOT Generic FFI compile through the existing versioned physical-options C entry.
Source authority + canonical issuer: the AOT Generic physical request, encoded once as the tracked C contract with ingress profile GENERIC_COMPAT(0).
Non-authority: MIR/Recipe meaning, profile labels alone, ambient environment, public ABI names, named harness, link dlsym, and tests issue no new semantics.
Fail-fast boundary: AOT args/FFI admission -> recipe/replay/pure/level/tool validation -> options symbol -> revision/size/profile/flags/tool copy -> JSON/invocation -> lowering -> artifact.
Smallest next slice: accept profile 0 in the shared options contract, pass that profile into the existing invocation, and delete only AOT Generic's old compile dlsym/type/call edge.
Non-claims: no public old-symbol removal, no named-harness or link change, no fallback/retry, no provider retirement, no LLVM18 evidence, and no R7/MIRBuilder completion.
```

Finite outcome and ownership boundary:

| issuer / input | terminal | retained / deleted | pre-effect obligation |
| --- | --- | --- | --- |
| AOT Generic + FFI + `pure-first/none` | existing generic C lowering or its typed error | accept; delete only old AOT compile dlsym edge | reject invalid args, FFI mode, recipe, replay, `HAKO_CAPI_PURE`, level, tool, and alias before dlsym |
| AOT Generic without FFI | existing `aot-compat-admission-required` terminal | retain; no fallback | do not invoke child or old public compile symbol |
| AOT Generic invalid contract/profile 3/unknown | existing compile-options contract reject | reject; retain profile 1/2 strict behavior | revision, size, profile, flags, recipe, replay, and tool checks precede JSON/lowering |
| named AOT harness | existing named harness provider/export | retain; no shared-profile change | keep direct harness route and its explicit compatibility admission |
| public C generic/pure-first/harness and link v1/v2 | existing public C/link owners | retain; out of scope | source coverage remains; no AOT compile/link re-entry is added |

The exact implementation obligations are: `hako_llvmc_physical_options_copy`
accepts physical profiles 0/1/2 and rejects 3/unknown; the options compile
entry maps the validated contract profile to the existing invocation profile
(0 Generic, 1 Boundary; Static remains a separate retained profile owner);
and AOT Generic constructs revision 1, `sizeof`-sized, flags-zero,
`GENERIC_COMPAT(0)`, `pure-first/none` contract data without mutating the
parent environment. AOT must carry the effective opt level, opt/llc paths,
and llc flags; a null llc-flags value is an explicit empty value in this
contract and must not silently restore the old ambient default. The AOT-side
`HAKO_CAPI_PURE` rejection remains before `dlopen`/`dlsym` because the old
generic export previously owned that admission.

Exact delete-set: the private AOT generic three-argument typedef, its
`dlsym(h, "hako_llvmc_compile_json")`, the private call, and the associated
missing-symbol diagnostic. Retain public
`hako_llvmc_compile_json{,_pure_first,_compat_harness}`, AOT v1/v2 and named
harness exports, Boundary/Static options, link dlsym, and direct harness
environment handling. No fallback or retry is permitted.

Design evidence: Rawls audited the finite AOT caller/delete-set matrix;
Gibbs independently audited profile-0 plumbing and confirmed that merely
allowing profile 0 is unsafe unless the contract profile reaches invocation
initialization and AOT preserves the pure/replay admission. Both audits were
read-only with no Cargo, fixture, or worktree changes. The implementation may
start in `fast` mode only after this Decision is recorded.

### AOT Generic compile-options I0 closeout (2026-09-12)

Implementation is complete in the bounded C owner set. AOT Generic FFI now
constructs the existing revisioned contract with Generic profile 0 and calls
`hako_llvmc_compile_json_with_options_v1`. The shared options copy admits
profiles 0/1/2 for their existing owners; the JSON compile entry maps only
Generic 0 and Boundary 1 into invocation profiles, while Static 2 remains on
its dedicated V2 entry and Explicit Harness 3 rejects. The old private AOT
three-argument compile typedef, dlsym, call, and missing-symbol diagnostic are
deleted. Public C compile exports, AOT named harness, AOT v1/v2, Boundary and
Static options, and link dlsym remain.

Observed acceptance:

- `bash tools/build_hako_llvmc_ffi.sh`: passed;
- `llvm_compile_options_contract_smoke.sh`: passed; direct Boundary and
  Generic profile paths consumed explicit fake opt/llc settings, AOT Generic
  produced an object through the options symbol, and recipe/replay,
  `HAKO_CAPI_PURE`, revision, and Explicit Harness profile rejects produced no
  artifact;
- `llvm_codegen_route_identity_guard.sh`,
  `llvm_llvmlite_production_census_guard.py`,
  `mir_r7_legacy_census_manifest.py`, `current_state_pointer_guard.sh`, and
  `git diff --check`: passed;
- source-size bound: `hako_aot_shared_impl.inc` is 742 lines and the new
  `hako_aot_generic_ffi_compile.inc` is 105 lines, both below the 800-line
  hard stop. The new include is part of the shared AOT source truth.

The object evidence uses fake tools and therefore claims route/contract
reachability only; LLVM18 is still unavailable on this host, so no LLVM18
object/EXE runtime claim is made. This closes only
`MIR-CALL-AOT-GENERIC-COMPILE-OPTIONS-I0`; public C, named compatibility,
provider, aggregate R7, backend parity, and whole-MIRBuilder completion remain
open by design. The next R7 decision stop must select another finite owner
with a non-empty delete-set.

### Public C Generic compile ingress disposition (superseded)

The pre-bridge Stop/retain proposal was superseded by the accepted options
bridge below, landed at `3d3b118ccf`. Git retains the earlier audit and matrix.

### Residual R7 owner selection audit (design stop 2026-09-12)

Locke independently rechecked the remaining non-public-Generic candidates
after the AOT Generic closeout and public Generic disposition. No
source-visible production owner with a non-empty exclusive delete-set remains
inside the active Call/R7 boundary.

| issuer / owner | terminal | caller classification | disposition |
| --- | --- | --- | --- |
| recipe-aware public pure-first C export | ambient replay reject or existing pure compile result | public ABI; repository direct uses are test-only | retain export and shared lowering; delete-set empty |
| named C harness and named AOT harness | tool/child/object terminal | AOT dlsym re-entry plus public/test callers | retain export, dlsym, and child; delete-set empty |
| ExplicitHarnessCompat -> llvmlite provider | provider shape/status/object terminal | hostbridge, loader-cold, Stage1, and ambient keep callers share provider | retain provider/script; delete-set empty |
| legacy CAPI TargetMachine probe | optional probe, then existing opt/llc path | shared by Generic, pure-first, AOT/Static V2 and direct core tests | retain explicit HAKO_CAPI_TM compat-probe keep; shared fail-fast design is unresolved |
| link v1/v2 and AOT link dlsym | existing link validation/failure/executable terminals | multiple production callers and public ABI | retain link body, public ABI, and dlsym; delete-set empty |

Census boundary: the remaining Call/R7 compatibility entrypoints from the
public C/AOT/harness/provider/link ingress through their existing terminal
errors or artifact publication; includes source-visible production callers,
public re-entry, test-only direct core users, and the explicit HAKO_CAPI_TM
keep; excludes caller-zero helper cleanup outside Call/R7, LLVM18 installation,
and unselected backend parity.

The exact shared gap is an invocation-owned compatibility admission that can
be consumed by the retained Generic/pure-first, harness, AOT, and ambient
selector callers without inventing a second semantic authority. The current
options contract cannot be widened by profile alone: public Generic preserves
ambient tool fallback and unknown-replay behavior, while the legacy
TargetMachine probe is explicitly retained by the existing task SSOT. Therefore
the correct state remains
NoSafeSlice__NoRemainingUnsharedM7SOwner; no code, fixture, route switch,
fallback, or new receipt is authorized.

Smallest next slice: co-seal the shared invocation-owned compatibility
admission's issuer, field-by-field terminal matrix, retained public/external
callers, and a real caller-specific delete-set. Until that design exists,
next_execution_card = none__R7NextOwner__DesignStop is intentional and the
goal remains active rather than claiming R7 or MIRBuilder completion.

Design evidence: Locke's audit was read-only and ran no Cargo, fixture, or
receipt-producing test. The direct HAKO_CAPI_TM source check also found the
existing task-pack contract that marks it as an explicit bypass/compat-probe
keep, so it is not silently reclassified as dead code.

### Public C Generic options bridge (design accepted 2026-09-12)

The second read-only audit conditionally approved a bounded bridge for the
public three-argument Generic C export. The condition is recorded here: the
bridge must preserve the legacy first-character interpretation of the opt
level, and the private delete-set must land in the same slice.

```text
Decision: retain the public Generic ABI and route its private body through the existing invocation-owned options entry.
Source authority + canonical issuer: public C alias/recipe/replay admission plus existing resolver helpers; the adapter issues no new semantic product.
Non-authority: profile labels, ambient values after capture, HAKO_CAPI_TM, named harness, AOT/link exports, and tests.
Fail-fast boundary: alias -> pure-first recipe -> replay=harness reject -> effective compatibility capture -> existing options copy -> JSON/lowering/artifact terminal.
Smallest next slice: replace the private Generic call with profile-0 options, delete the caller-zero Generic wrapper, and add public-ABI compatibility smoke/guards.
Non-claims: no public ABI retirement, no legacy CAPI probe removal, no process-global concurrency guarantee, no LLVM18 evidence, and no R7/MIRBuilder completion.
```

Finite compatibility matrix:

| issuer / input | terminal | disposition | pre-effect obligation |
| --- | --- | --- | --- |
| public Generic, alias unset | existing alias admission | retain reject | check `HAKO_CAPI_PURE` before contract construction |
| recipe unset/non-`pure-first` | `generic-capi-recipe-required` | retain reject | perform the existing recipe gate first |
| replay exactly `harness` | `generic-capi-compat-admission-required` | retain reject | do not enter JSON/lowering or named replay implicitly |
| replay unset/empty/unknown | existing no-replay Generic terminal | capture `none` | only exact `harness` is an ambient replay admission today |
| opt level with first byte `0..3` | existing mem2reg/O0..O3 behavior | capture one-character value | preserve legacy first-character semantics (`1x` remains `1`) |
| opt level with another first byte | existing effective O0/mem2reg behavior | capture `0` | pass only a contract-valid one-character level |
| configured tool exists | existing opt/llc terminal | capture resolved path | use `hako_llvmc_resolve_tool`, preserving invalid-configured-path fallback |
| configured tool missing, default tool absent | existing missing-tool terminal | retain null resolution | options copy, then lowering, must reject before child/artifact |
| llc flags unset/empty | existing `-O3 -mcpu=native` behavior | capture effective default | never pass explicit NULL and accidentally erase the default |
| `HAKO_CAPI_TM=1` | retained legacy probe/fallback terminal | retain explicit compat probe | no new TargetMachine or probe authority is introduced |
| pure-first export, named harness, AOT, Static V2, Boundary, link | existing neighboring terminal | retain | no profile coercion or cross-lane re-entry |

The exact issuer/terminal ownership is:

| issuer | terminal | retain / delete |
| --- | --- | --- |
| public `hako_llvmc_compile_json` -> Generic profile-0 options -> `compile_doc_compat_pure` | admission, options copy, JSON/lowering, legacy probe or opt/llc, object/error | retain public ABI, admission, options owner, and shared core; delete private `compile_json_compat_pure` and its route prototype/call edge |
| public `hako_llvmc_compile_json_pure_first` -> Boundary profile | existing pure-first object/error | retain all existing Boundary/profile helpers |
| named C/AOT harness, provider, Static V2, link | existing named/Static/link terminals | retain; outside this delete-set |

The adapter is synchronous and invocation-local after the existing options
copy. It does not mutate the environment and does not claim isolation from
another thread changing process-global environment variables. The existing
`HAKO_CAPI_TM` probe remains a deliberate shared compatibility keep, so it is
not part of this retirement slice.

Design evidence: Erdos returned a conditional approval after checking the
include order, options-copy pre-effect boundary, unknown replay behavior,
tool fallback, legacy probe, neighboring callers, and the private caller
graph. No files, Cargo, fixtures, or receipts were changed by the audit.

### MIR-CALL-PUBLIC-C-GENERIC-OPTIONS-I0 closeout (2026-09-12)

`3d3b118ccf`: public Generic retains its ABI/admission and consumes profile-0
options; the private `compile_json_compat_pure` wrapper/prototype/edge were
removed. Boundary/shared core, HAKO_CAPI_TM, harness, Static V2 and link remain.
C build, options smoke (unknown replay, first-character level, flags and
pre-artifact rejects), route/census/static/pointer/manifest guards passed.
ASan drivers compiled; their pre-install LLVM14 runtime evidence did not
prove LLVM18 execution. Current LLVM18 evidence follows below.
This closes the bounded bridge only; aggregate R7 remains open.

### LLVM18 installation and witness classification (2026-09-12)

`161aff97f6`: LLVM18.1.8 checks passed; initial G0 EXE at `3d3b118ccf` rejected
at site-missing. Commands remain in `llvm18-toolchain-installation-task-2026-09-12.md`;
latest acceptance is in `mirbuilder-loop-g0-helper-backend-reach-i1-2026-09-12.md`.

Warning follow-up remains with `MIRBUILDER-WARNING-SURFACE-CENSUS-R0`;
its current observation is lib=1793 / lib-test=523 (255 duplicates).

### G0 acceptance dependency: physical ABI and CFG coverage

Boundary: selected physical program -> ABI issuance/JSON -> C V4 admission
and emission; includes root, called helper, every block and terminator;
excludes unselected backends and source-shape expansion.
Accepted audits: Pauli (empty sites), Popper (kind equality, exact-state CFG,
shared incoming labels) and Kierkegaard (matched-dispose runtime reuse).
Durable contract: `docs/reference/abi/nyrt_c_abi_v0.md#selected-lifecycle-physical-program-v2`.
Existing physical-v2 SSA/dominance remains sole; canonical_cfg already issues
G0 edges without arguments. No C source reconstruction or bounded-memory claim.
Implementation and source-called EXE3 evidence are recorded below.

### Empty checked-site admission (verified 2026-09-12)

`80ebbb1ef7`: Rust/C admit zero checked sites plus epilogue site0; per-operation
required/unique/disjoint checks remain. No synthetic operation or new receipt.
Source ABI1 + physical ABI1/JSON9, C parser/build and guards passed. EXE then
exposed ordinary-parameter-count; the following publication fix supersedes it.
Carson's audit confirmed that absent return declaration+carrier passes validation:
source preservation must be asserted separately. Existing DraftSeal owns refresh.

### G0 declared-signature publication (verified 2026-09-12)

Shared draft/pending lowerer now transports descriptor source declarations and
Completion result through the existing setter/DraftSeal issuers; physical
receiver i64 remains unannotated. No backend repair or second contract issuer.
Jobs1 quick build 14m44s; G0 source1 + mutation1 (four cases) + capability4 +
physical ABI1/JSON9 pass (16 total). G0/R7/pointer/diff guards pass; warnings523.
Unchanged EXE now rejects at physical-json/instruction-unsupported before object;
dependency evidence only, not execution. Temporary runtime alias removed.

### Existing V4 scalar CFG consumer (verified; closeout)

Change: existing JSON/parser/V4 now consumes Compare/scalar PHI/Branch. Kind
equality and exact-state worklist replace recursion; shared incoming labels
cover ordinary/object/Map synthetic edges. Source/Recipe authority is unchanged.
Evidence: unchanged source-called G0 EXE3 without skip (2.24s; repeat 2.18s);
18 Rust tests pass, one jobs1 build14m54s, warnings523 unchanged. C scalar/Bool/
mutual PHI, signed extremes, tagged-Fault and Map-to-PHI execution pass; malformed
relations/types and unequal resource/Fault backedges reject. Existing Pair/Map
regressions pass, including 26 Map negatives; G0/R7/pointer/diff guards green.
Map probes require the no-entry lifecycle-core dependency archive; passing an
entry archive caused driver-main collision before admission, not a code red.
Source EXE uses the actual lifecycle archive directory, no temporary alias.
Landed/pushed `16f87f5eeb`; unchanged EXE acceptance rerun at `16329a0da2`
passes in 2.19s (one executed test, no skip). Aggregate R7, all-family Loop
and parity remain open.

### Harness compiler-path ownership (closed)

Named ownership landed/pushed `6e5b59c901`: C build/options ownership+OOM/direct+AOT/error-order smoke and route/pointer/census guards pass. Public ABI/replay adapter/child settings retained.
Decision: automatic replay branch/adapters retired at `34d9e409a6`; unconsumed private replay storage/copy/free and options lookup retired at `16329a0da2`, retaining wire validation and ambient public rejection.
Source authority + canonical issuer: pure-first public and Static reject ambient harness; explicit Generic/Boundary/Static options require none; direct allocation-capture core caller is test-only.
Non-authority: copied replay bytes and removed options lookup. Finite code census covers physical_options/common/route plus direct ownership test; sole predicate caller supplied NULL; no stored replay consumer remains.
Fail-fast boundary: preserve public JSON/contract/replay gates and existing unsupported terminal after core cleanup; no child launch after unsupported/emitter failure.
Evidence: `16329a0da2` is pushed. LLVM18 capture16/no-child negatives and probe retirement passed; C options smoke proves one fewer allocation and unchanged recipe/replay reject; route/census/pointer/diff pass, module/reference synchronized.
Non-claims: no public/provider/HAKO_CAPI_TM removal, new receipt, child-environment migration, concurrency isolation or aggregate R7 closure.

### Ordinary-new unclaimed-writer premise review

Premise review: parked M3-B couples two compatibility writers; the entry contract permits caller-local Stop without a successor. Review is finite, not a new broad R7 census.

Decision: caller-local Stop accepted after Meitner's complete-state review; deferred behind the reproduced scalar Call regression.
Source authority + canonical issuer: existing ordinary-new package claim and Birth recipe; no claim means no canonical target authority.
Non-authority: header lookup, class text, builtin injection and post-argument MIR.
Fail-fast boundary: the invocation non-direct-local branch must reject parser-covered classes for the exact registered App Main before argument/NewBox effects. Ledger app_main_identity plus Pending/Checked owner, not optional root_completion, identifies that scope.
Smallest next slice: existing ledger predicate consumed before this branch's `Ok(None)`; retain direct-local claims, foreign/transferred owners, absent ledger/site/owner and uncovered builtins. No change to shared writers.
Non-claims: no new receipt, source expansion, blanket constructor stop or whole-writer deletion. Static counterexample `box Page { birth() {} } static box Main { main() { return new Page() } }` reaches the old writer; publication/EXE not asserted.

### Priority scalar local Call recovery (accepted)

Not Fast path: the producer now records non-Map i64 local Calls, while the
affine row still selects Scalar and bypasses lifecycle binding publication;
the existing caller/return and local-call responsibilities must agree.
Evidence at unchanged `16329a0da2`: exact existing test
`mir::normal_callable_semantic_package::direct_call_lifecycle_tests::non_map_scalar_call_keeps_its_existing_owner`
fails at line410 (unexpected source Completion). This is pre-existing relative
to this docs-only diff, not a newly introduced failure or a passing baseline.
User counterexample: `local first = helper(10); return helper(20)`, with
source-declared i64 helper, must retain its supported source meaning.
Inspect the existing Completion callback, affine row co-seal and local physical
consumer; choose one shared admission contract, no new receipt/layer, and
retain G0 terminal EXE plus Map local/terminal behavior. Independent review is
limited to this source/consumer mapping; no broad Call census.
Next separate boundary: explicit `hako_llvmc_static_open_v2_with_options(NULL)`
must reject before option-validation bypass; public compatibility open remains.
Direct C recheck at `16329a0da2` confirms NULL returns success and allocates an
invocation for `{}`; the diagnostic closed that invocation, no artifact emitted.

Change: generalize only the existing non-Map affine-row co-seal branch. An
exact source-issued local relation selects the existing Lifecycle consumer,
removing its erroneous dispatch to ordinary Scalar Call without binding record.
Contract: same loan owner/site/destination, typed all-i64 target and argument
cardinality, existing terminal Call Completion with matching explicit return,
successful caller cleanup and no prior Homes. Absent local or terminal Call
relation keeps Scalar (including Plain/G0 terminal); Map checks remain unchanged.
Done: source local+terminal helper calls publish with two typed Invokes; retained
terminal/G0 and Map positives, unsupported prior-Home negative, existing corridor
guard, module README/reference and one jobs1 focused build. Parent cohort is
14pass/1fail; the stale no-Completion expectation must be replaced, not suppressed.
Stop: missing original Completion/target/site relation or new callee ABI meaning.
Meitner's issuer review and main's physical-consumer review agree: no new receipt,
callee-purity inference, layer, source callback narrowing or shared writer removal.

Kuhn's consumer review: Plain rejects pending physical local-call bindings,
not source local observations. `call_source_completion_for_owner` preserves
Scalar when absent and validates the original return when present. No Plain
payload expansion or weakened reject; initial build14m45s, cohort17/source1 pass.
Final build14m43s (warnings523) passed cohort18, source terminal1 (four modes;
exact argument10/20 and returned-second-result assertions), loan4 and unchanged
G0 EXE3 in2.21s without skip. Existing cleanup missing-binding mutation1 passes.
Parent `8624af8352`, byte-identical lifecycle_admission_tests.rs (`cmp`), isolated
target, jobs1/quick/no incremental: build17m14s, warnings523. Exact tests under
`mir::compiler::normal_default_pipeline::lifecycle_admission::tests::`:
- `scalar_local_then_plain_return_preserves_ordinary_call_through_publication`:
  parent and Call checkpoint both fail `root-cleanup-graph/residual-node` at line104.
- `scalar_local_then_terminal_call_reaches_published_lifecycle`: parent fails
  `local-call-binding-sequence`; current passes all four source/optimization cases.
Parent command: `CARGO_TARGET_DIR=/mnt/workdisk/hako-scalar-parent.64DJMK/target-parent
CARGO_BUILD_JOBS=1 CARGO_INCREMENTAL=0 RUST_MIN_STACK=16777216 cargo test --profile
quick --lib <fully-qualified Plain test above> -- --exact`; terminal test uses
that same built parent binary with its exact name. No implementation edits in parent.
Call-checkpoint classification/allowance: Plain was `known baseline debt`, owner
ordinary_new_admission/selected.rs::emit_root_home_exit_payload. It does not
block the independently proved local+terminal Call recovery checkpoint; keep
its failing regression visible and repair next, not ignored or called green.
No whole-publication completion claim. No unclassified current-change red remains.

### Empty-Home Plain cleanup repair (verified; closeout)

Parfit's finite read-only audit: root-exit capture -> finishing -> FinishedBindings
-> root/call validation; includes ingress/contraction/pruning, excludes source expansion.
Decision: correct existing emitter for empty-Home Plain; no acceptance expansion.
Source authority + canonical issuer: Completion terminal_homes -> begin_root_home_exit -> emit_root_home_exit_payload.
Non-authority: deleted-block presence, inferred destinations and final-MIR reachability as source meaning.
Fail-fast boundary: preserve missing-binding, sequence, ingress and Call-entry checks.
Smallest next slice: omit unused Fault allocation/emission/recording when operations.is_empty() and call.is_none(); retain frame ownership and clean Return/Jump.
Non-claims: no new receipt, projection filter, callee effect inference or aggregate R7 closure.
Reason: pre-repair emitter recorded a disconnected ReturnFault in this exact case;
DCE removes it and projection correctly rejects the residual recorded node.
Acceptance: unchanged Plain publication test off/on; existing Home cleanup
missing-binding mutation and Call normal/fault path tests; source terminal/G0
retention, existing corridor guard and module/reference update. Call recovery
checkpoint `ad5730b48d` is pushed; this separate repair resolves its Plain debt.
Evidence: jobs1 quick build14m44s, warnings523 unchanged. Admission3 (Plain
off/on plus no-orphan assertion, terminal four modes and foreign/Birth rejects),
cohort18 (Call normal/fault included), Home missing-binding mutation1 and loan4
pass. Source-called G0 EXE3 passes in2.27s, one executed/no skip; corridor/census/G0/pointer/diff guards green.
T0 evidence for deletion at `537e5e20fe` is corrected in CURRENT_STATE, R0/I1
and the contract README; selection provenance stays missing, not backdated.
llc policy, typed errors and caller-zero ambient helper remain separate follow-ups.
Follow-ups: decide Boundary llc-flags default in its existing request owner;
current C ambient default is `-O3 -mcpu=native` while explicit NULL flags become
empty; Rust `OwnedPhysicalCompileContract::from_request` captures only the env
override. This is a confirmed policy difference, not a measured performance loss.
typed rejects belong to direct_accum/nested_predicate/pending; inspect the
caller-zero `llvm_provider_flags.rs` helper before deletion. A parent-valued
census observed_commit is provenance, not by itself source drift; regenerate
only against actual manifest anchors/hashes. Normal bounded selection and
implementation may share a commit under the implementation-coupled commit law;
that permission never overrides an unresolved design_stop.

### MIR-CALL-LEGACY-TM-INVOCATION-CAPTURE-I0 (selected 2026-09-12)

Hegel's read-only worker review closed the design gap without reopening the
whole census. The existing `HakoLlvmcInvocation` is the private lifetime owner
for every selected compile entry, while the legacy CAPI probe remains a shared
compatibility terminal. Shared callers therefore retain the owner; this row
deletes only the emitter's ambient reads after migration.

```text
Decision: capture the legacy TargetMachine admission in HakoLlvmcInvocation and consume it from the existing emitter.
Source authority + canonical issuer: an eligible compile execution captures HAKO_CAPI_TM and HAKO-first/NYASH-fallback opt level once after root/profile/pinned-census gates.
Non-authority: MIR/Recipe meaning, public ABI, provider reachability, typed published rows, and profile labels do not issue a new semantic product.
Fail-fast boundary: root/profile/census rejection -> invocation capture -> existing probe -> existing opt/llc fallback -> artifact terminal.
Smallest next slice: add two private scalars, bypass capture for contract-bound/published-row routes, remove the emitter's three getenv reads, and add a source-backed mutation-after-capture test.
Non-claims: no public ABI deletion, TargetMachine/probe fallback deletion, published-row migration, concurrency guarantee, backend parity, or aggregate R7/MirBuilder closure.
```

Capture semantics are fixed to the current emitter: `HAKO_CAPI_TM` is enabled
only when present with first byte `1`; the opt level selects HAKO when that key
is present (including empty), otherwise NYASH when present, otherwise `0`;
first byte `1/2/3` maps to that level and every other value maps to `0`.
Typed/contract-bound and published-row lowering does not capture or consult this
legacy state. Probe failure still enters the existing opt/llc path exactly once;
successful probe still publishes directly. The public exports and the
`HAKO_CAPI_TM` compatibility keep remain retained.

Implementation task order:

1. Add invocation-owned `legacy_capi_tm_enabled` and normalized opt-level
   scalars; reset them on every execution and capture only after the existing
   root, legacy-op, and pinned-text gates.
2. Replace the legacy emitter's three environment reads with those scalars,
   retaining its non-Windows guard, dynamic symbol checks, cleanup, and
   existing failure fallback.
3. Extend the existing C route smoke/driver and reusable route guard for
   unset, empty, HAKO/NYASH precedence, suffix normalization, environment
   mutation after capture, typed-row bypass, and no new public symbol.
4. Update `lang/c-abi/README.md` with the private ownership boundary; no public
   ABI reference change is required because the exported contract is retained.

Done requires the focused driver and C smoke, route/census/pointer guards,
source-size and diff checks, with any real-app red classified as baseline or
environment evidence. The host RAM replacement is outside this code row:
until installed and validated, development builds use one build and at most
two compiler jobs; swap changes remain a separate measured host task.

### MIR-CALL-LEGACY-TM-INVOCATION-CAPTURE-I0 closeout (2026-09-12)

Implementation is complete in the existing private invocation owner. The legacy
emitter no longer reads `HAKO_CAPI_TM`, `HAKO_LLVM_OPT_LEVEL`, or
`NYASH_LLVM_OPT_LEVEL`; the captured state is reset per invocation, bypassed
for contract-bound/published-row lowering, and consumed by the existing
non-Windows probe. Public exports, the explicit probe, cleanup, and opt/llc
fallback remain unchanged.

Observed acceptance:

- `llvm_compile_options_contract_smoke.sh` passed, including the new C capture
  driver for unset, empty, precedence, suffix, mutation-after-capture, and
  non-legacy reset cases.
- `llvm_codegen_route_identity_guard.sh`,
  `llvm_llvmlite_production_census_guard.py`,
  `current_state_pointer_guard.sh`, and `git diff --check` passed.
- The focused C build completed. Changed source units remain below the 800-line
  hard stop (`invocation.inc` 89 lines; legacy emitter 124 lines).
- A direct real-LLVM probe reached the existing LLVM14 opaque-pointer/opt
  failure and produced no object. This is `known baseline/environment debt`,
  not a current-change failure; the private capture driver and route smoke are
  the accepted evidence for this transport slice.

This closes only the legacy selector capture row. The next design boundary is
the separate global published-call-row lifecycle: define invocation ownership,
re-entry behavior, and the exact caller-specific old-edge delete-set before
changing its consumer. No public ABI deletion, concurrency claim, or aggregate
R7/MirBuilder completion is implied.

### MIR-CALL-PUBLISHED-ROWS-INVOCATION-OWNERSHIP-D0 (queued design stop, 2026-09-12)

The current source fact is finite: `hako_llvmc_published_call_rows` stores the
borrowed row pointer, count, mode, and `used[1024]`; all production consumers
reach it through `peek/take/finish`, while Static V2 calls `begin` after its
invocation `bind` and cleans up at `end`. A second read-only consultation was
explicitly cancelled after observation timeout; that is not rejection
evidence. Do not implement from the global census alone.

Required design deliverable: a source-backed issuer/terminal matrix for the
Static V2 production caller and every generic lowering consumer, a complete
invocation pointer path for prepass/emission/residual checks, a decision for
same-invocation re-entry and distinct-invocation overlap, and an exact delete
set naming the old global reads/writes. Preserve borrowed Rust row lifetime,
typed kind/coordinate validation, duplicate-take rejection, zero-row V2
behavior, and cleanup on every pre-artifact failure. Acceptance is one closed
Decision plus focused re-entry/cleanup/row-consumption negatives; until then
After AOT child-env I1 Windows proof, this remains the next design stop; until
then the active pointer is the bounded I1 verification above.
