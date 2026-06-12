use super::*;

// ============ Phase 15.5: Safety Check Functions ============

/// Phase 15.5: ChatGPT recommended safety check with 4 core features
pub(crate) fn safety_check_v2(
    config_path: &PathBuf,
    library_filter: Option<&str>,
    box_type_filter: Option<&str>,
) {
    println!(
        "{}",
        "=== Plugin Safety Check v2 (ChatGPT Recommended Features) ===".bold()
    );
    println!("🛡️  Checking: Universal Slot Conflicts, E_METHOD Detection, TLV Response, StringBox Issues");

    // Load configuration
    let (config, raw_config) = match load_config(config_path) {
        Ok((cfg, raw)) => (cfg, raw),
        Err(e) => {
            eprintln!("{}: {}", "ERROR".red(), e);
            return;
        }
    };

    let config_base = config_path.parent().unwrap_or(Path::new("."));
    let mut total_issues = 0;
    let mut total_checks = 0;

    // Check each library
    for (lib_name, lib_def) in &config.libraries {
        if let Some(filter) = library_filter {
            if lib_name != filter {
                continue;
            }
        }

        println!("\n{}: {}", "Library".bold(), lib_name.cyan());

        // Check each box type
        for box_name in &lib_def.boxes {
            if let Some(filter) = box_type_filter {
                if box_name != filter {
                    continue;
                }
            }

            println!("\n  {}: {}", "Box Type".bold(), box_name.yellow());

            let box_config = match get_box_config(&raw_config, lib_name, box_name) {
                Some(cfg) => cfg,
                None => {
                    eprintln!("    {}: No configuration found", "WARNING".yellow());
                    continue;
                }
            };

            // Perform 4 safety checks
            let issues =
                perform_safety_checks(&box_config, lib_name, box_name, &lib_def, config_base);
            total_issues += issues;
            total_checks += 1;
        }
    }

    // Summary
    println!("\n{}", "=== Safety Check Summary ===".bold());
    println!("📊 Checked: {} box types", total_checks);
    if total_issues == 0 {
        println!("✅ {}: All safety checks passed!", "SUCCESS".green().bold());
    } else {
        println!(
            "🚨 {}: {} issues found",
            "ISSUES".red().bold(),
            total_issues
        );
        println!("   Please review and fix the issues above");
    }
}

/// Perform all 4 ChatGPT recommended safety checks
fn perform_safety_checks(
    box_config: &BoxTypeConfig,
    _lib_name: &str,
    box_name: &str,
    lib_def: &LibraryDefinition,
    config_base: &Path,
) -> u32 {
    let mut issues = 0;

    // 1. Universal Slot Conflicts Check
    issues += check_universal_slot_conflicts(&box_config.methods, box_name);

    // 2. StringBox Specific Issues Check
    if box_name == "StringBox" {
        issues += check_stringbox_issues(&box_config.methods);
    }

    // 3. E_METHOD Detection (requires plugin loading)
    issues += check_e_method_detection(box_config, lib_def, config_base);

    // 4. TLV Response Validation (requires plugin loading)
    issues += check_tlv_response_validation(box_config, lib_def, config_base);

    if issues == 0 {
        println!("    ✅ All safety checks passed");
    }

    issues
}

/// Check #1: Universal Slot Conflicts (0-3 reserved)
fn check_universal_slot_conflicts(
    methods: &HashMap<String, MethodDefinition>,
    _box_name: &str,
) -> u32 {
    let mut issues = 0;

    // Universal slot definitions (from MIR slot_registry.rs)
    let universal_slots = [(0, "toString"), (1, "type"), (2, "equals"), (3, "clone")];

    for (method_name, method_def) in methods {
        for (slot_id, universal_name) in &universal_slots {
            if method_def.method_id == *slot_id {
                if method_name != universal_name {
                    eprintln!(
                        "    🚨 {}: Method '{}' claims universal slot {} (reserved for '{}')",
                        "UNIVERSAL SLOT CONFLICT".red().bold(),
                        method_name,
                        slot_id,
                        universal_name
                    );
                    eprintln!(
                        "       Fix: Change method_id in nyash.toml to {} or higher",
                        4
                    );
                    issues += 1;
                } else {
                    println!(
                        "    ✅ Universal slot {}: {} correctly assigned",
                        slot_id, universal_name
                    );
                }
            }
        }
    }

    issues
}

