#!/usr/bin/env python3
"""Selected ordinary-New checks owned by the canonical call corridor guard."""

from pathlib import Path
import sys


def fail(message: str) -> None:
    raise SystemExit(f"[mir-call-canonical-corridor-guard] {message}")


root = Path(sys.argv[1])
new_expression = (root / "src/mir/builder/new_expression.rs").read_text()
raw_dispatch = (root / "src/mir/builder/raw_expression_dispatch/mod.rs").read_text()
selected = (root / "src/mir/builder/ordinary_new_admission/selected.rs").read_text()
admission = (root / "src/mir/builder/ordinary_new_admission.rs").read_text()
if "usize::from(!self.selected_ordinary_claim) * arguments.len()" not in new_expression:
    fail("selected New no longer removes raw argument child demand")
new_start = raw_dispatch.index("node @ ASTNode::New { .. } => {")
new_end = raw_dispatch.index("\n            _ =>", new_start)
new_window = raw_dispatch[new_start:new_end]
prepare = "prepared.prepare_ordinary_claim_v1(self, port)?;"
queue = "prepared.evaluated_argument_count()"
if prepare not in new_window or queue not in new_window:
    fail("New dispatch lost selected claim preparation or child-demand queue")
if new_window.index(prepare) > new_window.index(queue):
    fail("selected New creates argument child demand before claim preparation")
selected_start = new_expression.index("if selected_ordinary_claim =>")
selected_end = new_expression.index("PreparedRawNewExpressionRouteV1::Ordinary { arguments } =>", selected_start)
selected_window = new_expression[selected_start:selected_end]
if "port.emit_ordinary_new_claim(" not in selected_window or "ordinary_claim.expect" not in selected_window:
    fail("selected New no longer enters the typed ordinary-New emitter")
if "drive_legacy_expression_v1" in selected_window or "lower_ordinary_raw_new_with_port_v1" in selected_window:
    fail("selected New retained raw argument carriage")
if "drive_legacy_expression_v1" in selected:
    fail("selected ordinary-New emitter retained raw AST argument descent")
for token in ("emit_integer", "emit_bool", "value_for_exact_binding"):
    if token not in selected:
        fail(f"selected ordinary-New emitter lost {token} materialization")
if "for arg in arguments" not in admission:
    fail("ordinary-New compatibility argument loop was removed outside selected cutover")
v0_module = (root / "src/runner/mir_json_v0/module.rs").read_text()
v0_start = v0_module.index('"call" | "mir_call" =>')
v0_end = v0_module.index('"externcall" =>', v0_start)
v0_window = v0_module[v0_start:v0_end]
for token in ("mir-json-v0/legacy-call-stopped", "JsonV0FunctionCatalog", "build_call_instruction", "LegacyCallV0"):
    if token in v0_window and token != "mir-json-v0/legacy-call-stopped":
        fail(f"stopped JSON-v0 call arm retained {token}")
if "mir-json-v0/legacy-call-stopped" not in v0_window:
    fail("JSON-v0 call arm lost its fail-fast terminal")
