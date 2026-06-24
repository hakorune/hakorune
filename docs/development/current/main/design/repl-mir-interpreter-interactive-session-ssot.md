---
Status: SSOT
Date: 2026-06-24
Scope: REPL / interactive execution / MirInterpreter session model
---

# REPL MirInterpreter Interactive Session SSOT

## Decision

Do not build a new interpreter for the Python-like REPL lane.

The existing Rust `MirInterpreter` may remain a bootstrap/reference execution
engine for already-supported smoke checks, but it is not the owner for new
interactive interpreter feature work.

The target architecture remains complete input cells:

```text
complete input cell
  -> AST
  -> MIR fragment
  -> persistent MirInterpreter session
  -> execute cell entry
```

Physical-line execution is not the long-term contract. A physical line may be a
complete cell, but multiline `if`, `loop`, function, box, and using forms must
be accepted as one cell.

## Start Condition / Stop Line

REPL/interpreter product work is parked until the VM / `MirInterpreter`
execution owner has moved from Rust to `.hako`, or a future current-state card
explicitly reopens a smaller reference-only slice.

```text
rust_mir_interpreter_new_repl_feature_owner=0
hako_mir_interpreter_required_before_python_like_repl=1
rust_mir_interpreter_existing_reference_smokes=allowed
rust_mir_interpreter_feature_growth=denied
```

This prevents the REPL lane from extending the Rust VM just before it is meant
to be replaced. The current Rust implementation is evidence and regression
scaffolding, not the final interactive execution authority.

## Current Evidence

The repository already has a basic MIR-backed REPL:

```text
src/runner/repl/repl_runner.rs
  wraps input as Main.main, parses, rewrites, compiles to MIR, and executes.

src/runner/repl/ast_rewriter.rs
  rewrites undeclared variable get/set through __repl.get / __repl.set.

src/runner/repl/repl_session.rs
  stores REPL variables as VMValue and stores `_`.

src/backend/mir_interpreter/mod.rs
  owns MirInterpreter and can attach a ReplSessionBox.

src/backend/mir_interpreter/handlers/externals.rs
  handles __repl.get / __repl.set.
```

Current gap:

```text
persistent variable values:
  partially present through ReplSessionBox

persistent definitions:
  missing

persistent interpreter/object state:
  missing, because ReplRunner creates a fresh MirInterpreter per evaluation

multiline complete-cell input:
  missing
```

## Layering

Keep responsibilities thin:

```text
ReplFrontend
  prompt, multiline input, dot commands, history, completion

InteractiveSession
  session lifecycle, cell_id, bindings, definitions, imports

InteractiveCompiler
  complete cell -> AST -> MIR fragment

InteractiveExecutor
  owns persistent MirInterpreter
  installs fragments and runs cell entry

DisplayFormatter
  repr, diagnostics, traceback formatting

MirInterpreter
  execution state and object/static-box runtime state
```

`ReplRunner` must not become the owner of parser, compiler, and interpreter
state. It should stay an I/O runner that delegates to the session objects.

## Task Order

### 0. REPL-RUST-REFERENCE-BOUNDARY-001

Closed by this SSOT. The bootstrap/final split is:

```text
current execution owner:
  Rust MirInterpreter for existing reference smokes only

role:
  evidence and regression tests for the already-supported subset

final intended owner:
  .hako MirInterpreter produced or maintained through the Rust-to-Hako
  converter lane
```

Acceptance:

```text
Rust MirInterpreter is not described as final semantic authority
REPL work may use Rust MirInterpreter only as reference/bootstrap
zero-Rust / .hako-AOT REPL claim = 0
```

Migration trigger:

```text
after a selected VM / MirInterpreter slice has a `.hako` implementation and
Rust-vs-Hako differential fixtures are green, reopen the REPL task order from
REPL-VALUE-SESSION-CONTRACT-001.
```

### 1. REPL-VALUE-SESSION-CONTRACT-001

Status: parked until `.hako` MirInterpreter ownership exists.

