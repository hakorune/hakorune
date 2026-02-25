# selfhost/opt — Optimization/Preparation Boxes

Purpose
- Provide small, opt-in preparation/optimization stages implemented in Hakorune.
- First step: AotPrepBox — MIR(JSON) normalize + safe local const-fold for single-block const/binop/ret.

Responsibilities
- JSON normalization via shared MirIoBox (canonicalization hook).
- Behavior-preserving, local transforms only; Fail‑Fast for unsupported shapes.

Non-Responsibilities
- Global CFG rewrites, SSA rebuild, or MIR semantics changes.

Gates
- Runner uses HAKO_AOT_PREP=1 to save “prepared” MIR sidecar after emit‑exe.
- Later we may switch compile input behind a dedicated flag once stable.

