# mimalloc allocator-first task granularity SSOT

Status: SSOT
Decision: accepted
Date: 2026-05-15
Last slimmed: 2026-05-19
Scope: allocator-first implementation order and language-feature sidecar policy.

## Current Navigation Contract

Current row and latest-card pointers are owned by:

```text
docs/development/current/main/CURRENT_STATE.toml
```

This SSOT owns durable granularity rules, stop lines, and row-selection
boundaries. It is not the live current-status ledger and must not regrow into a
landed-history ledger.

For the active phase, read the concrete current row from
`CURRENT_STATE.toml`. Current stop lines are:

```text
closed until explicitly reopened:
  real raw pointer residence
  real segment-map mutation
  real allocator free-list mutation
  arena backing
  atomic bitmap execution
  OSVM/page-source execution
  worker scheduling / source-level concurrency
  provider activation / host allocator replacement / hooks / #[global_allocator]
  cross-function Result direct ABI
  runtime sum materialization
  backend matchers
```

## Slimming Contract

This file is intentionally short. It must answer:

```text
which lane owns granularity rules?
what is the current pointer owner?
what remains closed?
where does row detail live?
which legacy guard anchors are stable?
```

It must not answer:

```text
what landed in every historical row?
what exact card prose belongs to each row?
what is the latest current row history?
```

Ownership split:

| Content | Owner |
| --- | --- |
| Current row / latest card | `CURRENT_STATE.toml` |
| One-screen restart | `CURRENT_TASK.md`, `10-Now.md`, `05-Restart-Quick-Resume.md` |
| Durable row order | `docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md` |
| Row detail | Row card under `docs/development/current/main/phases/phase-293x/` |
| Historical granularity prose | Archive files listed below |
| Durable granularity rule / stop lines | This file |

Archive snapshots:

```text
docs/development/current/main/design/archive/mimalloc-allocator-first-task-granularity-full-ledger-2026-05-18.md
docs/development/current/main/design/archive/mimalloc-allocator-first-task-granularity-active-ledger-2026-05-19.md
```

New row detail rule:

```text
write detail in the active row card
summarize durable order in the taskboard
update this file only when the granularity policy or stop lines change
keep guard-compatible anchors as headings only
```

## Decision

Continue the mimalloc lane through allocator behavior first.

Hakorune language features should not be implemented speculatively. If the
allocator row hits a compiler/language blocker, split the blocker into a
minimal sidecar row with its own fixture and guard, then return to the allocator
row.

```text
primary:
  allocator behavior row

sidecar:
  smallest compiler/language acceptance row that unblocks the allocator row

defer:
  broad language semantics without allocator evidence
```

## Stop Lines

Each allocator row must own one durable behavior only.

Do not mix:

- allocation and release
- release and realloc
- OSVM and in-memory page model
- provider/hook/global allocator activation and allocator facade behavior
- compiler BoxCount fixes and allocator behavior in the same row
- BoxShape cleanup and allocator behavior in the same row

Unsupported backends must fail fast. VM remains diagnostic for object-heavy
allocator routes. LLVM/EXE is the primary acceptance backend for backend-facing
allocator routes.

Still closed until a row explicitly reopens them:

- real raw pointer residence
- real segment-map mutation
- real allocator free-list mutation
- real arena backing
- atomic bitmap execution
- OSVM/page-source execution beyond the active row contract
- worker scheduling / source-level concurrency
- provider activation / host allocator replacement / hooks / `#[global_allocator]`
- cross-function Result direct ABI
- runtime sum materialization
- backend matchers by app, owner, helper, or row name

## Row Granularity Rules

Use this cadence unless an active card says otherwise:

```text
planning row:
  L0 docs / selection only

scalar modeled behavior row:
  L0 + L1 VM + L2 MIR / route preflight

first-pattern row:
  L0 + L1 + L2 + L3 representative EXE

closeout row:
  L0 + L1 + L2 + L3 representative EXE
```

A row may add a proof app, manifest row, guard, SSOT, and focused owner helper
only when those artifacts prove the row boundary. Avoid broad fixture growth.

If a report carrier is all scalar data and the row is not opening record pass /
return / store escape, prefer an owner-local `record ...Fields` as an internal
carrier and keep existing box-return ABI stable until the record capability is
explicitly reopened.

## Current Row Pointer

Do not mirror the concrete row token here. The current row is owned by:

```text
docs/development/current/main/CURRENT_STATE.toml
```

## Durable Row Order

Do not paste the full row table here. The durable row order lives in:

```text
docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md
```

Historical full prose lives in the archive snapshots listed above.

