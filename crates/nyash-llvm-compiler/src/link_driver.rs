use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{bail, Context, Result};

use super::boundary_driver;
use super::DriverKind;

pub(super) fn finalize_emit_output(
    driver: DriverKind,
    obj_path: &Path,
    out_path: &Path,
    emit_exe: bool,
    nyrt_dir: Option<&PathBuf>,
    extra_libs: Option<&str>,
    object_label: &str,
) -> Result<()> {
    if emit_exe {
        link_executable_via_driver(driver, obj_path, out_path, nyrt_dir, extra_libs)?;
        println!("[ny-llvmc] executable written: {}", out_path.display());
    } else {
        println!(
            "[ny-llvmc] {} written: {}",
            object_label,
            obj_path.display()
        );
    }
    Ok(())
}

fn link_executable_via_driver(
    driver: DriverKind,
    obj: &Path,
    out_exe: &Path,
    nyrt_dir_opt: Option<&PathBuf>,
    extra_libs: Option<&str>,
) -> Result<()> {
    match driver {
        DriverKind::Boundary => boundary_driver::link_object_to_exe(
            obj,
            out_exe,
            nyrt_dir_opt.map(|path| path.as_path()),
            extra_libs,
        ),
        DriverKind::Harness | DriverKind::Native => {
            link_executable(obj, out_exe, nyrt_dir_opt, extra_libs)
        }
    }
}

