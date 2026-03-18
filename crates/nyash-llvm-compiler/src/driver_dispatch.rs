use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use super::{boundary_driver, link_driver, native_driver, DriverKind};

pub(super) fn emit_compile_output(
    args: &super::Args,
    input_path: &Path,
    obj_path: &Path,
    emit_exe: bool,
) -> Result<()> {
    emit_object_via_driver(args.driver, args.harness.as_ref(), input_path, obj_path).with_context(
        || {
            format!(
                "failed to compile MIR JSON via selected driver: {}",
                input_path.display()
            )
        },
    )?;
    link_driver::finalize_emit_output(
        args.driver,
        obj_path,
        &args.out,
        emit_exe,
        args.nyrt.as_ref(),
        args.libs.as_deref(),
        "object",
    )
}

pub(super) fn emit_dummy_object_via_driver(
    driver: DriverKind,
    harness: Option<&PathBuf>,
    out: &Path,
) -> Result<()> {
    match driver {
        DriverKind::Boundary => boundary_driver::emit_dummy_object(out),
        DriverKind::Harness => run_harness_dummy(harness, out),
        DriverKind::Native => run_native_dummy(out),
    }
}

pub(super) fn emit_object_via_driver(
    driver: DriverKind,
    harness: Option<&PathBuf>,
    input: &Path,
    out: &Path,
) -> Result<()> {
    match driver {
        DriverKind::Boundary => boundary_driver::emit_object_from_json(input, out),
        DriverKind::Harness => run_harness_in(harness, input, out),
        DriverKind::Native => run_native_in(input, out),
    }
}

fn run_harness_dummy(harness: Option<&PathBuf>, out: &Path) -> Result<()> {
    super::harness_driver::run_harness_dummy(harness, out)
}

fn run_harness_in(harness: Option<&PathBuf>, input: &Path, out: &Path) -> Result<()> {
    super::harness_driver::run_harness_in(harness, input, out)
}

fn run_native_dummy(out: &Path) -> Result<()> {
    native_driver::emit_dummy_object(out)
}

fn run_native_in(input: &Path, out: &Path) -> Result<()> {
    native_driver::emit_object_from_json(input, out)
}
