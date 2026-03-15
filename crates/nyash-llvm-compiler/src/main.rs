use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{bail, Context, Result};
use clap::{ArgAction, Parser, ValueEnum};
use serde_json::Value as JsonValue;

mod native_driver;

#[derive(Parser, Debug)]
#[command(
    name = "ny-llvmc",
    about = "Nyash LLVM compiler (llvmlite harness wrapper)",
    long_about = "Compile MIR(JSON) into an object or executable.\n\
Stable caller contract: --in, --out, --emit, --dummy, --nyrt, --libs.\n\
Implementation detail: --driver and --harness are for backend bring-up / wrapper routing only."
)]
struct Args {
    /// MIR JSON input file path (use '-' to read from stdin). When omitted with --dummy, a dummy ny_main is emitted.
    #[arg(long = "in", value_name = "FILE", default_value = "-", help_heading = "Stable CLI")]
    infile: String,

    /// Output path. For `--emit obj`, this is an object (.o). For `--emit exe`, this is an executable path.
    #[arg(long, value_name = "FILE", help_heading = "Stable CLI")]
    out: PathBuf,

    /// Generate a dummy object (ny_main -> i32 0). Ignores --in when set.
    #[arg(long, action = ArgAction::SetTrue, help_heading = "Stable CLI")]
    dummy: bool,

    /// Path to Python harness script (defaults to tools/llvmlite_harness.py in CWD)
    #[arg(long, value_name = "FILE", help_heading = "Implementation Detail")]
    harness: Option<PathBuf>,

    /// Object emission driver selector. Default keeps the current llvmlite wrapper route.
    #[arg(
        long,
        value_enum,
        default_value_t = DriverKind::Harness,
        help_heading = "Implementation Detail"
    )]
    driver: DriverKind,

    /// Emit kind: 'obj' (default) or 'exe'.
    #[arg(long, value_enum, default_value_t = EmitKind::Obj, help_heading = "Stable CLI")]
    emit: EmitKind,

    /// Path to directory containing libnyash_kernel.a when emitting an executable. If omitted, searches target/release then crates/nyash_kernel/target/release.
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
    Harness,
    Native,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Ensure parent dir exists
    if let Some(parent) = args.out.parent() {
        std::fs::create_dir_all(parent).ok();
    }

    // Resolve harness path
    let harness_path = resolve_harness_path(args.harness.clone());

    // Determine emit kind
    let emit_exe = matches!(args.emit, EmitKind::Exe);

    if args.dummy {
        // Dummy ny_main: always go through harness to produce an object then link if requested
        let obj_path = resolve_object_output_path(&args.out, emit_exe);
        emit_dummy_object_via_driver(args.driver, &harness_path, &obj_path)
            .with_context(|| "failed to emit object in dummy mode")?;
        if emit_exe {
            link_executable(
                &obj_path,
                &args.out,
                args.nyrt.as_ref(),
                args.libs.as_deref(),
            )?;
            println!("[ny-llvmc] executable written: {}", args.out.display());
        } else {
            println!("[ny-llvmc] dummy object written: {}", obj_path.display());
        }
        return Ok(());
    }

    let canary_norm = env::var("HAKO_LLVM_CANARY_NORMALIZE").ok().as_deref() == Some("1");
    let (input_path, temp_path) = prepare_input_json_path(&args.infile, canary_norm)?;

    if !input_path.exists() {
        bail!("input JSON not found: {}", input_path.display());
    }

    // Optional: dump incoming MIR JSON for diagnostics (AotPrep 後の入力を観測)
    if let Ok(dump_path) = env::var("NYASH_LLVM_DUMP_MIR_IN") {
        let _ = std::fs::copy(&input_path, &dump_path);
        eprintln!("[ny-llvmc] dumped MIR input to {}", dump_path);
    }

    // Optional: preflight shape/hints (best-effort; no behavior change)
    if let Ok(s) = std::fs::read_to_string(&input_path) {
        if let Ok(val) = serde_json::from_str::<JsonValue>(&s) {
            if let Some(hint) = shape_hint(&val) {
                eprintln!("[ny-llvmc/hint] {}", hint);
            }
        }
    }

    // Produce object first
    let obj_path = resolve_object_output_path(&args.out, emit_exe);

    // Optional: print concise shape hint in verbose mode when not normalizing
    if env::var("NYASH_CLI_VERBOSE").ok().as_deref() == Some("1")
        && env::var("HAKO_LLVM_CANARY_NORMALIZE").ok().as_deref() != Some("1")
    {
        if let Ok(mut f) = File::open(&input_path) {
            let mut buf = String::new();
            if f.read_to_string(&mut buf).is_ok() {
                if let Ok(val) = serde_json::from_str::<serde_json::Value>(&buf) {
                    if let Some(h) = shape_hint(&val) {
                        eprintln!("[ny-llvmc/hint] {}", h);
                    }
                }
            }
        }
    }

    emit_object_via_driver(args.driver, &harness_path, &input_path, &obj_path).with_context(|| {
        format!(
            "failed to compile MIR JSON via selected driver: {}",
            input_path.display()
        )
    })?;
    if emit_exe {
        link_executable(
            &obj_path,
            &args.out,
            args.nyrt.as_ref(),
            args.libs.as_deref(),
        )?;
        println!("[ny-llvmc] executable written: {}", args.out.display());
    } else {
        println!("[ny-llvmc] object written: {}", obj_path.display());
    }

    // Cleanup temp file if used
    cleanup_temp_input_json(temp_path);

    Ok(())
}