## Compatibility Anchors

Some older guard scripts still assert that this file contains row names or
`granularity` headings. Keep these anchors as stable strings while moving prose
to row cards and archives.

### MIMAP-014A single-page small allocation fast-path

Guard-compatible early row anchor. Detail lives in the row card and archive.

### MIMAP-014B active-page fallback and allocation miss

Guard-compatible early row anchor. Detail lives in the row card and archive.

### MIMAP-014C allocation fast-path stats observers

Guard-compatible early row anchor. Detail lives in the row card and archive.

### MIMAP-142A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-143A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-144A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### HAKO-ALLOC-ID-BRAND-001 granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### PURE-FIRST-BRAND-CONSTRUCT-001 granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### HAKO-ALLOC-ID-BRAND-002 granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### HAKO-ALLOC-ID-BRAND-003 granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-145A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### HAKO-ALLOC-REPORT-RECORD-001 granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### HAKO-ALLOC-REPORT-RECORD-002 granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-146A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### HAKO-ALLOC-RESULT-API-001 granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### PURE-FIRST-GUARDLET-ENUMMATCH-001 granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### HAKO-ALLOC-RESULT-API-002 granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-147A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### HAKO-ALLOC-RESULT-API-003 granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-148A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-149A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-150A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-151A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-152A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-153A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-154A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-155A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-156A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-157A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-158A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-159A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-160A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-161A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-162A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-163A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-164A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-165A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-166A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-167A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-168A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-169A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-170A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-171A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-172A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-173A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-174A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-175A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-176A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-177A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-178A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-179A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-180A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-181A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-182A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-183A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-184A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-185A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-186A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-187A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-188A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-189A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-190A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-191A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-192A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-193A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-194A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-195A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-196A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-197A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-198A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-199A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-200A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-201A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-202A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-203A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-204A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-205A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-206A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-207A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-208A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-209A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-210A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-211A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-212A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-213A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-214A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-215A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-216A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-217A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-218A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-219A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-220A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-221A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-222A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-223A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-224A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-225A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-226A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-227A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-228A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-229A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-230A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-231A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-232A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-233A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-234A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-235A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-236A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-237A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-238A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-239A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-240A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-241A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-242A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-243A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-244A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-245A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-246A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-247A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-248A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-249A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-250A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-251A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-252A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-253A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-254A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-255A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-256A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-257A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-258A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-259A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-260A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-261A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-262A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-263A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-264A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-265A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-266A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### HAKO-ALLOC-REPORT-RECORD-003 granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### HAKO-ALLOC-REPORT-RECORD-004 granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-267A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-020A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-021A / MIMAP-021B granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-022A / MIMAP-022B granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-023A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-024A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-025A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-026A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-027A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-042A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-042B granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-043A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-043B granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-044A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-044B granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-045A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-045B granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-046A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-046B granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-047A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-047B granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-048A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-048B granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-049A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-049B granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-050A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-051A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-051B granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### USES-002A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-052A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-052B granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-053A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-054A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-055A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-056A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-057A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-058A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-059A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-060A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-061A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-062A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-063A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-064A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-065A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-066A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-067A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-068A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-069A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-070A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-071A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-072A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-073A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-074A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-075A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-076A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-077A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-078A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-079A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-080A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-081A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-082A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-083A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-084A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-085A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-086A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-087A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-088A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-089A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-104A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-105A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-ROW-CADENCE-001 granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-106A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-107A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-116A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-117A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-118A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-119A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-120A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-121A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-122A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### PURE-FIRST-GLOBAL-CALL-001 granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-123A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### ROUTE-FIXPOINT-001 granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### ROUTE-DIAG-VOCAB-001 granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### GUARD-MANIFEST-011 granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### ROUTE-DIAG-VOCAB-002 granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-124A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### RUNTIME-UNWRAP-001 granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### WASM-LOG-001 granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-125A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-126A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-127A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-128A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-129A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-130A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-131A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-132A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-133A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-134A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-135A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-136A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-137A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-138A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-139A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-140A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### GUARD-MANIFEST-012 granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### GUARD-MANIFEST-013 granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-141A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-112A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-113A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-114A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-115A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-108A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-109A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-110A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-111A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### HAKO-ALLOC-SRC-CLEAN-001 granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-103A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-090A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-091A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-092A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-093A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-094A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-095A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-096A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-097A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-098A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-099A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-100A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-101A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.

### MIMAP-102A granularity

Guard-compatible granularity anchor. Detail lives in the row card, taskboard, or archive.
