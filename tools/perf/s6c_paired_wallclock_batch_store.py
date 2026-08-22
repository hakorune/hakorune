#!/usr/bin/env python3
"""Append-only filesystem owner for S6C MeasurementBatch V3 receipts."""

from __future__ import annotations

from contextlib import contextmanager
import fcntl
import hashlib
import json
import os
from pathlib import Path
import tempfile
from typing import Callable, Iterator

from s6c_paired_wallclock_batch import (
    SESSION_SLOTS, close_batch, issue_manifest, issue_session_terminal,
    validate_manifest,
)


MANIFEST_NAME = "manifest.json"
TERMINAL_NAME = "terminal.json"
BINARY_NAME = "meso-bench.frozen"
ALIGNMENT_NAME = "alignment.json"


class StoreError(RuntimeError):
    pass


def _json(value: object) -> str:
    return json.dumps(value, indent=2, sort_keys=True) + "\n"


def _fsync_directory(path: Path) -> None:
    descriptor = os.open(path, os.O_RDONLY | os.O_DIRECTORY)
    try:
        os.fsync(descriptor)
    finally:
        os.close(descriptor)


def create_bytes_once(path: Path, payload: bytes) -> None:
    """Publish one immutable file through a private temp plus exclusive link."""
    path.parent.mkdir(parents=True, exist_ok=True)
    temporary = path.with_name(f".{path.name}.{os.getpid()}.tmp")
    descriptor = os.open(temporary, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o600)
    try:
        with os.fdopen(descriptor, "wb") as stream:
            stream.write(payload)
            stream.flush()
            os.fsync(stream.fileno())
        try:
            os.link(temporary, path)
        except FileExistsError as error:
            raise StoreError(f"append-only path already exists: {path.name}") from error
        _fsync_directory(path.parent)
    finally:
        temporary.unlink(missing_ok=True)


def create_once(path: Path, payload: str) -> None:
    create_bytes_once(path, payload.encode())


def read_json(path: Path) -> dict[str, object]:
    try:
        value = json.loads(path.read_text())
    except (OSError, json.JSONDecodeError) as error:
        raise StoreError(f"unreadable receipt: {path.name}") from error
    if not isinstance(value, dict):
        raise StoreError(f"non-object receipt: {path.name}")
    return value


def session_path(directory: Path, slot: int) -> Path:
    return directory / f"session-{slot}.terminal.json"


def raw_path(directory: Path, slot: int) -> Path:
    return directory / f"session-{slot}.raw.csv"


def diagnostic_raw_path(directory: Path, slot: int) -> Path:
    return directory / f"session-{slot}.diagnostic.raw.csv"


def create_batch(
        root: Path, manifest: dict[str, object], *, frozen_binary: bytes,
        alignment_payload: str) -> Path:
    validate_manifest(manifest)
    if hashlib.sha256(frozen_binary).hexdigest() != manifest["candidate"]["binary_sha256"] or \
            hashlib.sha256(alignment_payload.encode()).hexdigest() != \
            manifest["candidate"]["alignment_sha256"]:
        raise StoreError("frozen candidate payload drift")
    root.mkdir(parents=True, exist_ok=True)
    directory = root / str(manifest["batch_id"])
    try:
        directory.mkdir()
    except FileExistsError as error:
        raise StoreError("batch ID already exists") from error
    try:
        create_once(directory / MANIFEST_NAME, _json(manifest))
        create_bytes_once(directory / BINARY_NAME, frozen_binary)
        (directory / BINARY_NAME).chmod(0o500)
        create_once(directory / ALIGNMENT_NAME, alignment_payload)
        _fsync_directory(root)
    except Exception:
        # An empty/pre-manifest directory has no batch authority and is left visible.
        raise
    return directory


def load_manifest(directory: Path) -> dict[str, object]:
    manifest = read_json(directory / MANIFEST_NAME)
    validate_manifest(manifest)
    if directory.name != manifest.get("batch_id"):
        raise StoreError("batch directory/manifest identity drift")
    return manifest