fn resolve_harness_path(harness: Option<PathBuf>) -> PathBuf {
    harness.unwrap_or_else(|| PathBuf::from("tools/llvmlite_harness.py"))
}

fn resolve_object_output_path(out: &Path, emit_exe: bool) -> PathBuf {
    if emit_exe {
        let mut path = out.to_path_buf();
        path.set_extension("o");
        path
    } else {
        out.to_path_buf()
    }
}

fn prepare_input_json_path(infile: &str, canary_norm: bool) -> Result<(PathBuf, Option<PathBuf>)> {
    if infile == "-" {
        let value = read_input_json_value_from_stdin(canary_norm)?;
        let tmp = write_temp_input_json("ny_llvmc_stdin.json", &value)?;
        return Ok((tmp.clone(), Some(tmp)));
    }

    let input_path = PathBuf::from(infile);
    if canary_norm {
        let value = read_input_json_value_from_path(&input_path, canary_norm)?;
        let tmp = write_temp_input_json("ny_llvmc_in.json", &value)?;
        Ok((tmp.clone(), Some(tmp)))
    } else {
        Ok((input_path, None))
    }
}

fn read_input_json_value_from_stdin(canary_norm: bool) -> Result<serde_json::Value> {
    let mut buf = String::new();
    std::io::stdin()
        .read_to_string(&mut buf)
        .context("reading MIR JSON from stdin")?;
    parse_input_json_value(&buf, "stdin does not contain valid JSON", canary_norm)
}

fn read_input_json_value_from_path(path: &Path, canary_norm: bool) -> Result<serde_json::Value> {
    let mut buf = String::new();
    File::open(path)
        .and_then(|mut f| f.read_to_string(&mut buf))
        .context("read input json")?;
    parse_input_json_value(&buf, "input is not valid JSON", canary_norm)
}

fn parse_input_json_value(
    input: &str,
    invalid_context: &'static str,
    canary_norm: bool,
) -> Result<serde_json::Value> {
    let mut value: serde_json::Value = serde_json::from_str(input).context(invalid_context)?;
    if canary_norm {
        value = normalize_canary_json(value);
    }
    Ok(value)
}

