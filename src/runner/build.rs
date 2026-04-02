use super::NyashRunner;
use std::path::{Path, PathBuf};

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

fn load_build_doc(cwd: &Path, cfg_path: &str) -> Result<toml::Value, String> {
    let cfg_abspath = if Path::new(cfg_path).is_absolute() {
        PathBuf::from(cfg_path)
    } else {
        cwd.join(cfg_path)
    };
    let text = std::fs::read_to_string(&cfg_abspath)
        .map_err(|e| format!("read {}: {}", cfg_abspath.display(), e))?;
    toml::from_str::<toml::Value>(&text)
        .map_err(|e| format!("parse {}: {}", cfg_abspath.display(), e))
}

fn apply_env_overrides(doc: &toml::Value) {
    if let Some(env_tbl) = doc.get("env").and_then(|v| v.as_table()) {
        for (k, v) in env_tbl {
            if let Some(s) = v.as_str() {
                std::env::set_var(k, s);
            }
        }
    }
}

fn build_plugins(
    cwd: &Path,
    doc: &toml::Value,
    profile: &str,
    target: Option<&str>,
) -> Result<(), String> {
    if let Some(pl_tbl) = doc.get("plugins").and_then(|v| v.as_table()) {
        for (name, v) in pl_tbl {
            if let Some(path) = v.as_str() {
                let plugin_dir = if Path::new(path).is_absolute() {
                    PathBuf::from(path)
                } else {
                    cwd.join(path)
                };
                let mut cmd = std::process::Command::new("cargo");
                cmd.arg("build");
                if profile == "release" {
                    cmd.arg("--release");
                }
                if let Some(triple) = target {
                    cmd.args(["--target", triple]);
                }
                cmd.current_dir(&plugin_dir);
                println!("[build] plugin {} at {}", name, plugin_dir.display());
                let status = cmd
                    .status()
                    .map_err(|e| format!("spawn cargo (plugin {}): {}", name, e))?;
                if !status.success() {
                    return Err(format!(
                        "plugin build failed: {} (dir={})",
                        name,
                        plugin_dir.display()
                    ));
                }
            }
        }
    }
    Ok(())
}

fn build_core(profile: &str, aot: &str, target: Option<&str>) -> Result<(), String> {
    let features = if aot == "llvm" {
        "llvm"
    } else {
        "cranelift-jit"
    };
    let mut cmd = std::process::Command::new("cargo");
    cmd.arg("build");
    if profile == "release" {
        cmd.arg("--release");
    }
    cmd.args(["--features", features]);
    if let Some(triple) = target {
        cmd.args(["--target", triple]);
    }
    println!("[build] nyash core ({}, features={})", profile, features);
    let status = cmd
        .status()
        .map_err(|e| format!("spawn cargo (core): {}", e))?;
    if !status.success() {
        return Err("nyash core build failed".into());
    }
    Ok(())
}

fn resolve_app_entry(
    cwd: &Path,
    doc: &toml::Value,
    explicit_app: Option<String>,
) -> Result<String, String> {
    if let Some(app) = explicit_app.filter(|s| !s.is_empty()) {
        return Ok(app);
    }
    if let Some(app) = doc
        .get("build")
        .and_then(|v| v.as_table())
        .and_then(|tbl| tbl.get("app"))
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
    {
        return Ok(app.to_string());
    }

    let mut candidates = Vec::new();
    collect_main_hako_candidates(&cwd.join("apps"), &mut candidates);
    let msg = if candidates.is_empty() {
        "no app specified (--app) and no apps/**/main.hako found".to_string()
    } else {
        format!(
            "no app specified (--app). Candidates:\n  - {}",
            candidates.join("\n  - ")
        )
    };
    Err(msg)
}

fn collect_main_hako_candidates(dir: &Path, acc: &mut Vec<String>) {
    if let Ok(rd) = std::fs::read_dir(dir) {
        for entry in rd.flatten() {
            let path = entry.path();
            if path.is_dir() {
                collect_main_hako_candidates(&path, acc);
            } else if path.file_name().map(|n| n == "main.hako").unwrap_or(false) {
                acc.push(path.display().to_string());
            }
        }
    }
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
