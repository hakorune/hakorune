use std::env;
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{ArgAction, Parser, ValueEnum};

mod boundary_driver;
mod boundary_driver_ffi;
mod compile_input;
mod driver_dispatch;
mod harness_driver;
mod link_driver;
mod native_driver;

#[derive(Parser, Debug)]
#[command(
    name = "ny-llvmc",
    about = "Nyash LLVM backend helper CLI",
    long_about = "Compile MIR(JSON) into an object or executable.\n\
Stable caller contract: --in, --out, --emit, --dummy, --nyrt, --libs.\n\
Implementation detail: --driver and --harness are for backend bring-up / compat wrapper routing only."
)]
struct Args {
    /// MIR JSON input file path (use '-' to read from stdin). When omitted with --dummy, a dummy ny_main is emitted.
    #[arg(
        long = "in",
        value_name = "FILE",
        default_value = "-",
        help_heading = "Stable CLI"
    )]
    infile: String,

    /// Output path. For `--emit obj`, this is an object (.o). For `--emit exe`, this is an executable path.
    #[arg(long, value_name = "FILE", help_heading = "Stable CLI")]
    out: PathBuf,

    /// Generate a dummy object (ny_main -> i32 0). Ignores --in when set.
    #[arg(long, action = ArgAction::SetTrue, help_heading = "Stable CLI")]
    dummy: bool,

    /// Path to Python compat harness script (defaults to tools/llvmlite_harness.py in CWD)
    #[arg(long, value_name = "FILE", help_heading = "Implementation Detail")]
    harness: Option<PathBuf>,

    /// Object emission driver selector. Default enters the boundary-owned route.
    #[arg(
        long,
        value_enum,
        default_value_t = DriverKind::Boundary,
        help_heading = "Implementation Detail"
    )]
    driver: DriverKind,

    /// Emit kind: 'obj' (default) or 'exe'.
    #[arg(long, value_enum, default_value_t = EmitKind::Obj, help_heading = "Stable CLI")]
    emit: EmitKind,

    /// Path to directory containing libnyash_kernel.a when emitting an executable. Boundary may resolve a fallback, but Harness/Native exe linking requires an explicit --nyrt <DIR>.
    #[arg(long, value_name = "DIR", help_heading = "Stable CLI")]
    nyrt: Option<PathBuf>,

    /// Extra linker libs/flags appended when emitting an executable (single string, space-separated).
    #[arg(long, value_name = "FLAGS", help_heading = "Stable CLI")]
    libs: Option<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
enum EmitKind {
    Obj,
    Exe,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
enum DriverKind {
    Boundary,
    Harness,
    Native,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Ensure parent dir exists
    if let Some(parent) = args.out.parent() {
        std::fs::create_dir_all(parent).ok();
    }

    // Determine emit kind
    let emit_exe = matches!(args.emit, EmitKind::Exe);

    if args.dummy {
        return run_dummy_mode(&args, emit_exe);
    }

    run_compile_mode(&args, emit_exe)
}

fn run_dummy_mode(args: &Args, emit_exe: bool) -> Result<()> {
    let obj_path = compile_input::resolve_object_output_path(&args.out, emit_exe);
    driver_dispatch::emit_dummy_object_via_driver(args.driver, args.harness.as_ref(), &obj_path)
        .with_context(|| "failed to emit object in dummy mode")?;
    link_driver::finalize_emit_output(
        args.driver,
        &obj_path,
        &args.out,
        emit_exe,
        args.nyrt.as_ref(),
        args.libs.as_deref(),
        "dummy object",
    )
}

fn run_compile_mode(args: &Args, emit_exe: bool) -> Result<()> {
    let canary_norm = env::var("HAKO_LLVM_CANARY_NORMALIZE").ok().as_deref() == Some("1");
    let (input_path, temp_path) =
        compile_input::prepare_input_json_path(&args.infile, canary_norm)?;
    compile_input::ensure_input_json_exists(&input_path)?;
    compile_input::maybe_dump_input_json(&input_path);
    compile_input::emit_preflight_shape_hint(&input_path);

    let obj_path = compile_input::resolve_object_output_path(&args.out, emit_exe);
    compile_input::maybe_emit_verbose_shape_hint(&input_path, canary_norm);
    driver_dispatch::emit_compile_output(args, &input_path, &obj_path, emit_exe)?;

    // Cleanup temp file if used
    compile_input::cleanup_temp_input_json(temp_path);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::error::ErrorKind;
    use clap::CommandFactory;

    #[test]
    fn cli_contract_defaults_to_obj_from_stdin() {
        let args = Args::try_parse_from(["ny-llvmc", "--out", "/tmp/out.o"]).unwrap();
        assert_eq!(args.infile, "-");
        assert_eq!(args.emit, EmitKind::Obj);
        assert_eq!(args.driver, DriverKind::Boundary);
        assert_eq!(args.out, PathBuf::from("/tmp/out.o"));
        assert!(!args.dummy);
        assert!(args.nyrt.is_none());
        assert!(args.libs.is_none());
    }

    #[test]
    fn cli_contract_accepts_exe_runtime_and_libs_flags() {
        let args = Args::try_parse_from([
            "ny-llvmc",
            "--in",
            "input.mir.json",
            "--out",
            "out.exe",
            "--emit",
            "exe",
            "--nyrt",
            "target/release",
            "--libs=-lssl -lcrypto",
        ])
        .unwrap();
        assert_eq!(args.infile, "input.mir.json");
        assert_eq!(args.emit, EmitKind::Exe);
        assert_eq!(args.nyrt, Some(PathBuf::from("target/release")));
        assert_eq!(args.libs.as_deref(), Some("-lssl -lcrypto"));
    }

    #[test]
    fn implementation_detail_driver_selector_accepts_native_opt_in() {
        let args =
            Args::try_parse_from(["ny-llvmc", "--out", "out.o", "--driver", "native"]).unwrap();
        assert_eq!(args.driver, DriverKind::Native);
    }

    #[test]
    fn implementation_detail_driver_selector_accepts_harness_opt_in() {
        let args =
            Args::try_parse_from(["ny-llvmc", "--out", "out.o", "--driver", "harness"]).unwrap();
        assert_eq!(args.driver, DriverKind::Harness);
    }

    #[test]
    fn cli_contract_rejects_unknown_emit_kind() {
        let err =
            Args::try_parse_from(["ny-llvmc", "--out", "out.bin", "--emit", "ll"]).unwrap_err();
        assert_eq!(err.kind(), ErrorKind::InvalidValue);
    }

    #[test]
    fn help_text_marks_harness_as_implementation_detail() {
        let mut buf = Vec::new();
        Args::command().write_long_help(&mut buf).unwrap();
        let help = String::from_utf8(buf).unwrap();
        assert!(help.contains("Stable CLI"));
        assert!(help.contains("Implementation Detail"));
        assert!(help.contains("--harness <FILE>"));
        assert!(help.contains("--driver <DRIVER>"));
    }
}