fn write_temp_input_json(filename: &str, value: &serde_json::Value) -> Result<PathBuf> {
    let tmp = std::env::temp_dir().join(filename);
    let mut file = File::create(&tmp).context("create temp json file")?;
    let serialized = serde_json::to_vec(value).context("serialize normalized json")?;
    file.write_all(&serialized).context("write temp json")?;
    Ok(tmp)
}

fn cleanup_temp_input_json(temp_path: Option<PathBuf>) {
    if let Some(path) = temp_path {
        let _ = std::fs::remove_file(path);
    }
}

/// Return a concise hint if the MIR JSON likely has a schema/shape mismatch for the Python harness.
fn shape_hint(v: &JsonValue) -> Option<String> {
    // Accept both v0/v1 tolerant; only emit hint on common canary shapes
    // 1) schema_version numeric 1 rather than string "1.0"
    if let Some(sv) = v.get("schema_version") {
        if sv.is_number() {
            if sv.as_i64() == Some(1) {
                return Some("schema_version=1 detected; set to \"1.0\" or enable HAKO_LLVM_CANARY_NORMALIZE=1".into());
            }
        } else if sv.as_str() == Some("1") {
            return Some("schema_version=\"1\" detected; prefer \"1.0\" or enable HAKO_LLVM_CANARY_NORMALIZE=1".into());
        }
    }
    // 2) blocks use 'inst' instead of 'instructions'
    if let Some(funcs) = v.get("functions") {
        if let Some(arr) = funcs.as_array() {
            for f in arr {
                if let Some(blocks) = f.get("blocks").and_then(|b| b.as_array()) {
                    for b in blocks {
                        if b.get("inst").is_some() && b.get("instructions").is_none() {
                            return Some("block key 'inst' found; rename to 'instructions' or enable HAKO_LLVM_CANARY_NORMALIZE=1".into());
                        }
                    }
                }
            }
        }
    }
    None
}

