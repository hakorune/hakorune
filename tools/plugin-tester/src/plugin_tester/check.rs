use super::*;

pub(crate) fn check_v2(config_path: &PathBuf, library_filter: Option<&str>) {
    println!("{}", "=== Plugin Check v2 (nyash.toml centric) ===".bold());

    // Load nyash.toml v2
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
            eprintln!("{}: Failed to parse nyash.toml v2: {}", "ERROR".red(), e);
            return;
        }
    };

    println!(
        "{}: Loaded {} libraries from nyash.toml",
        "✓".green(),
        config.libraries.len()
    );

    // Also parse raw TOML for nested box configs
    let raw_config: toml::Value = match toml::from_str(&config_content) {
        Ok(val) => val,
        Err(e) => {
            eprintln!("{}: Failed to parse TOML value: {}", "ERROR".red(), e);
            return;
        }
    };

    // Base dir for relative plugin paths
    let config_base = config_path.parent().unwrap_or(Path::new("."));

    // Check each library
    for (lib_name, lib_def) in &config.libraries {
        if let Some(filter) = library_filter {
            if lib_name != filter {
                continue;
            }
        }

        println!("\n{}: {}", "Library".bold(), lib_name.cyan());
        println!("  Path: {}", lib_def.path);
        println!("  Box types: {:?}", lib_def.boxes);

        // Try to load the plugin
        let lib_path = resolve_plugin_path(config_base, &lib_def.path);
        let library = match unsafe { Library::new(&lib_path) } {
            Ok(lib) => lib,
            Err(e) => {
                eprintln!(
                    "  {}: Failed to load: {} (path: {})",
                    "ERROR".red(),
                    e,
                    lib_path.display()
                );
                continue;
            }
        };

        println!("  {}: Plugin loaded successfully", "✓".green());

        // Check for nyash_plugin_invoke (the only required function!)
        match unsafe {
            library.get::<Symbol<
                unsafe extern "C" fn(u32, u32, u32, *const u8, usize, *mut u8, *mut usize) -> i32,
            >>(b"nyash_plugin_invoke")
        } {
            Ok(_) => println!("  {}: nyash_plugin_invoke found", "✓".green()),
            Err(_) => {
                eprintln!(
                    "  {}: nyash_plugin_invoke NOT FOUND - not a valid v2 plugin!",
                    "ERROR".red()
                );
                continue;
            }
        }

        // Check each box type from nyash.toml
        for box_name in &lib_def.boxes {
            println!("\n  {}: {}", "Box Type".bold(), box_name.cyan());

            // Get box config from nested TOML
            let box_config = get_box_config(&raw_config, lib_name, box_name);

            if let Some(config) = box_config {
                println!("    Type ID: {}", config.type_id);
                println!("    ABI Version: {}", config.abi_version);
                println!("    Methods: {}", config.methods.len());

                // List methods
                for (method_name, method_def) in &config.methods {
                    let method_type = match method_def.method_id {
                        0 => " (constructor)".yellow(),
                        4294967295 => " (destructor)".yellow(),
                        _ => "".normal(),
                    };

                    println!(
                        "    - {}: method_id={}{}",
                        method_name, method_def.method_id, method_type
                    );
                }
            } else {
                eprintln!(
                    "    {}: No configuration found for this box type",
                    "WARNING".yellow()
                );
            }
        }
    }

    println!("\n{}", "Check completed!".green().bold());
}
pub(crate) fn validate_all(config_path: &PathBuf) {
    println!("{}", "=== Validate All Plugins ===".bold());
    check_v2(config_path, None);
}
