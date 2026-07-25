---
Status: active design consultation
Date: 2026-07-26
Decision needed: NORMAL-ENTRY-CUTOVER-D2
Scope: choose exactly one production consumer for the proven normal-file forge
Prerequisite:
  - RESULT-CARRIER-NORMAL-CAPABILITY0-S0 closed
  - production caller remains zero
---

# NORMAL-ENTRY-CUTOVER-D2

## Decision boundary

The bounded normal-file forge is complete.  It owns one UTF-8 read, one
canonical parse, one opaque Raw handoff, and uses the existing exact
VM-reference execution/result terminal.  Its first profile is intentionally
narrow:

```text
NormalFileNoImportVmReferenceV1
  admitted result carriers = Unit / Integer / Bool / Float / String
  annotations / explicit Main return / ordinary callable = typed reject
  imports / macros / plugins / REPL / JSON / script args = typed reject
  normal/default routing = unchanged
  production caller = 0
```

The preceding caller census is still binding:

```text
plain source-hint production sites = 6
admissible existing normal caller  = 0
NoBoundedCallerFamily              = sealed
```

Therefore D2 must not reinterpret a benchmark, artifact emitter, Stage-1
compatibility route, VM fallback, or any existing source-hint caller as the
new normal family.

## Closed evidence

```text
FACTS0       = 8ce104f119
REJECTION0   = 6da5853965
FORGE-S2     = f76e5bc739
FORGE-REUSE0 = 766a00edef
FORGE-G0     = 125ae76d5c
```

The green fixture/guard evidence proves:

```text
one profile / one read / one canonical parse
no source rewrite / no fallback / no second compiler
Script scalar-and-Unit result matrix
Null/Void observed but not provenance-credited
function-exit and owner-bearing source exclusions are typed rejects
profile/source/parse/compile/process-fault/VM-fault -> later success
normal/default caller remains zero
```

## Choices

### A — recommended: forge one explicit opt-in production request

Create one new, default-off runner request whose only semantic work is:

```text
normal file path
  -> NormalFileVmFrontDoorV1
  -> existing Raw published compile kernel
  -> existing exact VM-reference terminal
  -> RawVmReferenceRunReportV1
```

The CLI spelling, if any, maps once to a typed request and is not semantic
authority.  The request is not `compile_with_source`, is not the default
compiler route, and does not replace an existing caller.  It has no legacy
caller sunset because no old caller is displaced.

### B — map a later, newly audited existing caller

Do not choose this unless a fresh caller census overturns the sealed D0
finding.  This requires a new design row; D2 must not silently relabel one of
the six rejected sites.

### C — retire the forge proof and keep caller zero

Use only if the explicit opt-in request is no longer desired.  This consumes
the reserved `NORMAL-FILE-VM0-FORGE-PROOF-RETIRE0-S0` path and does not widen
any legacy route.

## Recommendation

Choose A.  It is the accepted forge-front-door direction and gives the new
compiler a real file execution entry without widening the default route or
pretending a legacy caller was migrated.

## Required D2 answers

```text
Q1 exact request owner:
  Which one new explicit runner request owns the file path and sealed profile?

Q2 request visibility:
  Is the first entry CLI-visible default-off, or an embedding-only production API?

Q3 user-facing failure classes:
  Keep profile/source/parse/compile invocation failures separate from
  program Fault status 70?  (Recommended: yes, unchanged.)

Q4 promotion rule:
  What measured parity and caller-census evidence is required before this
  explicit request may replace any normal/default caller?
  (Recommended: a separate later cutover decision.)
```

## If A is accepted

```text
NORMAL-FILE-VM0-REQUEST0-S0
  -> one typed runner request, no default selection

NORMAL-FILE-VM0-REPORT0-S0
  -> retain RawVmReferenceRunReportV1 to the thin terminal

NORMAL-FILE-VM0-PARITY0-P0
  -> real-binary source/status/diagnostic evidence

NORMAL-FILE-VM0-CALLER0-I0
  -> exactly one route-scoped production caller

NORMAL-FILE-VM0-G0
  -> caller=1, fallback=0, all existing callers unchanged
```

No legacy caller retirement is issued in this branch.  Any later old-caller
replacement must name that exact caller before it receives a sunset.

## Non-claims

```text
default normal-entry cutover
compile_with_source change
existing six-caller reinterpretation
imports / macros / plugins / JSON / REPL / script arguments
dynamic/object result carrier
annotation or ordinary callable admission
LLVM/native/ny_main activation
legacy Raw-chain retirement
```