@contextmanager
def exclusive_batch(directory: Path) -> Iterator[None]:
    lock_path = directory / ".batch.lock"
    descriptor = os.open(lock_path, os.O_RDWR | os.O_CREAT, 0o600)
    try:
        try:
            fcntl.flock(descriptor, fcntl.LOCK_EX | fcntl.LOCK_NB)
        except BlockingIOError as error:
            raise StoreError("batch already has an active consumer") from error
        yield
    finally:
        os.close(descriptor)


def publish_complete_session(
        directory: Path, manifest: dict[str, object], *, slot: int,
        outcome: str, raw_csv: str) -> dict[str, object]:
    if (directory / TERMINAL_NAME).exists() or session_path(directory, slot).exists():
        raise StoreError("terminal batch/session cannot be reopened")
    raw_digest = hashlib.sha256(raw_csv.encode()).hexdigest()
    receipt = issue_session_terminal(
        manifest, slot=slot, terminal_state="Complete", outcome=outcome,
        raw_csv_sha256=raw_digest)
    create_once(raw_path(directory, slot), raw_csv)
    create_once(session_path(directory, slot), _json(receipt))
    return receipt


def publish_ineligible_session(
        directory: Path, manifest: dict[str, object], *, slot: int,
        terminal_state: str, reason: str,
        diagnostic_raw_csv: str | None = None) -> dict[str, object]:
    if (directory / TERMINAL_NAME).exists() or session_path(directory, slot).exists():
        raise StoreError("terminal batch/session cannot be reopened")
    diagnostic_digest = None
    if diagnostic_raw_csv is not None:
        diagnostic_digest = hashlib.sha256(diagnostic_raw_csv.encode()).hexdigest()
        path = diagnostic_raw_path(directory, slot)
        if path.exists():
            if hashlib.sha256(path.read_bytes()).hexdigest() != diagnostic_digest:
                raise StoreError("diagnostic raw payload drift")
        else:
            create_once(path, diagnostic_raw_csv)
    receipt = issue_session_terminal(
        manifest, slot=slot, terminal_state=terminal_state, reason=reason,
        diagnostic_raw_csv_sha256=diagnostic_digest)
    create_once(session_path(directory, slot), _json(receipt))
    return receipt


def _verify_session_payload(
        directory: Path, slot: int, receipt: dict[str, object]) -> None:
    expected = (
        (raw_path(directory, slot), receipt.get("raw_csv_sha256")),
        (diagnostic_raw_path(directory, slot),
         receipt.get("diagnostic_raw_csv_sha256")),
    )
    for path, digest in expected:
        if digest is None:
            if path.exists():
                raise StoreError(f"unbound session payload: {path.name}")
        elif not path.is_file() or hashlib.sha256(path.read_bytes()).hexdigest() != digest:
            raise StoreError(f"session payload digest drift: {path.name}")


def close(directory: Path) -> dict[str, object]:
    if (directory / TERMINAL_NAME).exists():
        raise StoreError("terminal batch cannot be reopened")
    manifest = load_manifest(directory)
    receipts = [read_json(session_path(directory, slot)) for slot in SESSION_SLOTS]
    for slot, receipt in zip(SESSION_SLOTS, receipts):
        _verify_session_payload(directory, slot, receipt)
    terminal = close_batch(manifest, receipts)
    create_once(directory / TERMINAL_NAME, _json(terminal))
    return terminal


def close_abandoned(directory: Path, reason: str) -> dict[str, object]:
    if not reason:
        raise StoreError("abandoned batch requires a stable reason")
    with exclusive_batch(directory):
        manifest = load_manifest(directory)
        for slot in SESSION_SLOTS:
            if not session_path(directory, slot).exists():
                diagnostic_path = diagnostic_raw_path(directory, slot)
                complete_path = raw_path(directory, slot)
                if complete_path.exists():
                    if diagnostic_path.exists():
                        raise StoreError("ambiguous orphan raw payloads")
                    os.rename(complete_path, diagnostic_path)
                    _fsync_directory(directory)
                publish_ineligible_session(
                    directory, manifest, slot=slot, terminal_state="Incomplete",
                    reason=reason,
                    diagnostic_raw_csv=diagnostic_path.read_text()
                    if diagnostic_path.exists() else None)
        return close(directory)