/// Check #2: StringBox Specific Issues (the problem we discovered!)
fn check_stringbox_issues(methods: &HashMap<String, MethodDefinition>) -> u32 {
    let mut issues = 0;

    // Check for the exact issue we found: get=1, set=2 conflicts
    if let Some(get_method) = methods.get("get") {
        if get_method.method_id <= 3 {
            eprintln!(
                "    🚨 {}: StringBox.get() uses method_id {} (universal slot!)",
                "STRINGBOX ISSUE".red().bold(),
                get_method.method_id
            );
            eprintln!("       This is the exact bug we found! WebChatGPT worked because it used different IDs");
            eprintln!("       Fix: Change get method_id to 4 or higher");
            issues += 1;
        }
    }

    if let Some(set_method) = methods.get("set") {
        if set_method.method_id <= 3 {
            eprintln!(
                "    🚨 {}: StringBox.set() uses method_id {} (universal slot!)",
                "STRINGBOX ISSUE".red().bold(),
                set_method.method_id
            );
            eprintln!("       Fix: Change set method_id to 5 or higher");
            issues += 1;
        }
    }

    if issues == 0 {
        println!("    ✅ StringBox method IDs look good");
    }

    issues
}

/// Check #3: E_METHOD Detection (checks if plugin actually implements declared methods)
fn check_e_method_detection(
    box_config: &BoxTypeConfig,
    lib_def: &LibraryDefinition,
    config_base: &Path,
) -> u32 {
    let mut issues = 0;

    // Try to load the plugin
    let lib_path = resolve_plugin_path(config_base, &lib_def.path);
    let library = match unsafe { Library::new(&lib_path) } {
        Ok(lib) => lib,
        Err(e) => {
            eprintln!(
                "    🚨 {}: Cannot load plugin: {} (path: {})",
                "PLUGIN LOAD ERROR".red().bold(),
                e,
                lib_path.display()
            );
            return 1;
        }
    };

    // Get invoke function
    let invoke_fn: Symbol<
        unsafe extern "C" fn(u32, u32, u32, *const u8, usize, *mut u8, *mut usize) -> i32,
    > = match unsafe { library.get(b"nyash_plugin_invoke") } {
        Ok(f) => f,
        Err(_) => {
            eprintln!(
                "    🚨 {}: nyash_plugin_invoke not found",
                "E_METHOD CHECK FAILED".red().bold()
            );
            return 1;
        }
    };

    println!("    🔍 Testing method implementations...");

    // Test each declared method to see if it returns E_METHOD (-3)
    for (method_name, method_def) in &box_config.methods {
        // Skip constructor and destructor for this test
        if method_def.method_id == 0 || method_def.method_id == 4294967295 {
            continue;
        }

        unsafe {
            let args = tlv_encode_empty();
            let mut result_buf = vec![0u8; 64];
            let mut result_len = result_buf.len();

            // Test with a dummy instance (we're just checking if method exists)
            let result = invoke_fn(
                box_config.type_id,
                method_def.method_id,
                999999, // Dummy instance ID
                args.as_ptr(),
                args.len(),
                result_buf.as_mut_ptr(),
                &mut result_len,
            );

            match result {
                -3 => {
                    eprintln!(
                        "    🚨 {}: Method '{}' (id={}) returns E_METHOD - NOT IMPLEMENTED!",
                        "E_METHOD DETECTED".red().bold(),
                        method_name,
                        method_def.method_id
                    );
                    eprintln!("       This is exactly what caused StringBox.get() to fail!");
                    eprintln!(
                        "       Fix: Implement method '{}' in plugin or remove from nyash.toml",
                        method_name
                    );
                    issues += 1;
                }
                -8 => {
                    println!(
                        "    ✅ Method '{}' exists (invalid handle, but method found)",
                        method_name
                    );
                }
                0 => {
                    println!("    ✅ Method '{}' exists and works", method_name);
                }
                _ => {
                    println!(
                        "    ⚠️  Method '{}' returned code {} (exists but may have issues)",
                        method_name, result
                    );
                }
            }
        }
    }

    issues
}

