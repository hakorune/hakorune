#!/usr/bin/env python3
"""Build and validate the observation-only R7 legacy census manifest.

The manifest records source anchors and scope.  It never feeds compiler
meaning, selects a backend, or authorizes LegacyCallV0 retirement.
"""

from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path
import re
import subprocess
import sys
from typing import Any


KIND = "MirR7LegacyCensusManifestV1"
SCHEMA_VERSION = 1
LEGACY_TOKEN = "LegacyCallV0"
LEGACY_EXPECTED = 243
LEGACY_FILES_EXPECTED = 127
ENV_EXPECTED = 4
LOOP_EXPECTED = 4
LOOP_LOC_EXPECTED = 1009

DECL_RE = re.compile(
    r"^\s*(?:(?:pub)(?:\([^)]*\))?\s+)?"
    r"(?:(?:async)\s+)?(?:fn|struct|enum|trait|impl|mod)\s+([A-Za-z_]\w*)"
)

ENV_ANCHORS = (
    ("src/host_providers/llvm_codegen/capi_transport.rs", 247, "compile_via_capi", "rust_capi"),
    ("src/host_providers/llvm_codegen/static_invocation.rs", 6, "compile_published_static_v2", "rust_static"),
    ("lang/c-abi/shims/hako_llvmc_ffi_route.inc", 358, "hako_llvmc_forward_link_to_aot_without_ffi", "c_route"),
    ("lang/c-abi/shims/hako_llvmc_ffi_route.inc", 372, "hako_llvmc_forward_link_to_aot_v2_without_ffi", "c_route"),
)

LOOP_FILES = (
    ("src/mir/builder/resolved_lowering/dynamic_loop_discard_tests.rs", 295, "dynamic_loop_discard_tests"),
    ("src/mir/builder/resolved_lowering/dynamic_loop_phi_tests.rs", 266, "dynamic_loop_phi_tests"),
    ("src/mir/builder/resolved_lowering/dynamic_loop_rebind_tests.rs", 305, "dynamic_loop_rebind_tests"),
    ("src/mir/global_call_route_plan/tests/void_sentinel/guards_and_loops/loop_scalar_phi_substring_void_sentinel_body.rs", 143, "loop_scalar_phi_substring_void_sentinel_body"),
)


def fail(message: str) -> None:
    print(f"[mir-r7-legacy-census] ERROR: {message}", file=sys.stderr)
    raise SystemExit(1)