pub(super) fn link_executable(
    obj: &Path,
    out_exe: &Path,
    nyrt_dir_opt: Option<&PathBuf>,
    extra_libs: Option<&str>,
) -> Result<()> {
    let nyrt_dir = nyrt_dir_opt.cloned().context(
        "explicit --nyrt <DIR> is required for Harness/Native exe linking; boundary route handles fallback",
    )?;
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
    let whole_archive_enabled = link_whole_archive_enabled()?;
    let gc_sections_enabled = link_gc_sections_enabled()?;

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
    let use_no_pie =
        cfg!(target_os = "linux") && env::var("NYASH_LLVM_FAST").ok().as_deref() == Some("1");
    if use_no_pie {
        cmd.arg("-no-pie");
    }
    if whole_archive_enabled {
        cmd.arg("-Wl,--whole-archive")
            .arg(&libnyrt)
            .arg("-Wl,--no-whole-archive");
    } else {
        cmd.arg(&libnyrt);
    }
    if gc_sections_enabled {
        cmd.arg("-Wl,--gc-sections");
    }
    let system_libs = link_system_libs()?;
    cmd.arg("-ldl").arg("-lpthread");
    if system_libs.include_math {
        cmd.arg("-lm");
    }
    if let Some(extras) = extra_libs {
        for tok in extras.split_whitespace() {
            cmd.arg(tok);
        }
    }
    let output = cmd
        .output()
        .with_context(|| format!("failed to invoke system linker: {}", linker))?;
    if !output.status.success() {
        eprintln!("[ny-llvmc/link] command: {}", linker);
        let archive_mode = if whole_archive_enabled {
            format!("-Wl,--whole-archive {} -Wl,--no-whole-archive", libnyrt.display())
        } else {
            libnyrt.display().to_string()
        };
        let math_arg = if system_libs.include_math { "-lm" } else { "" };
        eprintln!(
            "[ny-llvmc/link] args: -o {} {} {} {} -ldl -lpthread {} {}",
            out_exe.display(),
            obj.display(),
            if use_no_pie { "-no-pie" } else { "" },
            archive_mode,
            math_arg,
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

fn parse_link_whole_archive_enabled(raw: Option<&str>) -> Result<bool> {
    let Some(value) = raw.map(str::trim).filter(|s| !s.is_empty()) else {
        return Ok(true);
    };
    match value.to_ascii_lowercase().as_str() {
        "1" | "true" | "on" => Ok(true),
        "0" | "false" | "off" => Ok(false),
        other => bail!(
            "[freeze:contract][ny-llvmc/link/whole-archive] expected=1|0|true|false|on|off got={}",
            other
        ),
    }
}

fn link_whole_archive_enabled() -> Result<bool> {
    parse_link_whole_archive_enabled(env::var("NYASH_LLVM_LINK_WHOLE_ARCHIVE").ok().as_deref())
}

fn parse_link_gc_sections_enabled(raw: Option<&str>) -> Result<bool> {
    let Some(value) = raw.map(str::trim).filter(|s| !s.is_empty()) else {
        return Ok(false);
    };
    match value.to_ascii_lowercase().as_str() {
        "1" | "true" | "on" => Ok(true),
        "0" | "false" | "off" => Ok(false),
        other => bail!(
            "[freeze:contract][ny-llvmc/link/gc-sections] expected=1|0|true|false|on|off got={}",
            other
        ),
    }
}

fn link_gc_sections_enabled() -> Result<bool> {
    parse_link_gc_sections_enabled(env::var("NYASH_LLVM_LINK_GC_SECTIONS").ok().as_deref())
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct LinkSystemLibs {
    include_math: bool,
}

fn parse_link_system_libs(raw: Option<&str>) -> Result<LinkSystemLibs> {
    let Some(value) = raw.map(str::trim).filter(|s| !s.is_empty()) else {
        return Ok(LinkSystemLibs { include_math: true });
    };
    match value.to_ascii_lowercase().as_str() {
        "full" | "default" | "legacy" => Ok(LinkSystemLibs { include_math: true }),
        "minimal" | "no-math" | "no_lm" => Ok(LinkSystemLibs {
            include_math: false,
        }),
        other => bail!(
            "[freeze:contract][ny-llvmc/link/system-libs] expected=full|default|legacy|minimal|no-math|no_lm got={}",
            other
        ),
    }
}

fn link_system_libs() -> Result<LinkSystemLibs> {
    parse_link_system_libs(env::var("NYASH_LLVM_LINK_SYSTEM_LIBS").ok().as_deref())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn harness_and_native_exe_linking_requires_explicit_nyrt_dir() {
        let err = link_executable(
            Path::new("/tmp/in.o"),
            Path::new("/tmp/out.exe"),
            None,
            None,
        )
        .unwrap_err();
        let message = err.to_string();
        assert!(
            message.contains("explicit --nyrt <DIR> is required for Harness/Native exe linking")
        );
    }

    #[test]
    fn link_whole_archive_defaults_to_enabled() {
        assert!(parse_link_whole_archive_enabled(None).unwrap());
        assert!(parse_link_whole_archive_enabled(Some("")).unwrap());
    }

    #[test]
    fn link_whole_archive_accepts_disable_aliases() {
        assert!(!parse_link_whole_archive_enabled(Some("0")).unwrap());
        assert!(!parse_link_whole_archive_enabled(Some("false")).unwrap());
        assert!(!parse_link_whole_archive_enabled(Some("off")).unwrap());
    }

    #[test]
    fn link_whole_archive_accepts_enable_aliases() {
        assert!(parse_link_whole_archive_enabled(Some("1")).unwrap());
        assert!(parse_link_whole_archive_enabled(Some("true")).unwrap());
        assert!(parse_link_whole_archive_enabled(Some("on")).unwrap());
    }

    #[test]
    fn link_whole_archive_rejects_invalid_values() {
        let err = parse_link_whole_archive_enabled(Some("maybe")).unwrap_err();
        assert!(
            err.to_string()
                .contains("[freeze:contract][ny-llvmc/link/whole-archive]")
        );
    }

    #[test]
    fn link_gc_sections_defaults_to_disabled() {
        assert!(!parse_link_gc_sections_enabled(None).unwrap());
        assert!(!parse_link_gc_sections_enabled(Some("")).unwrap());
    }

    #[test]
    fn link_gc_sections_accepts_disable_aliases() {
        assert!(!parse_link_gc_sections_enabled(Some("0")).unwrap());
        assert!(!parse_link_gc_sections_enabled(Some("false")).unwrap());
        assert!(!parse_link_gc_sections_enabled(Some("off")).unwrap());
    }

    #[test]
    fn link_gc_sections_accepts_enable_aliases() {
        assert!(parse_link_gc_sections_enabled(Some("1")).unwrap());
        assert!(parse_link_gc_sections_enabled(Some("true")).unwrap());
        assert!(parse_link_gc_sections_enabled(Some("on")).unwrap());
    }

    #[test]
    fn link_gc_sections_rejects_invalid_values() {
        let err = parse_link_gc_sections_enabled(Some("maybe")).unwrap_err();
        assert!(
            err.to_string()
                .contains("[freeze:contract][ny-llvmc/link/gc-sections]")
        );
    }

    #[test]
    fn link_system_libs_defaults_to_full() {
        assert!(parse_link_system_libs(None).unwrap().include_math);
        assert!(parse_link_system_libs(Some("")).unwrap().include_math);
    }

    #[test]
    fn link_system_libs_accepts_full_aliases() {
        assert!(parse_link_system_libs(Some("full")).unwrap().include_math);
        assert!(parse_link_system_libs(Some("default")).unwrap().include_math);
        assert!(parse_link_system_libs(Some("legacy")).unwrap().include_math);
    }

    #[test]
    fn link_system_libs_accepts_minimal_aliases() {
        assert!(!parse_link_system_libs(Some("minimal")).unwrap().include_math);
        assert!(!parse_link_system_libs(Some("no-math")).unwrap().include_math);
        assert!(!parse_link_system_libs(Some("no_lm")).unwrap().include_math);
    }

    #[test]
    fn link_system_libs_rejects_invalid_values() {
        let err = parse_link_system_libs(Some("maybe")).unwrap_err();
        assert!(
            err.to_string()
                .contains("[freeze:contract][ny-llvmc/link/system-libs]")
        );
    }
}
