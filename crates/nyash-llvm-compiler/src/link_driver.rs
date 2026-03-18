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
    let nyrt_dir = if let Some(dir) = nyrt_dir_opt {
        dir.clone()
    } else {
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
    cmd.arg("-Wl,--whole-archive")
        .arg(&libnyrt)
        .arg("-Wl,--no-whole-archive");
    cmd.arg("-ldl").arg("-lpthread").arg("-lm");
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