/// Normalize a very small canary JSON into the shape expected by the Python harness.
/// - Accepts schema_version as number or string; coerces to "1.0" when 1.
/// - Renames block key 'inst' -> 'instructions'.
/// - Converts const {"ty":"i64","value":N} into {"value":{"type":"i64","value":N}}
fn normalize_canary_json(mut v: serde_json::Value) -> serde_json::Value {
    use serde_json::{Map, Value};
    // schema_version: number 1 -> string "1.0"
    match v.get_mut("schema_version") {
        Some(Value::Number(n)) if n.as_i64() == Some(1) => {
            *v.get_mut("schema_version").unwrap() = Value::String("1.0".to_string());
        }
        Some(Value::String(s)) if s == "1" => {
            *v.get_mut("schema_version").unwrap() = Value::String("1.0".to_string());
        }
        _ => {}
    }
    // functions as array
    if let Some(funcs) = v.get_mut("functions") {
        if let Value::Array(ref mut arr) = funcs {
            for func in arr.iter_mut() {
                if let Value::Object(ref mut fm) = func {
                    if let Some(blocks_v) = fm.get_mut("blocks") {
                        if let Value::Array(ref mut blks) = blocks_v {
                            for blk in blks.iter_mut() {
                                if let Value::Object(ref mut bm) = blk {
                                    // Rename 'inst' -> 'instructions'
                                    if let Some(insts) = bm.remove("inst") {
                                        bm.insert("instructions".to_string(), insts);
                                    }
                                    // Normalize instructions
                                    if let Some(Value::Array(ref mut ins_arr)) =
                                        bm.get_mut("instructions")
                                    {
                                        for ins in ins_arr.iter_mut() {
                                            if let Value::Object(ref mut im) = ins {
                                                if im.get("op").and_then(|x| x.as_str())
                                                    == Some("const")
                                                {
                                                    // if 'ty' and flat 'value' exist, wrap into typed value
                                                    if let (Some(ty), Some(val)) =
                                                        (im.remove("ty"), im.remove("value"))
                                                    {
                                                        let mut val_obj = Map::new();
                                                        if let Value::String(ts) = ty {
                                                            val_obj.insert(
                                                                "type".to_string(),
                                                                Value::String(ts),
                                                            );
                                                        } else {
                                                            val_obj.insert("type".to_string(), ty);
                                                        }
                                                        val_obj.insert("value".to_string(), val);
                                                        im.insert(
                                                            "value".to_string(),
                                                            Value::Object(val_obj),
                                                        );
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    v
}

fn run_harness_dummy(harness: &Path, out: &Path) -> Result<()> {
    ensure_python()?;
    let mut cmd = Command::new("python3");
    cmd.arg(harness).arg("--out").arg(out);
    propagate_opt_level(&mut cmd);
    let status = cmd
        .status()
        .context("failed to execute python harness (dummy)")?;
    if !status.success() {
        bail!("harness exited with status: {:?}", status.code());
    }
    Ok(())
}

fn run_harness_in(harness: &Path, input: &Path, out: &Path) -> Result<()> {
    ensure_python()?;
    let mut cmd = Command::new("python3");
    cmd.arg(harness)
        .arg("--in")
        .arg(input)
        .arg("--out")
        .arg(out);
    propagate_opt_level(&mut cmd);
    let status = cmd.status().context("failed to execute python harness")?;
    if !status.success() {
        bail!("harness exited with status: {:?}", status.code());
    }
    Ok(())
}

fn emit_dummy_object_via_driver(driver: DriverKind, harness: &Path, out: &Path) -> Result<()> {
    match driver {
        DriverKind::Harness => run_harness_dummy(harness, out),
        DriverKind::Native => run_native_dummy(out),
    }
}

fn emit_object_via_driver(
    driver: DriverKind,
    harness: &Path,
    input: &Path,
    out: &Path,
) -> Result<()> {
    match driver {
        DriverKind::Harness => run_harness_in(harness, input, out),
        DriverKind::Native => run_native_in(input, out),
    }
}

fn run_native_dummy(_out: &Path) -> Result<()> {
    native_driver::emit_dummy_object(_out)
}

fn run_native_in(_input: &Path, _out: &Path) -> Result<()> {
    native_driver::emit_object_from_json(_input, _out)
}

fn ensure_python() -> Result<()> {
    match Command::new("python3").arg("--version").output() {
        Ok(out) if out.status.success() => Ok(()),
        _ => bail!("python3 not found in PATH (required for llvmlite harness)"),
    }
}

fn propagate_opt_level(cmd: &mut Command) {
    let hako = env::var("HAKO_LLVM_OPT_LEVEL").ok();
    let nyash = env::var("NYASH_LLVM_OPT_LEVEL").ok();
    let level = nyash.clone().or(hako.clone());
    if let Some(level) = level {
        if hako.is_some() && nyash.is_none() {
            eprintln!(
                "[deprecate/env] 'HAKO_LLVM_OPT_LEVEL' is deprecated; use 'NYASH_LLVM_OPT_LEVEL'"
            );
        }
        cmd.env("HAKO_LLVM_OPT_LEVEL", &level);
        cmd.env("NYASH_LLVM_OPT_LEVEL", &level);
    }
}

fn link_executable(
    obj: &Path,
    out_exe: &Path,
    nyrt_dir_opt: Option<&PathBuf>,
    extra_libs: Option<&str>,
) -> Result<()> {
    // Resolve nyRT static lib
    let nyrt_dir = if let Some(dir) = nyrt_dir_opt {
        dir.clone()
    } else {
        // try target/release then crates/nyash_kernel/target/release
        let a = PathBuf::from("target/release");
        let b = PathBuf::from("crates/nyash_kernel/target/release");
        if a.join("libnyash_kernel.a").exists() {
            a
        } else {
            b
        }
    };
    let libnyrt = nyrt_dir.join("libnyash_kernel.a");
    if !libnyrt.exists() {
        bail!(
            "libnyash_kernel.a not found in {}.\n\
             hint: build the kernel staticlib first:\n\
               cargo build --release -p nyash_kernel\n\
             expected output (workspace default): target/release/libnyash_kernel.a\n\
             or pass an explicit directory via --nyrt <DIR>.\n\
             note: the llvmlite harness path (NYASH_LLVM_USE_HARNESS=1) does not need libnyash_kernel.a.",
            nyrt_dir.display(),
        );
    }

    // Choose a C linker
    let linker = ["cc", "clang", "gcc"]
        .into_iter()
        .find(|c| {
            Command::new(c)
                .arg("--version")
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false)
        })
        .unwrap_or("cc");

    let mut cmd = Command::new(linker);
    cmd.arg("-o").arg(out_exe);
    cmd.arg(obj);
    // Perf fast path (Linux): disable PIE to avoid TEXTREL/relocation overhead
    // on micro-bench AOT executables. Keep default behavior unchanged unless
    // NYASH_LLVM_FAST=1.
    let use_no_pie =
        cfg!(target_os = "linux") && env::var("NYASH_LLVM_FAST").ok().as_deref() == Some("1");
    if use_no_pie {
        cmd.arg("-no-pie");
    }
    // Whole-archive libnyash_kernel to ensure all objects are linked
    cmd.arg("-Wl,--whole-archive")
        .arg(&libnyrt)
        .arg("-Wl,--no-whole-archive");
    // Common libs on Linux
    cmd.arg("-ldl").arg("-lpthread").arg("-lm");
    if let Some(extras) = extra_libs {
        for tok in extras.split_whitespace() {
            cmd.arg(tok);
        }
    }
    // Run linker and capture diagnostics for better error reporting
    let output = cmd
        .output()
        .with_context(|| format!("failed to invoke system linker: {}", linker))?;
    if !output.status.success() {
        eprintln!("[ny-llvmc/link] command: {}", linker);
        // Show args (for debugging)
        // Note: std::process::Command doesn't expose argv back; re-emit essential parts
        eprintln!(
            "[ny-llvmc/link] args: -o {} {} {} -Wl,--whole-archive {} -Wl,--no-whole-archive -ldl -lpthread -lm {}",
            out_exe.display(),
            obj.display(),
            if use_no_pie { "-no-pie" } else { "" },
            libnyrt.display(),
            extra_libs.unwrap_or("")
        );
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        eprintln!("[ny-llvmc/link:stdout]\n{}", stdout);
        eprintln!("[ny-llvmc/link:stderr]\n{}", stderr);
        bail!("linker exited with status: {:?}", output.status.code());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;
    use clap::error::ErrorKind;

    #[test]
    fn cli_contract_defaults_to_obj_from_stdin() {
        let args = Args::try_parse_from(["ny-llvmc", "--out", "/tmp/out.o"]).unwrap();
        assert_eq!(args.infile, "-");
        assert_eq!(args.emit, EmitKind::Obj);
        assert_eq!(args.driver, DriverKind::Harness);
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
        let args = Args::try_parse_from([
            "ny-llvmc",
            "--out",
            "out.o",
            "--driver",
            "native",
        ])
        .unwrap();
        assert_eq!(args.driver, DriverKind::Native);
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

    #[test]
    fn resolve_harness_path_defaults_to_workspace_wrapper() {
        assert_eq!(
            resolve_harness_path(None),
            PathBuf::from("tools/llvmlite_harness.py")
        );
    }

    #[test]
    fn resolve_object_output_path_uses_o_suffix_for_exe() {
        assert_eq!(
            resolve_object_output_path(Path::new("/tmp/out.exe"), true),
            PathBuf::from("/tmp/out.o")
        );
        assert_eq!(
            resolve_object_output_path(Path::new("/tmp/out.o"), false),
            PathBuf::from("/tmp/out.o")
        );
    }
}