def _lock_probe(directory: Path) -> None:
    with exclusive_batch(directory):
        pass


def _expect_error(action: Callable[[], object]) -> None:
    try:
        action()
    except (StoreError, ValueError):
        return
    raise AssertionError("invalid append-only store transition accepted")


def self_test() -> None:
    binary, alignment = b"exact-binary", "{\"alignment\":true}\n"
    identity = dict(commit="a" * 40, binary_sha256=hashlib.sha256(binary).hexdigest(),
                    build_id="c" * 40,
                    alignment_sha256=hashlib.sha256(alignment.encode()).hexdigest(), cpu=0)
    with tempfile.TemporaryDirectory(prefix="s6c-batch-store-") as temporary:
        root = Path(temporary)
        manifest = issue_manifest(**identity)
        complete_dir = create_batch(
            root, manifest, frozen_binary=binary, alignment_payload=alignment)
        with exclusive_batch(complete_dir):
            _expect_error(lambda: _lock_probe(complete_dir))
            publish_complete_session(
                complete_dir, manifest, slot=0, outcome="development_green",
                raw_csv="header\nslot0\n")
            publish_complete_session(
                complete_dir, manifest, slot=1, outcome="development_green",
                raw_csv="header\nslot1\n")
            terminal = close(complete_dir)
        assert terminal["classification"] == "development_keeper"
        _expect_error(lambda: create_batch(
            root, manifest, frozen_binary=binary, alignment_payload=alignment))
        _expect_error(lambda: close(complete_dir))
        _expect_error(lambda: publish_complete_session(
            complete_dir, manifest, slot=0, outcome="development_green", raw_csv="again"))

        successor = issue_manifest(
            **identity, predecessor=terminal, repeat_reason="confirmatory_development")
        abandoned = create_batch(
            root, successor, frozen_binary=binary, alignment_payload=alignment)
        create_once(diagnostic_raw_path(abandoned, 0), "partial,diagnostic,only\n")
        abandoned_terminal = close_abandoned(abandoned, "controller_interrupted")
        assert abandoned_terminal["terminal_state"] == "Incomplete"
        assert all(session_path(abandoned, slot).exists() for slot in SESSION_SLOTS)
        assert diagnostic_raw_path(abandoned, 0).read_text() == "partial,diagnostic,only\n"
        receipt = read_json(session_path(abandoned, 0))
        assert receipt["diagnostic_raw_csv_sha256"] == hashlib.sha256(
            b"partial,diagnostic,only\n").hexdigest()
        _expect_error(lambda: publish_ineligible_session(
            abandoned, manifest, slot=0, terminal_state="Incomplete",
            reason="again", diagnostic_raw_csv="changed\n"))
        assert not any(abandoned.glob("*.tmp"))

        recovered_manifest = issue_manifest(
            **identity, predecessor=abandoned_terminal,
            repeat_reason="incomplete_predecessor")
        recovered = create_batch(
            root, recovered_manifest, frozen_binary=binary,
            alignment_payload=alignment)
        create_once(raw_path(recovered, 0), "orphan-complete-raw\n")
        recovered_terminal = close_abandoned(recovered, "controller_interrupted")
        assert recovered_terminal["terminal_state"] == "Incomplete"
        assert not raw_path(recovered, 0).exists()
        assert diagnostic_raw_path(recovered, 0).read_text() == "orphan-complete-raw\n"


if __name__ == "__main__":
    self_test()
    print("[s6c-paired-wallclock-batch-store] self-test ok")
