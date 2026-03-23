use crate::runner::NyashRunner;

/// Collect using targets and strip using lines (no inlining).
/// Returns (cleaned_source, prelude_paths, imports) where:
/// - `prelude_paths` are resolved file paths to be parsed separately and AST-merged (when `NYASH_USING_AST=1`)
/// - `imports` is a HashMap mapping alias names to box types (for MirBuilder resolution)
///
/// Notes
/// - This function enforces profile policies (prod: disallow file-using; only
///   packages/aliases from nyash.toml are accepted).
/// - SSOT: Resolution sources and aliases come exclusively from nyash.toml.
/// - All runner modes use this static path to avoid logic drift.
pub fn collect_using_and_strip(
    runner: &NyashRunner,
    code: &str,
    filename: &str,
) -> Result<
    (
        String,
        Vec<String>,
        std::collections::HashMap<String, String>,
    ),
    String,
> {
    if !crate::config::env::enable_using() {
        return Ok((
            code.to_string(),
            Vec::new(),
            std::collections::HashMap::new(),
        ));
    }
    let plan = plan_using_strip(runner, code, filename)?;
    Ok(apply_using_strip_plan(plan))
}

struct UsingStripPlan {
    kept_lines: Vec<String>,
    kept_len: usize,
    prelude_paths: Vec<String>,
    imports: std::collections::HashMap<String, String>,
}

fn apply_using_strip_plan(
    plan: UsingStripPlan,
) -> (
    String,
    Vec<String>,
    std::collections::HashMap<String, String>,
) {
    let mut out = String::with_capacity(plan.kept_len + 64);
    for line in plan.kept_lines {
        out.push_str(&line);
        out.push('\n');
    }
    // Optional prelude boundary comment (helps manual inspection; parser ignores comments)
    if crate::config::env::resolve_seam_debug() {
        let mut with_marker = String::with_capacity(out.len() + 64);
        with_marker.push_str("\n/* --- using boundary (AST) --- */\n");
        with_marker.push_str(&out);
        out = with_marker;
    }
    (out, plan.prelude_paths, plan.imports)
}