Make the current MVP truthful before adding new execution machinery.

Acceptance:

```text
x = 1
x
  -> 1

local z = 3
z
  -> 3

1 + 1
print("hello")
_
  -> 2

.reset
x
  -> Undefined variable
```

Boundaries:

```text
no persistent MirInterpreter yet
no multiline parser completeness yet
no definition environment yet
```

Known audit points:

```text
local declarations may be collected as declared_names and skip __repl.set
`_` may be updated after statement/Void execution instead of displayed value
```

### 2. REPL-PERSISTENT-MIR-EXECUTOR-001

Status: parked until `.hako` MirInterpreter ownership exists.

One interactive session owns one `MirInterpreter`.

Persistent state:

```text
object fields
static box registry
installed definitions
session bindings
```

Per-cell transient state:

```text
registers
aliases / caches
call stack
current function / block
recent trace
```

Acceptance:

```text
object/static state survives across cells
register/call-stack state does not leak across cells
__repl.get/set still operate through the same SessionBindings owner
```

### 3. REPL-COMPLETE-CELL-INPUT-001

Status: parked until `.hako` MirInterpreter ownership exists.

Introduce parser completeness:

```text
Complete(AST)
Incomplete(expected)
Error(diagnostic)
```

Acceptance:

```text
>>> if x > 0 {
...   print(x)
... }
```

is one cell, not three independent one-line programs.

### 4. REPL-MODULE-FRAGMENT-INSTALL-001

Status: parked until `.hako` MirInterpreter ownership exists.

Split module installation from cell execution:

```text
install_module_fragment(fragment)
execute_cell_entry(cell_id)
```

Acceptance:

```text
function twice(x) {
  return x * 2
}

twice(4)
  -> 8
```

Definitions must merge into the session registry and remain callable from later
cells.

### 5. REPL-DISPLAY-DIAGNOSTICS-001

Make the UX Python-like after semantics are stable:

```text
repr vs print separation
string display with quotes
Array / Map / UserBox recursive display
source cell number in errors
call traceback
Ctrl-C interrupts current cell only
Ctrl-D exits
```

### 6. REPL-SHELL-UX-001

Add shell comforts last:

```text
history
left/right editing
completion
search
color
```

### 7. REPL-HAKO-INTERPRETER-SHADOW-001

Port only the stable interpreter core needed by the REPL contract to `.hako`.

Input:

```text
selected Rust MirInterpreter slice
stable REPL fixtures
normalized interpreter state facts
```

Output:

```text
.hako MirInterpreter shadow
Rust-vs-Hako differential gate
same REPL value/session fixtures green
```

Non-claims:

```text
no whole MirInterpreter translation
no AOT mainline switch
no Python/converter replacement yet
```

### 8. REPL-HAKO-AOT-INTERPRETER-001

After the `.hako` shadow is green, compile it through AOT and use it as the
candidate interactive execution engine.

Acceptance:

```text
.hako interpreter AOT executes the stable REPL fixtures
Rust MirInterpreter remains oracle/reference only
fallback from .hako AOT to Rust at runtime = 0
```

## Non-Claims

```text
no new interpreter
no AOT dependency
no boxed-sum / converter ABI dependency
no legacy --backend vm resurrection
no giant incremental MIR module rewrite requirement
no final zero-Rust interpreter claim while executing with Rust MirInterpreter
```

## Relationship To Other Lanes

Keep this lane separate from the current MirBuilder converter / boxed-sum AOT
lane.

```text
boxed-sum / Option<i64> ABI:
  LLVM/AOT representation lane

REPL:
  runner / interactive compiler / MirInterpreter lane
```

REPL work may use the Rust `MirInterpreter` even if legacy `--backend vm` is
retired as a mainline product route. That use is a bootstrap/reference contract,
not the final selfhost interpreter claim. The final route is:

```text
Rust MirInterpreter reference
  -> stabilized REPL/interpreter fixtures
  -> selected interpreter slice converted to .hako
  -> .hako shadow differential green
  -> .hako AOT candidate
```