/// Check #4: TLV Response Validation (checks if methods return proper TLV data)
fn check_tlv_response_validation(
    box_config: &BoxTypeConfig,
    lib_def: &LibraryDefinition,
    config_base: &Path,
) -> u32 {
    let mut issues = 0;

    // Try to load the plugin (reuse logic from E_METHOD check)
    let lib_path = resolve_plugin_path(config_base, &lib_def.path);
    let library = match unsafe { Library::new(&lib_path) } {
        Ok(lib) => lib,
        Err(_) => {
            eprintln!(
                "    🚨 {}: Cannot load plugin for TLV validation",
                "TLV CHECK FAILED".red().bold()
            );
            return 1;
        }
    };

    let invoke_fn: Symbol<
        unsafe extern "C" fn(u32, u32, u32, *const u8, usize, *mut u8, *mut usize) -> i32,
    > = match unsafe { library.get(b"nyash_plugin_invoke") } {
        Ok(f) => f,
        Err(_) => {
            eprintln!(
                "    🚨 {}: nyash_plugin_invoke not found for TLV validation",
                "TLV CHECK FAILED".red().bold()
            );
            return 1;
        }
    };

    println!("    🔍 Testing TLV response formats...");

    // Test birth method (constructor) for proper TLV Handle response
    if box_config.methods.contains_key("birth")
        || box_config.methods.values().any(|m| m.method_id == 0)
    {
        unsafe {
            let args = tlv_encode_empty();
            let mut result_buf = vec![0u8; 1024];
            let mut result_len = result_buf.len();

            let result = invoke_fn(
                box_config.type_id,
                0, // birth method
                0, // static call
                args.as_ptr(),
                args.len(),
                result_buf.as_mut_ptr(),
                &mut result_len,
            );

            if result == 0 {
                // Check if response is valid TLV with Handle (tag=8)
                if result_len >= 8 && result_buf[4] == 8 {
                    println!("    ✅ Constructor returns proper TLV Handle (tag=8)");
                } else {
                    eprintln!(
                        "    🚨 {}: Constructor returns invalid TLV format (expected Handle tag=8)",
                        "TLV FORMAT ERROR".red().bold()
                    );
                    eprintln!(
                        "       Got length={}, first_tag={}",
                        result_len,
                        if result_len > 4 { result_buf[4] } else { 0 }
                    );
                    issues += 1;
                }
            } else {
                eprintln!(
                    "    ⚠️  Constructor failed (code={}), cannot validate TLV",
                    result
                );
            }
        }
    }

    // Test string-returning methods (should return tag=6 String or tag=7 Bytes)
    let string_methods = ["toString", "type", "get", "toUtf8"];
    for method_name in &string_methods {
        if let Some(method_def) = box_config.methods.get(*method_name) {
            unsafe {
                let args = tlv_encode_empty();
                let mut result_buf = vec![0u8; 1024];
                let mut result_len = result_buf.len();

                let result = invoke_fn(
                    box_config.type_id,
                    method_def.method_id,
                    999999, // Dummy instance
                    args.as_ptr(),
                    args.len(),
                    result_buf.as_mut_ptr(),
                    &mut result_len,
                );

                if result == 0 && result_len >= 8 {
                    let tag = result_buf[4];
                    match tag {
                        6 => println!("    ✅ Method '{}' returns String TLV (tag=6)", method_name),
                        7 => println!("    ✅ Method '{}' returns Bytes TLV (tag=7)", method_name),
                        _ => {
                            eprintln!("    🚨 {}: Method '{}' returns unexpected TLV tag {} (expected 6 or 7)",
                                "TLV TYPE ERROR".red().bold(), method_name, tag);
                            issues += 1;
                        }
                    }
                } else if result == -8 {
                    println!(
                        "    ⚠️  Method '{}' needs valid instance for TLV test",
                        method_name
                    );
                }
            }
        }
    }

    issues
}