fn plan_using_strip(
    runner: &NyashRunner,
    code: &str,
    filename: &str,
) -> Result<UsingStripPlan, String> {
    let using_ctx = runner.init_using_context();
    let prod = crate::config::env::using_is_prod();
    let strict = crate::config::env::env_bool("NYASH_USING_STRICT");
    let verbose = crate::config::env::cli_verbose() || crate::config::env::resolve_trace();
    let ctx_dir = std::path::Path::new(filename).parent();

    let mut kept_lines: Vec<String> = Vec::new();
    let mut kept_len: usize = 0;
    let mut prelude_paths: Vec<String> = Vec::new();
    // Duplicate-using detection (same target imported multiple times or alias rebound): error in all profiles
    use std::collections::HashMap;
    let mut seen_paths: HashMap<String, (String, usize)> = HashMap::new(); // canon_path -> (alias/label, first_line)
    let mut seen_aliases: HashMap<String, (String, usize)> = HashMap::new(); // alias -> (canon_path, first_line)
    let mut imports: HashMap<String, String> = HashMap::new(); // alias -> exported static box name
                                                                             // Determine if this file is inside a declared package root; if so, allow
                                                                             // internal file-using within the package even when file-using is globally disallowed.
    let filename_canon = std::fs::canonicalize(filename).ok();
    let mut inside_pkg = false;
    if let Some(ref fc) = filename_canon {
        for (_name, pkg) in &using_ctx.packages {
            let base = std::path::Path::new(&pkg.path);
            if let Ok(root) = std::fs::canonicalize(base) {
                if fc.starts_with(&root) {
                    inside_pkg = true;
                    break;
                }
            }
        }
    }
    for (lineno0, line) in code.lines().enumerate() {
        let line_no = lineno0 + 1;
        let t = line.trim_start();
        if t.starts_with("using ") {
            crate::cli_v!("[using] stripped line: {}", line);
            let rest0 = t.strip_prefix("using ").unwrap().trim();
            let rest0 = rest0.split('#').next().unwrap_or(rest0).trim();
            let rest0 = rest0.strip_suffix(';').unwrap_or(rest0).trim();
            let (target, alias_name) = if let Some(pos) = rest0.find(" as ") {
                (
                    rest0[..pos].trim().to_string(),
                    Some(rest0[pos + 4..].trim().to_string()),
                )
            } else {
                (rest0.to_string(), None)
            };
            // Strip quotes from target for alias/module lookup and path detection
            let target_unquoted = target.trim_matches('"').to_string();

            // Check if this is a known alias or module FIRST before treating as file path
            let is_known_alias_or_module = using_ctx.aliases.contains_key(&target_unquoted)
                || using_ctx
                    .pending_modules
                    .iter()
                    .any(|(k, _)| k == &target_unquoted)
                || using_ctx.packages.contains_key(&target_unquoted);

            let is_path = if is_known_alias_or_module {
                // Known alias/module - don't treat as file path even if quoted
                false
            } else {
                // SSOT: delegate path pattern check
                crate::runner::modes::common_util::resolve::path_util::is_using_target_path_unquoted(
                    &target_unquoted,
                )
            };
            if is_path {
                // SSOT: Disallow file-using at top-level; allow only for sources located
                // under a declared package root (internal package wiring), so that packages
                // can organize their modules via file paths.
                if (prod || !crate::config::env::allow_using_file()) && !inside_pkg {
                    return Err(format!(
                        "{}:{}: using: file paths are disallowed in this profile. Add it to nyash.toml [using]/[modules] and reference by name: {}\n  suggestions: using \"alias.name\" as Name  |  dev/test: set NYASH_PREINCLUDE=1 to expand includes ahead of VM\n  docs: see docs/reference/using.md",
                        filename,
                        line_no,
                        target
                    ));
                }
                let path = target_unquoted.clone();
                // Resolve relative to current file dir
                let mut p = std::path::PathBuf::from(&path);
                if p.is_relative() {
                    if let Some(dir) = ctx_dir {
                        let cand = dir.join(&p);
                        if cand.exists() {
                            p = cand;
                        }
                    }
                    // Also try repo root when available (repo-root relative like "apps/...")
                    if p.is_relative() {
                        if let Some(root) =
                            crate::runner::modes::common_util::resolve::root::resolve_repo_root(
                                Some(filename),
                            )
                        {
                            let cand = root.join(&p);
                            if cand.exists() {
                                p = cand;
                            }
                        } else {
                            // Fallback: guess project root from executable path (target/release/nyash)
                            if let Ok(exe) = std::env::current_exe() {
                                if let Some(root) = exe
                                    .parent()
                                    .and_then(|p| p.parent())
                                    .and_then(|p| p.parent())
                                {
                                    let cand = root.join(&p);
                                    if cand.exists() {
                                        p = cand;
                                    }
                                }
                            }
                        }
                    }
                }
                if verbose {
                    crate::runner::trace::log(format!(
                        "[using/resolve] file '{}' -> '{}'",
                        target,
                        p.display()
                    ));
                }
                let path_str = p.to_string_lossy().to_string();
                // Duplicate detection
                let canon = std::fs::canonicalize(&path_str)
                    .ok()
                    .map(|pb| pb.to_string_lossy().to_string())
                    .unwrap_or_else(|| path_str.clone());
                if let Some((prev_alias, prev_line)) = seen_paths.get(&canon) {
                    return Err(format!(
                        "using: duplicate import of '{}' at {}:{} (previous alias: '{}' first seen at line {})",
                        canon,
                        filename,
                        line_no,
                        prev_alias,
                        prev_line
                    ));
                } else {
                    seen_paths.insert(
                        canon.clone(),
                        (
                            alias_name.clone().unwrap_or_else(|| "<none>".into()),
                            line_no,
                        ),
                    );
                }
                if let Some(alias) = alias_name.clone() {
                    if let Some((prev_path, prev_line)) = seen_aliases.get(&alias) {
                        if prev_path != &canon {
                            return Err(format!(
                                "using: alias '{}' rebound at {}:{} (was '{}' first seen at line {})",
                                alias,
                                filename,
                                line_no,
                                prev_path,
                                prev_line
                            ));
                        }
                    } else {
                        seen_aliases.insert(alias, (canon, line_no));
                    }
                }
                prelude_paths.push(path_str);
                remember_import_binding(
                    &mut imports,
                    alias_name.as_deref(),
                    &target_unquoted,
                    prelude_paths.last().expect("path just pushed"),
                )?;
                continue;
            }
            // Resolve namespaces/packages
            if prod {
                // prod: allow only names present in aliases/modules/packages (nyash.toml)
                let mut name: String = target_unquoted.clone();
                if let Some(v) = using_ctx.aliases.get(&target_unquoted) {
                    name = v.clone();
                }

                // SSOT: try central resolver first (modules/packages/relative)
                if let Ok(resolved) = crate::using::resolver::resolve_using_target_common(
                    &name,
                    &using_ctx.pending_modules,
                    &using_ctx.module_roots,
                    &using_ctx.using_paths,
                    &using_ctx.packages,
                    ctx_dir,
                    strict,
                    verbose,
                ) {
                    if resolved.starts_with("dylib:") {
                        continue;
                    }
                    let canon = std::fs::canonicalize(&resolved)
                        .ok()
                        .map(|pb| pb.to_string_lossy().to_string())
                        .unwrap_or_else(|| resolved.clone());
                    if let Some((prev_alias, prev_line)) = seen_paths.get(&canon) {
                        return Err(format!(
                            "using: duplicate import of '{}' at {}:{} (previous alias: '{}' first seen at line {})",
                            canon, filename, line_no, prev_alias, prev_line
                        ));
                    } else {
                        seen_paths.insert(
                            canon.clone(),
                            (
                                alias_name.clone().unwrap_or_else(|| "<none>".into()),
                                line_no,
                            ),
                        );
                    }
                    if let Some(alias) = alias_name.clone() {
                        if let Some((prev_path, prev_line)) = seen_aliases.get(&alias) {
                            if prev_path != &canon {
                                return Err(format!(
                                    "using: alias '{}' rebound at {}:{} (was '{}' first seen at line {})",
                                    alias, filename, line_no, prev_path, prev_line
                                ));
                            }
                        } else {
                            seen_aliases.insert(alias, (canon, line_no));
                        }
                    }
                    prelude_paths.push(resolved);
                    remember_import_binding(
                        &mut imports,
                        alias_name.as_deref(),
                        &target_unquoted,
                        prelude_paths.last().expect("path just pushed"),
                    )?;
                    continue;
                }

                // 1) modules mapping (name -> path)
                if let Some((_, mod_path)) =
                    using_ctx.pending_modules.iter().find(|(n, _)| n == &name)
                {
                    let out_path = mod_path.clone();
                    // Duplicate detection (same semantics as packages below)
                    let canon = std::fs::canonicalize(&out_path)
                        .ok()
                        .map(|pb| pb.to_string_lossy().to_string())
                        .unwrap_or_else(|| out_path.clone());
                    if let Some((prev_alias, prev_line)) = seen_paths.get(&canon) {
                        return Err(format!(
                            "using: duplicate import of '{}' at {}:{} (previous alias: '{}' first seen at line {})",
                            canon, filename, line_no, prev_alias, prev_line
                        ));
                    } else {
                        seen_paths.insert(
                            canon.clone(),
                            (
                                alias_name.clone().unwrap_or_else(|| "<none>".into()),
                                line_no,
                            ),
                        );
                    }
                    if let Some(alias) = alias_name.clone() {
                        if let Some((prev_path, prev_line)) = seen_aliases.get(&alias) {
                            if prev_path != &canon {
                                return Err(format!(
                                    "using: alias '{}' rebound at {}:{} (was '{}' first seen at line {})",
                                    alias, filename, line_no, prev_path, prev_line
                                ));
                            }
                        } else {
                            seen_aliases.insert(alias, (canon, line_no));
                        }
                    }
                    prelude_paths.push(out_path);
                    remember_import_binding(
                        &mut imports,
                        alias_name.as_deref(),
                        &target_unquoted,
                        prelude_paths.last().expect("path just pushed"),
                    )?;
                }
                // 2) named packages
                else if let Some(pkg) = using_ctx.packages.get(&name) {
                    use crate::using::spec::PackageKind;
                    match pkg.kind {
                        PackageKind::Dylib => {
                            // dylib: nothing to prelude-parse; runtime loader handles it.
                        }
                        PackageKind::Package => {
                            let base = std::path::Path::new(&pkg.path);
                            let out = if let Some(m) = &pkg.main {
                                if matches!(
                                    base.extension().and_then(|s| s.to_str()),
                                    Some("nyash") | Some("hako")
                                ) {
                                    pkg.path.clone()
                                } else {
                                    base.join(m).to_string_lossy().to_string()
                                }
                            } else if matches!(
                                base.extension().and_then(|s| s.to_str()),
                                Some("nyash") | Some("hako")
                            ) {
                                pkg.path.clone()
                            } else {
                                let leaf =
                                    base.file_name().and_then(|s| s.to_str()).unwrap_or(&name);
                                base.join(format!("{}.hako", leaf))
                                    .to_string_lossy()
                                    .to_string()
                            };
                            // Duplicate detection for prod package alias resolution
                            let canon = std::fs::canonicalize(&out)
                                .ok()
                                .map(|pb| pb.to_string_lossy().to_string())
                                .unwrap_or_else(|| out.clone());
                            if let Some((prev_alias, prev_line)) = seen_paths.get(&canon) {
                                return Err(format!(
                                    "using: duplicate import of '{}' at {}:{} (previous alias: '{}' first seen at line {})",
                                    canon,
                                    filename,
                                    line_no,
                                    prev_alias,
                                    prev_line
                                ));
                            } else {
                                seen_paths.insert(
                                    canon.clone(),
                                    (
                                        alias_name.clone().unwrap_or_else(|| "<none>".into()),
                                        line_no,
                                    ),
                                );
                            }
                            if let Some(alias) = alias_name.clone() {
                                if let Some((prev_path, prev_line)) = seen_aliases.get(&alias) {
                                    if prev_path != &canon {
                                        return Err(format!(
                                            "using: alias '{}' rebound at {}:{} (was '{}' first seen at line {})",
                                            alias, filename, line_no, prev_path, prev_line
                                        ));
                                    }
                                } else {
                                    seen_aliases.insert(alias, (canon, line_no));
                                }
                            }
                            // push resolved file path for text-prelude merge
                            prelude_paths.push(out);
                            remember_import_binding(
                                &mut imports,
                                alias_name.as_deref(),
                                &target_unquoted,
                                prelude_paths.last().expect("path just pushed"),
                            )?;
                        }
                    }
                } else {
                    // ⚠️ Phase 0.3: User-friendly "Did you mean?" suggestions
                    let similar: Vec<_> = using_ctx
                        .aliases
                        .keys()
                        .filter(|k| {
                            k.to_lowercase().contains(&target_unquoted.to_lowercase())
                                || target_unquoted.to_lowercase().contains(&k.to_lowercase())
                        })
                        .take(3)
                        .collect();

                    let mut err_msg = format!(
                        "{}:{}: using: '{}' not found in nyash.toml [using]/[modules]",
                        filename, line_no, target_unquoted
                    );

                    if !similar.is_empty() {
                        err_msg.push_str("\n\n💡 Did you mean:");
                        for s in similar {
                            err_msg.push_str(&format!("\n   - {}", s));
                        }
                    }

                    if using_ctx.aliases.is_empty() {
                        err_msg
                            .push_str("\n\n⚠️  No aliases loaded (check TOML parse errors above)");
                    } else {
                        err_msg.push_str(&format!(
                            "\n\nAvailable modules: {} aliases",
                            using_ctx.aliases.len()
                        ));
                    }

                    err_msg.push_str("\n\n📝 Suggestions:");
                    err_msg.push_str("\n   - Add an alias in nyash.toml: [using.aliases] YourModule = \"path/to/module\"");
                    err_msg.push_str("\n   - Use the alias: using YourModule as YourModule");
                    err_msg.push_str("\n   - Dev/test mode: NYASH_PREINCLUDE=1");
                    err_msg.push_str("\n\n🔍 Debug: NYASH_DEBUG_USING=1 for detailed logs");

                    return Err(err_msg);
                }
            } else {
                // dev/ci: allow broader resolution via resolver
                match crate::runner::pipeline::resolve_using_target(
                    &target_unquoted,
                    false,
                    &using_ctx.pending_modules,
                    &using_ctx.module_roots,
                    &using_ctx.using_paths,
                    &using_ctx.aliases,
                    &using_ctx.packages,
                    ctx_dir,
                    strict,
                    verbose,
                ) {
                    Ok(value) => {
                        // Only file paths are candidates for AST prelude merge
                        if crate::runner::modes::common_util::resolve::path_util::is_using_target_path_unquoted(&value)
                            {
                            // Resolve relative
                            let mut p = std::path::PathBuf::from(&value);
                            if p.is_relative() {
                                if let Some(dir) = ctx_dir {
                                    let cand = dir.join(&p);
                                    if cand.exists() {
                                        p = cand;
                                    }
                                }
                                if p.is_relative() {
                                    if let Some(root) = crate::runner::modes::common_util::resolve::root::resolve_repo_root(Some(filename)) {
                                        let cand = root.join(&p);
                                        if cand.exists() {
                                            p = cand;
                                        }
                                    } else if let Ok(exe) = std::env::current_exe() {
                                        if let Some(root) = exe
                                            .parent()
                                            .and_then(|p| p.parent())
                                            .and_then(|p| p.parent())
                                        {
                                            let cand = root.join(&p);
                                            if cand.exists() {
                                                p = cand;
                                            }
                                        }
                                    }
                                }
                            }
                            if verbose {
                                crate::runner::trace::log(format!(
                                    "[using/resolve] dev-file '{}' -> '{}'",
                                    value,
                                    p.display()
                                ));
                            }
                            let path_str = p.to_string_lossy().to_string();
                            let canon = std::fs::canonicalize(&path_str)
                                .ok()
                                .map(|pb| pb.to_string_lossy().to_string())
                                .unwrap_or_else(|| path_str.clone());
                            if let Some((prev_alias, prev_line)) = seen_paths.get(&canon) {
                                return Err(format!(
                                    "using: duplicate import of '{}' at {}:{} (previous alias: '{}' first seen at line {})",
                                    canon,
                                    filename,
                                    line_no,
                                    prev_alias,
                                    prev_line
                                ));
                            } else {
                                seen_paths.insert(
                                    canon.clone(),
                                    (
                                        alias_name.clone().unwrap_or_else(|| "<none>".into()),
                                        line_no,
                                    ),
                                );
                            }
                            if let Some(alias) = alias_name.clone() {
                                if let Some((prev_path, prev_line)) = seen_aliases.get(&alias) {
                                    if prev_path != &canon {
                                        return Err(format!(
                                            "using: alias '{}' rebound at {}:{} (was '{}' first seen at line {})",
                                            alias, filename, line_no, prev_path, prev_line
                                        ));
                                    }
                                } else {
                                    seen_aliases.insert(alias, (canon, line_no));
                                }
                            }
                            prelude_paths.push(path_str);
                            remember_import_binding(
                                &mut imports,
                                alias_name.as_deref(),
                                &target_unquoted,
                                prelude_paths.last().expect("path just pushed"),
                            )?;
                        }
                    }
                    Err(e) => return Err(format!("{}:{}: using: {}", filename, line_no, e)),
                }
            }
            continue;
        }
        kept_len += line.len() + 1;
        kept_lines.push(line.to_string());
    }
    Ok(UsingStripPlan {
        kept_lines,
        kept_len,
        prelude_paths,
        imports,
    })
}

