#!/usr/bin/env python3
"""Thin entrypoint for the BindingContext Rust-derived Hako artifact."""

from __future__ import annotations

import argparse

from mirbuilder_family_artifacts import run_mirbuilder_family_artifact_generator


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true", help="fail if generated files differ")
    args = parser.parse_args()
    run_mirbuilder_family_artifact_generator("binding_context", check=args.check)


if __name__ == "__main__":
    main()
