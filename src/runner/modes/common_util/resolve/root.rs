//! root — repository root resolution helpers (SSOT)
//!
//! `HAKO_ROOT` is allowed as an override for tools, but runtime semantics must
//! not depend on whether it is set. When we need to locate repo-relative assets
//! (e.g., operator preludes), prefer resolving a stable root via:
//! 1) `HAKO_ROOT` when set (`NYASH_ROOT` remains a compatibility alias)
//! 2) walking up from an on-disk hint file (e.g., the main source file)
//! 3) current working directory
//! 4) current executable path

use std::path::{Path, PathBuf};

pub fn resolve_repo_root(hint_file: Option<&str>) -> Option<PathBuf> {
    if let Some(root) = crate::config::env::hako_root() {
        let p = PathBuf::from(root);
        if p.exists() {
            return Some(p);
        }
    }

    if let Some(hint) = hint_file {
        if let Some(root) = walk_up_to_repo_root(Path::new(hint).parent()?) {
            return Some(root);
        }
    }

    if let Ok(cwd) = std::env::current_dir() {
        if let Some(root) = walk_up_to_repo_root(&cwd) {
            return Some(root);
        }
    }

    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            if let Some(root) = walk_up_to_repo_root(dir) {
                return Some(root);
            }
        }
    }

    None
}

fn walk_up_to_repo_root(start: &Path) -> Option<PathBuf> {
    let mut cur = start;
    for _ in 0..16 {
        if cur.join("Cargo.toml").exists() {
            return Some(cur.to_path_buf());
        }
        cur = cur.parent()?;
    }
    None
}
