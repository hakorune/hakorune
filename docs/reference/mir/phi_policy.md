## MIR PHI Policy (Phase‑15)

Status
- Default: PHI‑on. MIR builders always emit SSA `Phi` nodes at merge heads, and verifiers run with full dominance checks.
- Legacy fallback: Set `NYASH_MIR_NO_PHI=1` to enforce the former edge‑copy mode (PHI‑off) for targeted debug sessions.
- LLVM: The llvmlite harness still validates and, when necessary, rewires PHIs, but it no longer compensates for missing SSA form in the input.

Rationale
- Break/continue correctness: PHI‑off exposed MIR generation bugs around loop exits; keeping PHIs generated in the core builder avoids silent value reuse mistakes.
- ChatGPT Pro design review: external audit recommended shipping Phase‑15 with PHI enabled by default to minimize backend divergence and simplify documentation.
- Maintained parity: PyVM and LLVM continue to share the same MIR stream; PHI nodes remain the single source of truth for join semantics.

Operational Rules (PHI‑on)
- Merge blocks place PHIs first, with one incoming per predecessor, covering loop latches, break/continue exits, and structured control flow.
- `verify_allow_no_phi()` mirrors `NYASH_MIR_NO_PHI`; with PHI‑on it stays strict and fails if SSA form is missing.
- Use `NYASH_LLVM_TRACE_PHI=1` to inspect wiring; traces now confirm the builder’s SSA layout instead of synthesizing it from edge copies.

Resolved nested-If recipe boundary (D2)
- The selected production profile is exactly one outer plus one inner
  explicit-`else` If over one shared trivial binding and pure fallthrough.
- Its portable Recipe/JoinSig is admitted before physical effects. The
  existing canonical lowerer is the sole CFG/SSA/PHI physicalizer and emits
  the two merge PHIs; no nested-specific PHI writer or retry route exists.
- Deeper nesting, calls, effects, returns, or multiple bindings are outside
  this profile and must not be inferred from these two-PHI rules.

Fallback Mode (PHI‑off)
- Toggle: `NYASH_MIR_NO_PHI=1` (optionally pair with `NYASH_VERIFY_ALLOW_NO_PHI=1`).
- Behavior: MIR builders revert to edge copies per predecessor and skip PHI emission. This path is retained only for diagnosing older JSON dumps.
- Guardrails: tooling should mark PHI‑off runs as legacy; new smokes and CI stay on PHI‑on unless explicitly overridden.

Backends
- LLVM harness consumes the PHI‑rich MIR stream and validates incoming edges; no extra synthesis is performed unless legacy mode is forced.
- Cranelift/JIT paths operate on the same MIR14 form; Phase‑15 keeps them secondary but expects PHIs to be present.

Acceptance
- Default smokes/CI run with PHI‑on.
- Legacy PHI‑off runs must document the reason in `CURRENT_TASK.md` (e.g., reproducing historical MIR13 bugs) and avoid committing the override into shared scripts.

Loop M6-B caller-zero boundary
- `LoopPhiMaterializerV1` is a test/evidence observer only. It consumes
  verified logical JoinSig obligations and an explicit physical path map; it
  does not select routes, infer CFG, or publish MIR.
- Its PHI writes are transactional through `PhiTxn`; canonical CFG and
  function-owned Binding SSA remain the production lifecycle owners.
- The M6-B closeout and structural P1b witness change no source grammar, MIR
  shape, Retry/fallback, or route behavior.

Resolved DirectAccum final-carrier profile (M10a D2-S5/P4-S1)
- The canonical resolved lowerer is the sole production caller for this
  singleton profile and borrows the existing CFG, Binding SSA, and `PhiTxn`.
- It seals `After` first, reads carrier keys 0/1 through the canonical adapter,
  and stores a typed final-binding receipt before identity/PHI completion.
- Because `After` has one predecessor, the final reads are aliases and do not
  authorize synthetic After PHIs. The test-only P4 snapshot observes actual
  candidate MIR; it does not alter PHI policy or infer source semantics.
