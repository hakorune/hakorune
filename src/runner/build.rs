use super::NyashRunner;
use std::path::{Path, PathBuf};

#[path = "build_shared.rs"]
mod build_shared;
use build_shared::{
    apply_env_overrides, build_core, build_plugins, load_build_doc, resolve_app_entry,
};

pub(super) fn run_build_mvp_impl(runner: &NyashRunner, cfg_path: &str) -> Result<(), String> {
    let cwd = std::env::current_dir().unwrap_or(PathBuf::from("."));
    let doc = load_build_doc(&cwd, cfg_path)?;
    apply_env_overrides(&doc);

    let profile = runner
        .config
        .build_profile
        .clone()
        .unwrap_or_else(|| "release".into());
    let aot = runner
        .config
        .build_aot
        .clone()
        .unwrap_or_else(|| "cranelift".into());
    let out = runner.config.build_out.clone();
    let target = runner.config.build_target.clone();
    build_plugins(&cwd, &doc, &profile, target.as_deref())?;
    build_core(&profile, &aot, target.as_deref())?;

    let app = resolve_app_entry(&cwd, &doc, runner.config.build_app.clone())?;

    let obj_dir = cwd.join("target").join("aot_objects");
    let _ = std::fs::create_dir_all(&obj_dir);
    let obj_path = obj_dir.join("main.o");
    emit_object(&cwd, &profile, &aot, &app, &obj_dir, &obj_path)?;
    ensure_object_exists(&obj_dir, &obj_path)?;

    let out_path = if let Some(o) = out {
        PathBuf::from(o)
    } else {
        if cfg!(windows) {
            cwd.join("app.exe")
        } else {
            cwd.join("app")
        }
    };
    // 7) Link
    println!("[link] → {}", out_path.display());
    #[cfg(windows)]
    {
        // Prefer MSVC link.exe, then clang fallback
        if let Ok(link) = which::which("link") {
            let status = std::process::Command::new(&link)
                .args([
                    "/NOLOGO",
                    &format!("/OUT:{}", out_path.display().to_string()),
                ])
                .arg(&obj_path)
                .arg(cwd.join("target").join("release").join("nyrt.lib"))
                .status()
                .map_err(|e| format!("spawn link.exe: {}", e))?;
            if status.success() {
                println!("OK");
                return Ok(());
            }
        }
        if let Ok(clang) = which::which("clang") {
            let status = std::process::Command::new(&clang)
                .args([
                    "-o",
                    &out_path.display().to_string(),
                    &obj_path.display().to_string(),
                ])
                .arg(
                    cwd.join("target")
                        .join("release")
                        .join("nyrt.lib")
                        .display()
                        .to_string(),
                )
                .arg("-lntdll")
                .status()
                .map_err(|e| format!("spawn clang: {}", e))?;
            if status.success() {
                println!("OK");
                return Ok(());
            }
            return Err("link failed on Windows (tried link.exe and clang)".into());
        }
        return Err("no linker found (need Visual Studio link.exe or LLVM clang)".into());
    }
    #[cfg(not(windows))]
    {
        let status = std::process::Command::new("cc")
            .arg(&obj_path)
            .args([
                "-L",
                &cwd.join("target").join("release").display().to_string(),
            ])
            .args([
                "-Wl,--whole-archive",
                "-lnyrt",
                "-Wl,--no-whole-archive",
                "-lpthread",
                "-ldl",
                "-lm",
            ])
            .args(["-o", &out_path.display().to_string()])
            .status()
            .map_err(|e| format!("spawn cc: {}", e))?;
        if !status.success() {
            return Err("link failed (cc)".into());
        }
    }
    println!("✅ Success: {}", out_path.display());
    Ok(())
}

fn emit_object(
    cwd: &Path,
    profile: &str,
    aot: &str,
    app: &str,
    obj_dir: &Path,
    obj_path: &Path,
) -> Result<(), String> {
    if aot == "llvm" {
        emit_llvm_object(cwd, profile, app, obj_path)
    } else {
        emit_engineering_object(cwd, profile, app, obj_dir)
    }
}

fn emit_llvm_object(cwd: &Path, profile: &str, app: &str, obj_path: &Path) -> Result<(), String> {
    std::env::set_var("NYASH_LLVM_OBJ_OUT", obj_path);
    println!("[emit] LLVM object → {}", obj_path.display());
    let status = std::process::Command::new(nyash_bin_path(cwd, profile))
        .args(["--backend", "llvm", app])
        .status()
        .map_err(|e| format!("spawn nyash llvm: {}", e))?;
    if !status.success() {
        return Err("LLVM emit failed".into());
    }
    Ok(())
}

fn emit_engineering_object(
    cwd: &Path,
    profile: &str,
    app: &str,
    obj_dir: &Path,
) -> Result<(), String> {
    std::env::set_var("NYASH_AOT_OBJECT_OUT", obj_dir);
    println!("[emit] Cranelift object → {} (directory)", obj_dir.display());
    let status = std::process::Command::new(nyash_bin_path(cwd, profile))
        .args(["--backend", "vm", app])
        .status()
        .map_err(|e| format!("spawn nyash jit-aot: {}", e))?;
    if !status.success() {
        return Err("Cranelift emit failed".into());
    }
    Ok(())
}

fn nyash_bin_path(cwd: &Path, profile: &str) -> PathBuf {
    cwd.join("target")
        .join(profile)
        .join(if cfg!(windows) { "nyash.exe" } else { "nyash" })
}

fn ensure_object_exists(obj_dir: &Path, obj_path: &Path) -> Result<(), String> {
    if !obj_path.exists() && !obj_dir.join("main.o").exists() {
        return Err(format!("object not generated under {}", obj_dir.display()));
    }
    Ok(())
}
