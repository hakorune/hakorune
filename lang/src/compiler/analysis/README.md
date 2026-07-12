# Compiler Analysis

Status: Active
Scope: read-only compiler observations that are not parser, MIRBuilder, plan,
backend, or runtime authority.

`bounded_body_snapshot_v0` defines the ProgramV0 wire observational quotient.
It must consume structured values only. Raw JSON scans, source-kind recovery,
MIR/ID allocation, route selection, lowering, and runtime behavior are
forbidden.
