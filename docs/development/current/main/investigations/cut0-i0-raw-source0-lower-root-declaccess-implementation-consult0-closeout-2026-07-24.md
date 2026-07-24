# DECLACCESS0 implementation consultation closeout

Status: **Decision locked; next executable rows are COVERAGE0 → MANIFEST0 → COINSTALL0**  
Date: 2026-07-24  
Decision: **DECLACCESS-IMPLEMENTATION-prime-r1**

The worker audit of PHYSICAL0, ScalarControl0, and the Builder/shell owners is
closed. The existing `cargo check --lib` and `raw_root` test family remain
green, and no production consumer was added.

## Q1 — manifest-before-physical ordering: adopt option 1

Introduce one internal consuming state between eligibility and PHYSICAL0:

```text
EligibleSourceBoundRawRootPackageV1
  -> ManifestBoundRawRootPackageV1
       exact manifest + eligibility proof + source/plan/config
  -> open_physical(current)
       existing session/shell/collector open
       manifest moved into RawRootPhysicalCoreV1
```

Manifest construction happens while the eligible owner still owns the
source, before `RawRootPhysicalStateV1::open` or
`ModuleBuilderInvocationSessionV1::open_for_token`. Manifest failure retains
the whole eligible owner. `ManifestBoundRawRootPackageV1` is not a public
caller choice and does not add a second physical-open terminal.

Before the manifest producer is added, ELIGIBILITY0 receives a narrow
coverage witness: Script declaration/top-level-function/non-Main-box work is
rejected explicitly, and plain static-Main App coverage is sealed. DECLACCESS
does not become a second eligibility authority.

## Q2 — exact source payload: adopt option 1

Source analysis is performed once for the accepted route in a new sibling
`raw_root_source_facts.rs`, not by growing `raw_root_plan0.rs` past the 800-line
boundary. The source-facts product owns:

```text
physical root identity and route
lexical helper schedule
located ScalarControl0 payload
  (LiteralValue, variable name, operator, path, Span)
exact App Main locator and callable rows
complete existing callable declaration catalog, sealed once and moved
runtime-input and ingress-config snapshots
typed ProvedAbsent static-data / closure / process-slot seals
```

The existing callable catalog is retained as a complete owned product because
its resolver contract includes parameter declarations, return type, and body
AST. It is sealed once from the source and never re-sealed from
`current_module` or a later AST lookup. The located ScalarControl0 payload is
the BODY0 root-body authority; the callable catalog is the callable-resolution
authority. Neither is reconstructed after the manifest is sealed.

The summary fields in `RawRootEnvironmentPlanV1` become a derived/transition
view and are not a second environment authority. A longer-term cleanup may
make PLAN0 consume the source-facts product directly, but this row must not
add a second classifier authority.

## Q3 — Builder-owned co-install: adopt option 1

Add one Builder sibling aggregate terminal:

```text
RawRootEnvironmentInstallerV1::prepare(session, physical, projections)
  -> PreparedRawEnvironmentInstallV1
  -> private infallible commit(self)
```

Preparation borrows both owners and validates brand/family, session readiness
without closing the session, Raw-owned destination lanes (allowing the
prelude enum baseline), shell metadata vacancy, tracker/reservation vacancy,
and callable-catalog vacancy. It does not call `prepare_module_session()`.

The compiler never calls `builder_mut()`, `into_parts()`, or receives a
shell/collector/ledger tuple. Builder-private methods install the candidate
context and shell projections together. Failure returns a named discard-only
owner retaining session and physical state; no retry, parts extraction, or
partial commit exists.

## Executable order

```text
DECLACCESS-COVERAGE0
  -> ManifestBoundRawRootPackageV1
  -> DECLACCESS-MANIFEST0
  -> DECLACCESS-COINSTALL0
  -> DECLACCESS-S0 (declare_environment(self) + DeclaredRawRootInvocationV1)
```

Each row has no production consumer. BODY0 remains the only continuation from
`DeclaredRawRootInvocationV1`; root batch, drain, finalization, postprocess,
external commit, public ingress, JSON behavior, retry, fallback, and CUT0
remain zero.

## Guards and acceptance

```text
manifest producer before physical open = 1
source classifier authority             = 1
callable catalog sealing                = 1
compiler raw physical tuple access     = 0
compiler builder_mut / into_parts      = 0
current_module / AST rescan after seal = 0
partial commit / retry / fallback      = 0
all touched files < 800 lines
```

Natural tests cover Script/App coverage rejection, manifest source-site and
literal retention, callable catalog single sealing, physical-open ordering,
lane-specific destination vacancy, co-install success, and mutation-free
rejection.