def sha256_bytes(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def source_sha(path: Path) -> str:
    try:
        return sha256_bytes(path.read_bytes())
    except OSError as exc:
        fail(f"cannot read source {path}: {exc}")


def git_revision(root: Path) -> str:
    try:
        value = subprocess.run(
            ["git", "rev-parse", "HEAD"],
            cwd=root,
            check=True,
            capture_output=True,
            text=True,
        ).stdout.strip()
    except (OSError, subprocess.CalledProcessError) as exc:
        fail(f"cannot read git revision: {exc}")
    if not re.fullmatch(r"[0-9a-f]{40}", value):
        fail("git revision is not a full hexadecimal commit")
    return value


def is_test_path(path: str) -> bool:
    p = Path(path)
    return (
        "tests" in p.parts
        or "test" in p.parts
        or p.name in {"test.rs", "tests.rs"}
        or p.name.startswith("test_")
        or p.name.endswith("_test.rs")
        or p.name.endswith("_tests.rs")
    )


def brace_delta(line: str) -> int:
    # These files use ordinary Rust module braces.  Keeping this scanner
    # lexical and conservative is intentional: it is an observation census,
    # not a Rust parser or semantic classifier.
    return line.count("{") - line.count("}")


def cfg_test_lines(lines: list[str]) -> set[int]:
    """Return 1-based lines inside #[cfg(test)] modules."""

    skipped: set[int] = set()
    pending = False
    depth = 0
    for number, line in enumerate(lines, 1):
        if depth:
            skipped.add(number)
            depth += brace_delta(line)
            if depth <= 0:
                depth = 0
            continue
        if pending:
            skipped.add(number)
            if "{" in line:
                depth = brace_delta(line)
                pending = False
            elif ";" in line:
                # An out-of-line `#[cfg(test)] mod tests;` has no body in
                # this file.  Do not hide the rest of the production module.
                pending = False
            continue
        if "#[cfg(test)]" in line:
            skipped.add(number)
            if "{" in line:
                depth = brace_delta(line)
            else:
                pending = True
    return skipped


def enclosing_symbols(lines: list[str]) -> list[str | None]:
    current: str | None = None
    symbols: list[str | None] = [None]
    for line in lines:
        match = DECL_RE.match(line)
        if match:
            current = match.group(1)
        symbols.append(current)
    return symbols


def legacy_role(path: str, line: str) -> tuple[str, str, bool]:
    if path.endswith("calls/unified_emitter/compat_entrypoints.rs"):
        return "compatibility_constructor", "explicit compatibility", False
    if path == "src/runner/mir_json_v0/module.rs":
        return "compatibility_constructor", "json-v0 boxcall ingress", False
    if path.endswith("joinir_id_remapper.rs") and "=> LegacyCallV0" in line:
        return "mechanical_reissuer", "id remap only", False
    if "mir_json_emit" in path or path.endswith("runner/product/llvm/mod.rs"):
        return "reader_projection", "compatibility projection", False
    if "callsite_canonicalize" in path:
        return "canonicalizer_input", "selected native corridor input", True
    if path.endswith("inline_leaf.rs") or "backend_core_ops" in path:
        return "diagnostic_or_contract", "reject/contract observation", False
    if line.lstrip().startswith(("//", "*", "//!")):
        return "comment_or_contract", "non-executable reference", False
    return "production_surface", "production lexical surface", False


def legacy_rows(root: Path) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    for base in (root / "src", root / "crates"):
        if not base.is_dir():
            continue
        for path in sorted(base.rglob("*.rs")):
            rel = path.relative_to(root).as_posix()
            if is_test_path(rel):
                continue
            lines = path.read_text(encoding="utf-8", errors="replace").splitlines()
            skipped = cfg_test_lines(lines)
            symbols = enclosing_symbols(lines)
            digest = source_sha(path)
            for number, line in enumerate(lines, 1):
                if number in skipped or LEGACY_TOKEN not in line:
                    continue
                symbol = symbols[number] or "file-scope"
                role, disposition, selected = legacy_role(rel, line)
                material = f"legacy|{rel}|{number}|{symbol}|{LEGACY_TOKEN}"
                rows.append(
                    {
                        "id": f"r7.legacy.{hashlib.sha256(material.encode()).hexdigest()[:16]}",
                        "kind": "legacy_occurrence",
                        "path": rel,
                        "line": number,
                        "symbol": symbol,
                        "token": LEGACY_TOKEN,
                        "line_text": line.strip(),
                        "source_sha256": digest,
                        "role": role,
                        "profile": "selected_native" if selected else ("json_v0" if "json_v0" in rel else "compatibility_surface"),
                        "reachability": "production_scope",
                        "func_authority": "carrier" if "func" in line else "not_present",
                        "selected_corridor": selected,
                        "owner": rel,
                        "disposition": disposition,
                        "retire_when": "retain until an explicit replacement reaches caller-zero; do not infer deletion from this census",
                        "reopen_trigger": "new non-test constructor/reissuer or selected backend consumption",
                    }
                )
    rows.sort(key=lambda row: (row["path"], row["line"], row["symbol"], row["id"]))
    return rows


def require_anchor(root: Path, rel: str, line: int, symbol: str) -> tuple[list[str], str]:
    path = root / rel
    if not path.is_file():
        fail(f"anchor file missing: {rel}")
    lines = path.read_text(encoding="utf-8", errors="replace").splitlines()
    if line < 1 or line > len(lines) or symbol not in lines[line - 1]:
        fail(f"anchor drift: {rel}:{line}:{symbol}")
    return lines, source_sha(path)


def env_rows(root: Path) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    for rel, line, symbol, family in ENV_ANCHORS:
        lines, digest = require_anchor(root, rel, line, symbol)
        material = f"env|{rel}|{line}|{symbol}|compile-env"
        rows.append(
            {
                "id": f"r7.env.{hashlib.sha256(material.encode()).hexdigest()[:16]}",
                "kind": "compile_env_route",
                "path": rel,
                "line": line,
                "symbol": symbol,
                "token": "compile-env save/set/restore",
                "line_text": lines[line - 1].strip(),
                "source_sha256": digest,
                "role": "environment_route",
                "profile": family,
                "reachability": "production_scope",
                "func_authority": "configuration_only",
                "selected_corridor": False,
                "owner": rel,
                "disposition": "process-scoped environment observation; no semantic authority",
                "retire_when": "replace only after invocation-owned settings preserve route and restoration semantics",
                "reopen_trigger": "new save/set/restore helper or route family",
            }
        )
    return rows


def loop_rows(root: Path) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    for rel, loc, symbol in LOOP_FILES:
        path = root / rel
        if not path.is_file():
            fail(f"Loop-PHI file missing: {rel}")
        lines = path.read_text(encoding="utf-8", errors="replace").splitlines()
        if len(lines) != loc:
            fail(f"Loop-PHI LOC drift: {rel}: manifest={loc} source={len(lines)}")
        material = f"loop_phi|{rel}|1|{symbol}|dynamic-loop-phi"
        rows.append(
            {
                "id": f"r7.loop_phi.{hashlib.sha256(material.encode()).hexdigest()[:16]}",
                "kind": "test_only_loop_phi_file",
                "path": rel,
                "line": 1,
                "symbol": symbol,
                "token": "dynamic-loop-phi test-only residue",
                "line_text": lines[0].strip() if lines else "",
                "source_sha256": source_sha(path),
                "loc_count": loc,
                "role": "canary_inventory",
                "profile": "test_only",
                "reachability": "no_production_caller_observed",
                "func_authority": "none",
                "selected_corridor": False,
                "owner": rel,
                "disposition": "retain as test-only inventory; not production evidence",
                "production_callers": 0,
                "retire_when": "remove only with a separate test inventory decision and replacement evidence",
                "reopen_trigger": "a production caller is added or file scope/LOC drifts",
            }
        )
    return rows


def build_manifest(root: Path) -> dict[str, Any]:
    legacy = legacy_rows(root)
    env = env_rows(root)
    loop = loop_rows(root)
    rows = legacy + env + loop
    anchors = [f"{row['path']}:{row['line']}:{row['token']}" for row in rows]
    scope_digest = sha256_bytes("\n".join(sorted(anchors)).encode())
    return {
        "schema_version": SCHEMA_VERSION,
        "kind": KIND,
        "observed_commit": git_revision(root),
        "scope": {
            "roots": ["src (Rust production scope)", "crates (Rust production scope)", "lang/c-abi route (fixed env anchors)"],
            "excludes": ["tests and fixtures", "#[cfg(test)] modules", "unrelated runtime/startup env", "runtime hook registry", "future VM/WASM parity"],
            "classifier": "lexical LegacyCallV0 rows + fixed compile-env anchors + fixed test-only Loop-PHI files",
            "scope_sha256": scope_digest,
        },
        "counts": {
            "legacy_occurrences": len(legacy),
            "legacy_files": len({row["path"] for row in legacy}),
            "compile_env_routes": len(env),
            "compile_env_families": len({row["profile"] for row in env}),
            "loop_phi_files": len(loop),
            "loop_phi_loc": sum(row["loc_count"] for row in loop),
            "independent_rows": len(rows),
        },
        "boundary": {
            "start": "canonical/compatibility MIR ingress + compile profile",
            "end": "LegacyCallV0 constructors/reissuer/readers/egress -> selected artifact reject or compatibility terminal; test-only Loop-PHI -> production-caller census terminal",
            "no_semantic_input": True,
            "no_retirement_claim": True,
        },
        "rows": rows,
    }


def validate_manifest(root: Path, data: dict[str, Any]) -> None:
    if data.get("schema_version") != SCHEMA_VERSION or data.get("kind") != KIND:
        fail("schema_version/kind mismatch")
    current = build_manifest(root)
    # The manifest itself is committed after it is written, so its pinned
    # observation commit is normally the parent of the commit running this
    # check.  Compare every generated field except that pin; source hashes and
    # the scope digest still make source drift fail closed.
    observed_commit = data.get("observed_commit")
    if not isinstance(observed_commit, str) or not re.fullmatch(r"[0-9a-f]{40}", observed_commit):
        fail("observed_commit must be a full hexadecimal commit")
    try:
        subprocess.run(
            ["git", "cat-file", "-e", f"{observed_commit}^{{commit}}"],
            cwd=root,
            check=True,
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL,
        )
    except (OSError, subprocess.CalledProcessError) as exc:
        fail(f"observed_commit is not reachable: {observed_commit}: {exc}")
    expected = dict(current)
    expected["observed_commit"] = observed_commit
    if data != expected:
        fail("manifest drift: run with --write after a source or scope change")
    counts = current["counts"]
    expected = {
        "legacy_occurrences": LEGACY_EXPECTED,
        "legacy_files": LEGACY_FILES_EXPECTED,
        "compile_env_routes": ENV_EXPECTED,
        "loop_phi_files": LOOP_EXPECTED,
        "loop_phi_loc": LOOP_LOC_EXPECTED,
        "independent_rows": LEGACY_EXPECTED + ENV_EXPECTED + LOOP_EXPECTED,
    }
    for key, value in expected.items():
        if counts.get(key) != value:
            fail(f"{key} drift: expected={value} observed={counts.get(key)}")
    ids = [row.get("id") for row in current["rows"]]
    if len(ids) != len(set(ids)):
        fail("duplicate row id")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", default=".")
    parser.add_argument("--manifest", default="tools/checks/manifests/mir_r7_legacy_census_manifest_v1.json")
    parser.add_argument("--write", action="store_true", help="write the deterministic manifest")
    args = parser.parse_args()
    root = Path(args.root).resolve()
    path = root / args.manifest
    if args.write:
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(json.dumps(build_manifest(root), indent=2, sort_keys=True) + "\n", encoding="utf-8")
        print(f"[mir-r7-legacy-census] wrote {path}")
        return 0
    try:
        data = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as exc:
        fail(f"cannot read manifest {path}: {exc}")
    if not isinstance(data, dict):
        fail("manifest root must be an object")
    validate_manifest(root, data)
    counts = data["counts"]
    print(
        "[mir-r7-legacy-census] ok "
        f"rows={counts['independent_rows']} "
        f"legacy={counts['legacy_occurrences']} "
        f"env={counts['compile_env_routes']} "
        f"loop_phi={counts['loop_phi_files']}"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
