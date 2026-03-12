//! Using Resolution Box - 綺麗綺麗なusing文解決専門家！📦
//!
//! 巨大な `collect_using_and_strip` 関数を箱に分解して、
//! 責務を明確にしてテストしやすくするにゃ！

use crate::runner::NyashRunner;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// 📦 UsingResolutionBox - using文解決の専門家！
///
/// using文の解析、パス解決、重複チェックを一手に引き受ける箱にゃ！
pub struct UsingResolutionBox<'a> {
    runner: &'a NyashRunner,
    config: UsingConfig,
    ctx_dir: Option<PathBuf>,
    #[allow(dead_code)]
    filename_canon: Option<PathBuf>,
    inside_pkg: bool,
    seen_paths: HashMap<String, (String, usize)>, // canon_path -> (alias/label, first_line)
    seen_aliases: HashMap<String, (String, usize)>, // alias -> (canon_path, first_line)
}

/// 🎯 UsingTarget - 解析済みusing文の構造体にゃ！
#[derive(Debug, Clone)]
pub struct UsingTarget {
    pub original: String,
    pub target: String,
    pub target_unquoted: String,
    pub alias: Option<String>,
    pub line_no: usize,
    pub is_path: bool,
}

/// ⚙️ UsingConfig - using解決の設定！
#[derive(Debug)]
pub struct UsingConfig {
    pub prod: bool,
    pub strict: bool,
    pub verbose: bool,
    pub allow_file_using: bool,
}

impl<'a> UsingResolutionBox<'a> {
    /// 🌟 新しいUsingResolutionBoxを作るにゃ！
    pub fn new(runner: &'a NyashRunner, filename: &str) -> Result<Self, String> {
        let using_ctx = runner.init_using_context();
        let config = UsingConfig {
            prod: crate::config::env::using_is_prod(),
            strict: crate::config::env::env_bool("NYASH_USING_STRICT"),
            verbose: crate::config::env::cli_verbose() || crate::config::env::resolve_trace(),
            allow_file_using: crate::config::env::allow_using_file(),
        };

        let ctx_dir = Path::new(filename).parent().map(|p| p.to_path_buf());

        // ファイルがパッケージ内にあるかチェック
        let filename_canon = std::fs::canonicalize(filename).ok();
        let mut inside_pkg = false;
        if let Some(ref fc) = filename_canon {
            for (_name, pkg) in &using_ctx.packages {
                let base = Path::new(&pkg.path);
                if let Ok(root) = std::fs::canonicalize(base) {
                    if fc.starts_with(&root) {
                        inside_pkg = true;
                        break;
                    }
                }
            }
        }

        Ok(Self {
            runner,
            config,
            ctx_dir,
            filename_canon,
            inside_pkg,
            seen_paths: HashMap::new(),
            seen_aliases: HashMap::new(),
        })
    }

    /// 🔍 using文を解析するにゃ！
    pub fn parse_using_line(&self, line: &str, line_no: usize) -> Option<UsingTarget> {
        let t = line.trim_start();
        if !t.starts_with("using ") {
            return None;
        }

        crate::cli_v!("[using] stripped line: {}", line);

        let rest0 = t.strip_prefix("using ").unwrap().trim();
        let rest0 = rest0.split('#').next().unwrap_or(rest0).trim();
        let rest0 = rest0.strip_suffix(';').unwrap_or(rest0).trim();

        let (target, alias) = if let Some(pos) = rest0.find(" as ") {
            (
                rest0[..pos].trim().to_string(),
                Some(rest0[pos + 4..].trim().to_string()),
            )
        } else {
            (rest0.to_string(), None)
        };

        let target_unquoted = target.trim_matches('"').to_string();
        let using_ctx = self.runner.init_using_context();

        // 既知のエイリアスかモジュールかチェック
        let is_known_alias_or_module = using_ctx.aliases.contains_key(&target_unquoted)
            || using_ctx
                .pending_modules
                .iter()
                .any(|(k, _)| k == &target_unquoted)
            || using_ctx.packages.contains_key(&target_unquoted);

        let is_path = if is_known_alias_or_module {
            false
        } else {
            crate::runner::modes::common_util::resolve::path_util::is_using_target_path_unquoted(
                &target_unquoted,
            )
        };

        Some(UsingTarget {
            original: line.to_string(),
            target,
            target_unquoted,
            alias,
            line_no,
            is_path,
        })
    }

    /// 🚀 パスを解決するにゃ！
    pub fn resolve_path(&self, target: &UsingTarget) -> Result<String, String> {
        if !target.is_path {
            return Err("Not a file path".to_string());
        }

        // ファイルusingチェック
        if (self.config.prod || !self.config.allow_file_using) && !self.inside_pkg {
            return Err(format!(
                "{}:{}: using: file paths are disallowed in this profile. Add it to nyash.toml [using]/[modules] and reference by name: {}\n  suggestions: using \"alias.name\" as Name  |  dev/test: set NYASH_PREINCLUDE=1 to expand includes ahead of VM\n  docs: see docs/reference/using.md",
                "filename", // TODO: 実際のファイル名を渡す
                target.line_no,
                target.target
            ));
        }

        let path = target.target.trim_matches('"').to_string();
        let mut p = PathBuf::from(&path);

        // 相対パス解決
        if p.is_relative() {
            if let Some(dir) = &self.ctx_dir {
                let cand = dir.join(&p);
                if cand.exists() {
                    p = cand;
                }
            }

            // NYASH_ROOTも試す
            if p.is_relative() {
                if let Some(root) =
                    crate::runner::modes::common_util::resolve::root::resolve_repo_root(None)
                {
                    let cand = root.join(&p);
                    if cand.exists() {
                        p = cand;
                    }
                }
            }
        }

        p.to_str()
            .ok_or_else(|| "Invalid path".to_string())
            .map(|s| s.to_string())
    }

    /// 🛡️ 重複チェックするにゃ！
    pub fn check_duplicates(
        &mut self,
        target: &UsingTarget,
        resolved_path: &str,
    ) -> Result<(), String> {
        let canon_path =
            std::fs::canonicalize(resolved_path).unwrap_or_else(|_| PathBuf::from(resolved_path));
        let canon_str = canon_path.to_string_lossy();

        // パスの重複チェック
        if let Some((prev_alias, prev_line)) = self.seen_paths.get(&canon_str.to_string()) {
            return Err(format!(
                "{}:{}: using: duplicate target (first imported at {}:{})",
                "filename", // TODO: 実際のファイル名を渡す
                target.line_no,
                prev_alias,
                prev_line
            ));
        }

        // エイリアスの重複チェック
        if let Some(ref alias_name) = target.alias {
            if let Some((prev_path, prev_line)) = self.seen_aliases.get(alias_name) {
                return Err(format!(
                    "{}:{}: using: duplicate alias '{}' (first used for {} at {})",
                    "filename", // TODO: 実際のファイル名を渡す
                    target.line_no,
                    alias_name,
                    prev_path,
                    prev_line
                ));
            }
        }

        // 記録
        let alias_label = target.alias.as_ref().unwrap_or(&target.target).clone();
        self.seen_paths
            .insert(canon_str.to_string(), (alias_label.clone(), target.line_no));

        if let Some(ref alias_name) = target.alias {
            self.seen_aliases.insert(
                alias_name.clone(),
                (resolved_path.to_string(), target.line_no),
            );
        }

        Ok(())
    }

    /// 📊 設定を取得するにゃ！
    pub fn config(&self) -> &UsingConfig {
        &self.config
    }
}
