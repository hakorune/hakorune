#!/usr/bin/env python3
"""Thin entrypoint for the MetadataContext.value_caller Rust-derived artifact."""

from __future__ import annotations

import argparse

from mirbuilder_metadata_value_caller_artifacts import run_metadata_value_caller_artifact_generator


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    run_metadata_value_caller_artifact_generator(check=args.check)


if __name__ == "__main__":
    main()