fn remember_import_binding(
    imports: &mut std::collections::HashMap<String, String>,
    alias_name: Option<&str>,
    target_unquoted: &str,
    resolved_path: &str,
) -> Result<(), String> {
    let Some(alias) = using_alias_key(alias_name, target_unquoted) else {
        return Ok(());
    };
    let Some(box_name) =
        crate::using::simple_registry::resolve_imported_static_box(resolved_path, &alias)
    else {
        return Ok(());
    };

    if let Some(prev) = imports.get(&alias) {
        if prev != &box_name {
            return Err(format!(
                "using: imported static box alias '{}' is ambiguous ({} vs {})",
                alias, prev, box_name
            ));
        }
        return Ok(());
    }

    imports.insert(alias, box_name);
    Ok(())
}

fn using_alias_key(alias_name: Option<&str>, target_unquoted: &str) -> Option<String> {
    let alias = alias_name.unwrap_or_else(|| default_using_alias(target_unquoted));
    let alias = alias.trim();
    if alias.is_empty() {
        None
    } else {
        Some(alias.to_string())
    }
}

fn default_using_alias(target: &str) -> &str {
    target
        .rsplit_once('.')
        .map(|(_, tail)| tail)
        .or_else(|| target.rsplit_once('/').map(|(_, tail)| tail))
        .unwrap_or(target)
}
