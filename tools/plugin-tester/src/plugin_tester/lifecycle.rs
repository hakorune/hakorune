use super::*;

pub(crate) fn test_lifecycle_v2(config_path: &PathBuf, box_type: &str) {
    println!("{}", "=== Lifecycle Test v2 ===".bold());
    println!("Box type: {}", box_type.cyan());

    // Load nyash.toml
    let config_content = match fs::read_to_string(config_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("{}: Failed to read config: {}", "ERROR".red(), e);
            return;
        }
    };

    let config: NyashConfigV2 = match toml::from_str(&config_content) {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("{}: Failed to parse nyash.toml: {}", "ERROR".red(), e);
            return;
        }
    };

    let raw_config: toml::Value = toml::from_str(&config_content).unwrap();

    // Find library that provides this box type
    let (lib_name, lib_def) = match find_library_for_box(&config, box_type) {
        Some((name, def)) => (name, def),
        None => {
            eprintln!(
                "{}: Box type '{}' not found in nyash.toml",
                "ERROR".red(),
                box_type
            );
            return;
        }
    };

    println!("Found in library: {}", lib_name.cyan());

    // Get box configuration
    let box_config = match get_box_config(&raw_config, lib_name, box_type) {
        Some(cfg) => cfg,
        None => {
            eprintln!("{}: No configuration for box type", "ERROR".red());
            return;
        }
    };

    println!("Type ID: {}", box_config.type_id);

    // Resolve plugin path relative to config dir
    let config_base = config_path.parent().unwrap_or(Path::new("."));
    let lib_path = resolve_plugin_path(config_base, &lib_def.path);

    // Load plugin
    let library = match unsafe { Library::new(&lib_path) } {
        Ok(lib) => lib,
        Err(e) => {
            eprintln!(
                "{}: Failed to load plugin: {} (path: {})",
                "ERROR".red(),
                e,
                lib_path.display()
            );
            return;
        }
    };

    // Get invoke function
    let invoke_fn: Symbol<
        unsafe extern "C" fn(u32, u32, u32, *const u8, usize, *mut u8, *mut usize) -> i32,
    > = match unsafe { library.get(b"nyash_plugin_invoke") } {
        Ok(f) => f,
        Err(_) => {
            eprintln!("{}: nyash_plugin_invoke not found", "ERROR".red());
            return;
        }
    };

    unsafe {
        // Test birth
        println!("\n{}", "1. Testing birth (constructor)...".cyan());

        let args = tlv_encode_empty(); // No arguments for FileBox birth
        let mut result_buf = vec![0u8; 1024];
        let mut result_len = result_buf.len();

        let result = invoke_fn(
            box_config.type_id,
            0, // method_id = 0 (birth)
            0, // instance_id = 0 (static/birth)
            args.as_ptr(),
            args.len(),
            result_buf.as_mut_ptr(),
            &mut result_len,
        );

        if result != 0 {
            eprintln!("{}: Birth failed with code {}", "ERROR".red(), result);
            return;
        }

        // Parse instance_id from result
        let instance_id = match tlv_decode_u32(&result_buf[..result_len]) {
            Ok(id) => id,
            Err(e) => {
                eprintln!("{}: Failed to decode instance_id: {}", "ERROR".red(), e);
                return;
            }
        };

        println!(
            "{}: Birth successful, instance_id = {}",
            "✓".green(),
            instance_id
        );

        // 1b. Open the first instance (if open exists)
        if let Some(open_def) = box_config.methods.get("open") {
            println!(
                "\n{}",
                "1b. Opening src FileBox (id=instance_id) ...".cyan()
            );
            let args_open = tlv_encode_two_strings("test_lifecycle.txt", "w");
            let mut out = vec![0u8; 1024];
            let mut out_len = out.len();
            let rc = invoke_fn(
                box_config.type_id,
                open_def.method_id,
                instance_id,
                args_open.as_ptr(),
                args_open.len(),
                out.as_mut_ptr(),
                &mut out_len,
            );
            if rc == 0 {
                println!("{}: open ok", "✓".green());
            } else {
                eprintln!("{}: open rc={}", "WARN".yellow(), rc);
            }
        }

        // 1c. Write some bytes (if write exists)
        if let Some(write_def) = box_config.methods.get("write") {
            println!("\n{}", "1c. Writing to src FileBox ...".cyan());
            let args_write = tlv_encode_bytes(b"hello nyash");
            let mut out = vec![0u8; 1024];
            let mut out_len = out.len();
            let rc = invoke_fn(
                box_config.type_id,
                write_def.method_id,
                instance_id,
                args_write.as_ptr(),
                args_write.len(),
                out.as_mut_ptr(),
                &mut out_len,
            );
            if rc == 0 {
                println!("{}: write ok", "✓".green());
            } else {
                eprintln!("{}: write rc={}", "WARN".yellow(), rc);
            }
        }

        // 1d. Create destination instance via cloneSelf() if available; else birth
        let mut dst_id = None;
        if let Some(clone_def) = box_config.methods.get("cloneSelf") {
            println!("\n{}", "1d. Cloning via cloneSelf() ...".cyan());
            let args0 = tlv_encode_empty();
            let mut out = vec![0u8; 1024];
            let mut out_len = out.len();
            let rc = invoke_fn(
                box_config.type_id,
                clone_def.method_id,
                instance_id,
                args0.as_ptr(),
                args0.len(),
                out.as_mut_ptr(),
                &mut out_len,
            );
            if rc == 0 && out_len >= 16 && out[4] == 8 {
                // Handle
                // parse handle payload at bytes 8..16
                let t = u32::from_le_bytes([out[8], out[9], out[10], out[11]]);
                let i = u32::from_le_bytes([out[12], out[13], out[14], out[15]]);
                if t == box_config.type_id {
                    dst_id = Some(i);
                    println!("{}: cloneSelf returned id={}", "✓".green(), i);
                }
            } else {
                eprintln!("{}: cloneSelf rc={}", "WARN".yellow(), rc);
            }
        }
        if dst_id.is_none() {
            println!("\n{}", "1d. Cloning fallback via birth() ...".cyan());
            let args0 = tlv_encode_empty();
            let mut out = vec![0u8; 1024];
            let mut out_len = out.len();
            let rc = invoke_fn(
                box_config.type_id,
                0,
                0,
                args0.as_ptr(),
                args0.len(),
                out.as_mut_ptr(),
                &mut out_len,
            );
            if rc == 0 {
                dst_id = tlv_decode_u32(&out[..out_len]).ok();
            }
            if let Some(i) = dst_id {
                println!("{}: birth dst id={}", "✓".green(), i);
            } else {
                eprintln!("{}: birth dst failed rc={}", "WARN".yellow(), rc);
            }
        }

        // 1e. copyFrom(dst <- src)
        if let (Some(copy_def), Some(dst)) = (box_config.methods.get("copyFrom"), dst_id) {
            println!("\n{}", "1e. Testing copyFrom(dst <- src) ...".cyan());
            let arg_buf = tlv_encode_one_handle(box_config.type_id, instance_id);
            let mut out = vec![0u8; 1024];
            let mut out_len = out.len();
            let rc = invoke_fn(
                box_config.type_id,
                copy_def.method_id,
                dst,
                arg_buf.as_ptr(),
                arg_buf.len(),
                out.as_mut_ptr(),
                &mut out_len,
            );
            if rc == 0 {
                println!("{}: copyFrom ok", "✓".green());
            } else {
                eprintln!("{}: copyFrom rc={}", "WARN".yellow(), rc);
            }
        }

        // 1f. close both
        if let Some(close_def) = box_config.methods.get("close") {
            println!("\n{}", "1f. Closing both instances ...".cyan());
            let args0 = tlv_encode_empty();
            let mut out = vec![0u8; 64];
            let mut out_len = out.len();
            let _ = invoke_fn(
                box_config.type_id,
                close_def.method_id,
                instance_id,
                args0.as_ptr(),
                args0.len(),
                out.as_mut_ptr(),
                &mut out_len,
            );
            if let Some(dst) = dst_id {
                out_len = out.len();
                let _ = invoke_fn(
                    box_config.type_id,
                    close_def.method_id,
                    dst,
                    args0.as_ptr(),
                    args0.len(),
                    out.as_mut_ptr(),
                    &mut out_len,
                );
            }
            println!("{}: close done", "✓".green());
        }

        // Optional: If method 'cloneSelf' exists, call it and verify Handle return
        if box_config.methods.contains_key("cloneSelf") {
            println!(
                "\n{}",
                "1c. Testing method returning Box: cloneSelf() ...".cyan()
            );
            let args0 = tlv_encode_empty();
            let mut out = vec![0u8; 1024];
            let mut out_len = out.len();
            let method_id = box_config.methods.get("cloneSelf").unwrap().method_id;
            let rc = invoke_fn(
                box_config.type_id,
                method_id,
                instance_id,
                args0.as_ptr(),
                args0.len(),
                out.as_mut_ptr(),
                &mut out_len,
            );
            if rc == 0 {
                // Parse TLV header + entry, expecting tag=8 size=8
                if out_len >= 12 && out[4] == 8 && out[7] as usize == 8 {
                    // simplistic check
                    println!("{}: cloneSelf returned a Handle (tag=8)", "✓".green());
                } else {
                    eprintln!("{}: cloneSelf returned unexpected format", "WARN".yellow());
                }
            } else {
                eprintln!("{}: cloneSelf call failed (rc={})", "WARN".yellow(), rc);
            }
        }

        // Test fini
        println!("\n{}", "2. Testing fini (destructor)...".cyan());

        result_len = result_buf.len();
        let result = invoke_fn(
            box_config.type_id,
            4294967295, // method_id = 0xFFFFFFFF (fini)
            instance_id,
            args.as_ptr(),
            args.len(),
            result_buf.as_mut_ptr(),
            &mut result_len,
        );

        if result != 0 {
            eprintln!("{}: Fini failed with code {}", "ERROR".red(), result);
        } else {
            println!("{}: Fini successful", "✓".green());
        }
    }

    println!("\n{}", "Lifecycle test completed!".green().bold());
}
