#!/usr/bin/env python3
"""Reusable structural guard for MODULEDRAFT0-HEADERPORT0.

P0 pins the disconnected capability: the invocation is external, its header
port is read-only, and the old direct-reader inventory is explicit. I0/G0 will
extend this guard with the inverse reader assertions after the atomic cutover.
"""

from __future__ import annotations

import pathlib
import sys


ROOT = pathlib.Path(__file__).resolve().parents[3]
BUILDER = ROOT / "src/mir/builder.rs"
COMPILATION = ROOT / "src/mir/builder/compilation_context.rs"
INVOCATION = ROOT / "src/mir/builder/module_lowering_invocation.rs"
PENDING_TERMINAL = ROOT / "src/mir/builder/calls/function_session/terminal.rs"

P0_DIRECT_HEADER_READER_FRAGMENTS = {
    "src/mir/builder/calls/annotation.rs": "module.functions.get(name)",
    "src/mir/builder/calls/lowering.rs": "self.current_module.as_ref()",
    "src/mir/builder/method_call_handlers.rs": "module.functions.get(&fname)",
    "src/mir/builder/rewrite/known.rs": "module.functions.get(fname)",
    "src/mir/builder/builder_method_index.rs": "module.functions.keys()",
    "src/mir/builder/calls/static_resolution.rs": "module\n                    .functions\n                    .keys()",
    "src/mir/builder/calls/materializer.rs": "module.functions.contains_key(name)",
    "src/mir/builder/builder_build.rs": "module.functions.contains_key(&lowered)",
}


def read(path: pathlib.Path) -> str:
    try:
        return path.read_text()
    except OSError as error:
        raise AssertionError(f"cannot read {path}: {error}") from error


def require(text: str, fragment: str, subject: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {subject}: {fragment!r}")


def forbid(text: str, fragment: str, subject: str) -> None:
    if fragment in text:
        raise AssertionError(f"forbidden {subject}: {fragment!r}")


def main() -> int:
    invocation = read(INVOCATION)
    builder = read(BUILDER)
    compilation = read(COMPILATION)
    pending_terminal = read(PENDING_TERMINAL)

    for fragment in (
        "struct LoweringHeaderPortV1",
        "struct ModuleLoweringPortV1",
        "struct ModuleLoweringInvocationV1",
        "collector: ModuleDraftCollectorV1",
        "builder: &'builder mut MirBuilder",
        "fn with_header_port",
        "fn with_module_port",
        "fn prepare_draft_admission",
    ):
        require(invocation, fragment, "HEADERPORT0 invocation vocabulary")

    header_impl = invocation.split("impl LoweringHeaderPortV1", 1)[1].split(
        "/// Stack-owned capability", 1
    )[0]
    for fragment in (
        "signature",
        "contains_symbol",
        "symbol_count",
        "visit_symbols",
    ):
        require(header_impl, fragment, "HEADERPORT0 read capability")
    for fragment in ("prepare", "collect", "MirFunction", "current_module"):
        forbid(header_impl, fragment, "HEADERPORT0 read capability")
    require(invocation, "for<'header>", "HEADERPORT0 non-escaping read borrow")
    require(invocation, "for<'port>", "RAWPORT0 non-escaping stack port")

    module_port_impl = invocation.split("struct ModuleLoweringPortV1", 1)[1].split(
        "/// The external owner", 1
    )[0]
    for fragment in ("collector: &'collector mut ModuleDraftCollectorV1", "with_headers", "prepare_draft_admission"):
        require(module_port_impl, fragment, "RAWPORT0 stack-owned capability")
    for fragment in ("MirBuilder", "current_module", "thread_local", "OnceLock"):
        forbid(module_port_impl, fragment, "RAWPORT0 stack-owned capability")

    pending_decl = pending_terminal.split("struct PendingFunctionSessionCloseV1", 1)[1].split(
        "impl<'builder> CanonicalFunctionLoweringSessionV1", 1
    )[0]
    for fragment in ("session: CanonicalFunctionLoweringSessionV1", "draft: Option<MirFunction>"):
        require(pending_decl, fragment, "RAWPORT0 pending terminal")
    for fragment in ("ModuleDraftCollectorV1", "MirBuilder", "current_module"):
        forbid(pending_decl, fragment, "RAWPORT0 pending terminal")
    forbid(pending_terminal, "derive(Clone)", "RAWPORT0 pending terminal")

    for fragment in ("thread_local", "OnceLock", "static ", "derive(Clone)"):
        forbid(invocation, fragment, "HEADERPORT0 external invocation")
    for fragment in ("LoweringHeaderPortV1", "ModuleDraftCollectorV1"):
        forbid(builder.split("pub struct MirBuilder", 1)[1].split("impl Default", 1)[0], fragment, "MirBuilder field")
        forbid(compilation, fragment, "CompilationContext field")

    for relative, fragment in P0_DIRECT_HEADER_READER_FRAGMENTS.items():
        require(read(ROOT / relative), fragment, f"P0 direct header reader {relative}")

    print(
        "[module-draft-headerport-guard] ok "
        f"p0_reader_families={len(P0_DIRECT_HEADER_READER_FRAGMENTS)}"
    )
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except AssertionError as error:
        print(f"[module-draft-headerport-guard] FAIL: {error}", file=sys.stderr)
        raise SystemExit(1)
