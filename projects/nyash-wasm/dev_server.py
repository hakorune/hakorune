#!/usr/bin/env python3
"""
Nyash WASM playground dev server.

Serves static files from projects/nyash-wasm and provides:
- POST /api/compile : compile .hako source to wasm bytes via hakorune --compile-wasm
"""

from __future__ import annotations

import json
import shutil
import subprocess
import tempfile
from http import HTTPStatus
from http.server import SimpleHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
from typing import Optional


SCRIPT_DIR = Path(__file__).resolve().parent
REPO_ROOT = SCRIPT_DIR.parent.parent
DEFAULT_PORT = 8001


def resolve_hakorune_bin() -> str:
    candidates = [
        REPO_ROOT / "target" / "release" / "hakorune",
        REPO_ROOT / "target" / "debug" / "hakorune",
    ]
    for path in candidates:
        if path.exists() and path.is_file():
            return str(path)
    from_path = shutil.which("hakorune")
    if from_path:
        return from_path
    raise FileNotFoundError(
        "hakorune binary not found. Build with `cargo build --release --bin hakorune` first."
    )


def json_error(handler: SimpleHTTPRequestHandler, status: int, message: str) -> None:
    payload = json.dumps({"ok": False, "error": message}).encode("utf-8")
    handler.send_response(status)
    handler.send_header("Content-Type", "application/json; charset=utf-8")
    handler.send_header("Content-Length", str(len(payload)))
    handler.end_headers()
    handler.wfile.write(payload)


class NyashPlaygroundHandler(SimpleHTTPRequestHandler):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, directory=str(SCRIPT_DIR), **kwargs)

    def do_GET(self) -> None:  # noqa: N802 (http handler method name)
        if self.path == "/api/health":
            payload = json.dumps({"ok": True, "service": "nyash-wasm-dev-server"}).encode("utf-8")
            self.send_response(HTTPStatus.OK)
            self.send_header("Content-Type", "application/json; charset=utf-8")
            self.send_header("Content-Length", str(len(payload)))
            self.end_headers()
            self.wfile.write(payload)
            return
        super().do_GET()

    def do_POST(self) -> None:  # noqa: N802 (http handler method name)
        if self.path != "/api/compile":
            json_error(self, HTTPStatus.NOT_FOUND, f"unsupported endpoint: {self.path}")
            return

        try:
            content_length = int(self.headers.get("Content-Length", "0"))
        except ValueError:
            json_error(self, HTTPStatus.BAD_REQUEST, "invalid Content-Length")
            return

        try:
            raw = self.rfile.read(content_length)
            payload = json.loads(raw.decode("utf-8"))
        except Exception as exc:  # noqa: BLE001
            json_error(self, HTTPStatus.BAD_REQUEST, f"invalid JSON payload: {exc}")
            return

        code = payload.get("code")
        if not isinstance(code, str) or not code.strip():
            json_error(self, HTTPStatus.BAD_REQUEST, "field `code` must be non-empty string")
            return

        with tempfile.TemporaryDirectory(prefix="nyash_wasm_compile_") as tmp_dir:
            tmp_path = Path(tmp_dir)
            src_path = tmp_path / "playground.hako"
            out_base = tmp_path / "playground_out"
            wasm_path = tmp_path / "playground_out.wasm"
            src_path.write_text(code, encoding="utf-8")

            result = try_compile_with_hakorune_bin(src_path, out_base)
            if should_retry_with_cargo(result):
                result = compile_with_cargo(src_path, out_base)

            if result.returncode != 0:
                stderr = result.stderr.strip()
                stdout = result.stdout.strip()
                detail = stderr or stdout or "compile-wasm failed"
                json_error(self, HTTPStatus.BAD_REQUEST, detail)
                return
            if not wasm_path.exists():
                json_error(self, HTTPStatus.INTERNAL_SERVER_ERROR, "compile succeeded but wasm output missing")
                return

            wasm_bytes = wasm_path.read_bytes()
            self.send_response(HTTPStatus.OK)
            self.send_header("Content-Type", "application/wasm")
            self.send_header("Content-Length", str(len(wasm_bytes)))
            self.end_headers()
            self.wfile.write(wasm_bytes)


def compile_with_command(cmd: list[str]) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        cmd,
        cwd=REPO_ROOT,
        text=True,
        capture_output=True,
        timeout=60,
        check=False,
    )


def try_compile_with_hakorune_bin(src_path: Path, out_base: Path) -> subprocess.CompletedProcess[str]:
    try:
        hakorune_bin = resolve_hakorune_bin()
    except FileNotFoundError:
        return subprocess.CompletedProcess(args=[], returncode=127, stdout="", stderr="hakorune binary not found")

    cmd = [
        hakorune_bin,
        "--compile-wasm",
        "-o",
        str(out_base),
        str(src_path),
    ]
    return compile_with_command(cmd)


def compile_with_cargo(src_path: Path, out_base: Path) -> subprocess.CompletedProcess[str]:
    cmd = [
        "cargo",
        "run",
        "--quiet",
        "--features",
        "wasm-backend",
        "--bin",
        "hakorune",
        "--",
        "--compile-wasm",
        "-o",
        str(out_base),
        str(src_path),
    ]
    return compile_with_command(cmd)


def should_retry_with_cargo(result: subprocess.CompletedProcess[str]) -> bool:
    if result.returncode == 0:
        return False
    stderr = result.stderr or ""
    stdout = result.stdout or ""
    joined = f"{stderr}\n{stdout}"
    return "WASM backend not available" in joined or "hakorune binary not found" in joined


def main() -> None:
    import argparse

    parser = argparse.ArgumentParser(description="Nyash WASM playground dev server")
    parser.add_argument("--port", type=int, default=DEFAULT_PORT)
    args = parser.parse_args()

    server = ThreadingHTTPServer(("0.0.0.0", args.port), NyashPlaygroundHandler)
    print(f"[nyash-wasm] serving {SCRIPT_DIR} on http://0.0.0.0:{args.port}")
    print("[nyash-wasm] compile endpoint: POST /api/compile")
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        pass
    finally:
        server.server_close()


if __name__ == "__main__":
    main()
