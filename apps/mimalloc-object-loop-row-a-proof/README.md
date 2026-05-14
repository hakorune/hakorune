# MIMAP-012 MIR-ROW-A Object Loop Probe

Status: Active

Purpose:

```text
Prove the first dynamic object-loop acceptance row:
  loop + if guard + pages.get(i)
  scalar selected result only
  no helper call
  no nullable selected object field
  no dense proof reads
```

Acceptance:

```text
tools/checks/k2_wide_mimap012_object_loop_row_a_exe_guard.sh
```

Backend policy:

```text
LLVM/EXE is primary.
VM is diagnostic-only for object-heavy follow-up rows.
```

