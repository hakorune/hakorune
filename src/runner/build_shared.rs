use std::path::{Path, PathBuf};

pub(super) fn load_build_doc(cwd: &Path, cfg_path: &str) -> Result<toml::Value, String> {
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

pub(super) fn apply_env_overrides(doc: &toml::Value) {
    if let Some(env_tbl) = doc.get("env").and_then(|v| v.as_table()) {
        for (k, v) in env_tbl {
            if let Some(s) = v.as_str() {
                std::env::set_var(k, s);
            }
        }
    }
}

pub(super) fn build_plugins(
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

pub(super) fn build_core(profile: &str, aot: &str, target: Option<&str>) -> Result<(), String> {
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

pub(super) fn resolve_app_entry(
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
