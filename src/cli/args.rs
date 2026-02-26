use super::utils::parse_debug_fuel;
use super::CliConfig;
use clap::{Arg, ArgMatches, Command};
use serde_json;

pub fn parse() -> CliConfig {
    let argv: Vec<String> = std::env::args().collect();
    if let Some(pos) = argv.iter().position(|s| s == "--") {
        let script_args: Vec<String> = argv.iter().skip(pos + 1).cloned().collect();
        if !script_args.is_empty() {
            if let Ok(json) = serde_json::to_string(&script_args) {
                std::env::set_var("NYASH_SCRIPT_ARGS_JSON", json);
            }
            // Provide HEX-escaped JSON as an alternate robust path for multiline/special bytes
            // Each arg is encoded as lowercase hex of its UTF-8 bytes
            let hex_args: Vec<String> = script_args.iter().map(|s| hex_encode_utf8(s)).collect();
            if let Ok(hex_json) = serde_json::to_string(&hex_args) {
                std::env::set_var("NYASH_SCRIPT_ARGS_HEX_JSON", hex_json);
            }
        }
        let matches = build_command()
            .try_get_matches_from(&argv[..pos])
            .unwrap_or_else(|e| e.exit());
        from_matches(&matches)
    } else {
        let matches = build_command().get_matches();
        from_matches(&matches)
    }
}

pub fn build_command() -> Command {
    Command::new("nyash")
        .version(if cfg!(feature = "llvm") { "1.0 features:llvm" } else { "1.0" })
        .author("Claude Code <claude@anthropic.com>")
        .about("🦀 Nyash Programming Language - Everything is Box in Rust! 🦀")
        .arg(Arg::new("dev").long("dev").help("Enable development defaults (AST using ON; Operator Boxes observe; safe diagnostics)").action(clap::ArgAction::SetTrue))
        .arg(Arg::new("file").help("Nyash file to execute").value_name("FILE").index(1))
        .arg(Arg::new("macro-expand-child").long("macro-expand-child").value_name("FILE").help("Macro sandbox child: read AST JSON v0 from stdin, expand using Nyash macro file, write AST JSON v0 to stdout (PoC)"))
        .arg(Arg::new("dump-ast").long("dump-ast").help("Dump parsed AST and exit").action(clap::ArgAction::SetTrue))
        .arg(Arg::new("macro-preexpand").long("macro-preexpand").help("Enable selfhost macro pre-expand").action(clap::ArgAction::SetTrue))
        .arg(Arg::new("macro-preexpand-auto").long("macro-preexpand-auto").help("Auto enable selfhost macro pre-expand").action(clap::ArgAction::SetTrue))
        .arg(Arg::new("macro-top-level-allow").long("macro-top-level-allow").help("Allow top-level macro usage").action(clap::ArgAction::SetTrue))
        .arg(Arg::new("macro-profile").long("macro-profile").value_name("{dev|ci-fast|strict}").help("Select macro profile"))
        .arg(Arg::new("dump-expanded-ast-json").long("dump-expanded-ast-json").help("Dump AST after macro expansion as JSON v0 and exit").action(clap::ArgAction::SetTrue))
        .arg(Arg::new("macro-ctx-json").long("macro-ctx-json").value_name("JSON").help("Provide MacroCtx as JSON string for macro child routes"))
        .arg(
            Arg::new("gc")
                .long("gc")
                .value_name(crate::runtime::gc_mode::GC_MODE_CLI_VALUE_NAME)
                .value_parser(clap::builder::PossibleValuesParser::new(
                    crate::runtime::gc_mode::GC_MODE_ALLOWED_VALUES,
                ))
                .help("Select GC mode (default: rc+cycle)"),
        )
        .arg(Arg::new("ny-parser-pipe").long("ny-parser-pipe").help("Read Ny JSON IR v0 from stdin and execute via MIR Interpreter").action(clap::ArgAction::SetTrue))
        .arg(Arg::new("json-file").long("json-file").value_name("FILE").help("Read Ny JSON IR v0 from a file and execute via MIR Interpreter"))
        .arg(Arg::new("mir-json-file").long("mir-json-file").value_name("FILE").help("[Diagnostic] Read MIR JSON v0 from a file and perform minimal validation/inspection (experimental)") )
        .arg(Arg::new("emit-mir-json").long("emit-mir-json").value_name("FILE").help("Emit MIR JSON v0 to file and exit"))
        .arg(Arg::new("emit-ast-json").long("emit-ast-json").value_name("FILE").help("Emit AST JSON to file and exit (direct Rust parser route)"))
        .arg(Arg::new("emit-program-json").long("emit-program-json").value_name("FILE").help("[Deprecated] Alias of --emit-ast-json (was misnamed)"))
        .arg(Arg::new("emit-program-json-v0").long("emit-program-json-v0").value_name("FILE").help("Emit Program(JSON v0) to file and exit (Stage-1 stub route)"))
        .arg(
            Arg::new("hako-emit-program-json")
                .long("hako-emit-program-json")
                .value_name("FILE")
                .help("Emit Program(JSON v0) via Stage-1 (.hako) stub and exit")
                .conflicts_with_all([
                    "emit-program-json-v0",
                    "emit-program-json",
                    "emit-ast-json",
                    "emit-mir-json",
                    "hako-emit-mir-json",
                    "hako-run",
                    "program-json-to-mir",
                ]),
        )
        .arg(
            Arg::new("hako-emit-mir-json")
                .long("hako-emit-mir-json")
                .value_name("FILE")
                .help("Emit MIR(JSON) via Stage-1 (.hako) stub (json_v0_bridge path)")
                .conflicts_with_all([
                    "emit-mir-json",
                    "emit-program-json-v0",
                    "emit-program-json",
                    "emit-ast-json",
                    "hako-emit-program-json",
                    "hako-run",
                    "program-json-to-mir",
                ]),
        )
        .arg(
            Arg::new("hako-run")
                .long("hako-run")
                .help("Run via Stage-1 (.hako) stub (equivalent to NYASH_USE_STAGE1_CLI=1)")
                .action(clap::ArgAction::SetTrue)
                .conflicts_with_all([
                    "hako-emit-program-json",
                    "hako-emit-mir-json",
                    "emit-program-json-v0",
                    "emit-program-json",
                    "emit-ast-json",
                    "emit-mir-json",
                    "program-json-to-mir",
                ]),
        )
        .arg(Arg::new("program-json-to-mir").long("program-json-to-mir").value_name("FILE").help("Convert Program(JSON v0) to MIR(JSON) and exit (use with --json-file)"))
        .arg(Arg::new("emit-exe").long("emit-exe").value_name("FILE").help("Emit native executable via ny-llvmc and exit"))
        .arg(Arg::new("emit-exe-nyrt").long("emit-exe-nyrt").value_name("DIR").help("Directory containing libnyash_kernel.a (used with --emit-exe). Hint: build via `cargo build -p nyash_kernel --release` (default output: target/release/libnyash_kernel.a)"))
        .arg(Arg::new("emit-exe-libs").long("emit-exe-libs").value_name("FLAGS").help("Extra linker flags for ny-llvmc when emitting executable"))
        .arg(Arg::new("stage3").long("stage3").help("Enable Stage-3 syntax acceptance for selfhost parser").action(clap::ArgAction::SetTrue))
        .arg(Arg::new("ny-compiler-args").long("ny-compiler-args").value_name("ARGS").help("Pass additional args to selfhost child compiler"))
        .arg(Arg::new("using").long("using").value_name("NAME").help("Add a using directive to current session; repeat").action(clap::ArgAction::Append))
        .arg(Arg::new("debug-fuel").long("debug-fuel").value_name("N|unlimited").help("Limit interpreter/JIT steps or 'unlimited' (default 100000)").default_value("100000"))
        .arg(Arg::new("run-tests").long("run-tests").help("Run inline tests in the module (functions starting with 'test_')").action(clap::ArgAction::SetTrue))
        .arg(Arg::new("test-filter").long("test-filter").value_name("SUBSTR").help("Only run tests whose name contains SUBSTR (with --run-tests)"))
        .arg(Arg::new("test-entry").long("test-entry").value_name("{wrap|override}").help("When --run-tests and a main exists: wrap or override") )
        .arg(Arg::new("test-return").long("test-return").value_name("{tests|original}").help("Harness return policy (tests or original)") )
        .arg(Arg::new("dump-mir").long("dump-mir").help("Dump MIR instead of executing").action(clap::ArgAction::SetTrue))
        .arg(Arg::new("verify").long("verify").help("Verify MIR integrity and exit").action(clap::ArgAction::SetTrue))
        .arg(Arg::new("mir-verbose").long("mir-verbose").help("Show verbose MIR output with statistics").action(clap::ArgAction::SetTrue))
        .arg(Arg::new("mir-verbose-effects").long("mir-verbose-effects").help("Show per-instruction effect category").action(clap::ArgAction::SetTrue))
        .arg(Arg::new("no-optimize").long("no-optimize").help("Disable MIR optimizer passes").action(clap::ArgAction::SetTrue))
        .arg(Arg::new("backend").long("backend").value_name("BACKEND").help("Backend: vm (default), vm-hako (S0 frame), llvm, interpreter").default_value("vm"))
        .arg(Arg::new("verbose").long("verbose").short('v').help("Verbose CLI output (sets NYASH_CLI_VERBOSE=1)").action(clap::ArgAction::SetTrue))
        .arg(Arg::new("compile-wasm").long("compile-wasm").help("Compile to WebAssembly binary (.wasm)").action(clap::ArgAction::SetTrue))
        .arg(
            Arg::new("emit-wat")
                .long("emit-wat")
                .value_name("FILE")
                .help("Emit WebAssembly Text (WAT) to FILE and exit")
                .conflicts_with("compile-wasm")
                .conflicts_with("compile-native")
                .conflicts_with("aot"),
        )
        .arg(Arg::new("compile-native").long("compile-native").help("Compile to native executable (AOT)").action(clap::ArgAction::SetTrue))
        .arg(Arg::new("aot").long("aot").help("Short form of --compile-native").action(clap::ArgAction::SetTrue))
        .arg(Arg::new("output").long("output").short('o').value_name("FILE").help("Output file for compilation"))
        .arg(Arg::new("benchmark").long("benchmark").help("Run performance benchmarks").action(clap::ArgAction::SetTrue))
        .arg(Arg::new("iterations").long("iterations").value_name("COUNT").help("Iterations for benchmarks").default_value("10"))
        .arg(Arg::new("vm-stats").long("vm-stats").help("Enable VM instruction statistics").action(clap::ArgAction::SetTrue))
        .arg(Arg::new("vm-stats-json").long("vm-stats-json").help("Output VM statistics in JSON").action(clap::ArgAction::SetTrue))
        .arg(Arg::new("jit-exec").long("jit-exec").help("Enable JIT execution").action(clap::ArgAction::SetTrue))
        .arg(Arg::new("jit-stats").long("jit-stats").help("Print JIT statistics").action(clap::ArgAction::SetTrue))
        .arg(Arg::new("jit-stats-json").long("jit-stats-json").help("Output JIT stats in JSON").action(clap::ArgAction::SetTrue))
        .arg(Arg::new("jit-dump").long("jit-dump").help("Dump JIT lowering summary").action(clap::ArgAction::SetTrue))
        .arg(Arg::new("jit-events").long("jit-events").help("Emit JIT events JSONL").action(clap::ArgAction::SetTrue))
        .arg(Arg::new("jit-events-compile").long("jit-events-compile").help("Emit compile-time JIT events").action(clap::ArgAction::SetTrue))
        .arg(Arg::new("jit-events-runtime").long("jit-events-runtime").help("Emit runtime JIT events").action(clap::ArgAction::SetTrue))
        .arg(Arg::new("jit-events-path").long("jit-events-path").value_name("FILE").help("Write JIT events JSONL to file"))
        .arg(Arg::new("jit-threshold").long("jit-threshold").value_name("N").help("Hotness threshold for JIT compilation"))
        .arg(Arg::new("jit-phi-min").long("jit-phi-min").help("Minimal PHI path for branches").action(clap::ArgAction::SetTrue))
        .arg(Arg::new("jit-hostcall").long("jit-hostcall").help("Enable JIT hostcall bridge").action(clap::ArgAction::SetTrue))
        .arg(Arg::new("jit-handle-debug").long("jit-handle-debug").help("Print JIT handle allocation debug logs").action(clap::ArgAction::SetTrue))
        .arg(Arg::new("jit-native-f64").long("jit-native-f64").help("Enable native f64 ABI path").action(clap::ArgAction::SetTrue))
        .arg(Arg::new("jit-native-bool").long("jit-native-bool").help("Enable native bool ABI path").action(clap::ArgAction::SetTrue))
        .arg(Arg::new("jit-only").long("jit-only").help("Run JIT only (no VM fallback)").action(clap::ArgAction::SetTrue))
        .arg(Arg::new("jit-direct").long("jit-direct").help("Independent JIT engine mode").action(clap::ArgAction::SetTrue))
        .arg(Arg::new("emit-cfg").long("emit-cfg").value_name("DOT_FILE").help("Emit JIT CFG as DOT"))
        .arg(Arg::new("run-task").long("run-task").value_name("NAME").help("Run a named task from nyash.toml"))
        .arg(Arg::new("load-ny-plugins").long("load-ny-plugins").help("Load scripts from nyash.toml [ny_plugins]").action(clap::ArgAction::SetTrue))
        .arg(Arg::new("build").long("build").value_name("PATH").help("Build AOT executable using nyash.toml at PATH (MVP)"))
        .arg(Arg::new("build-app").long("app").value_name("FILE").help("Entry Nyash script for --build"))
        .arg(Arg::new("build-out").long("out").value_name("FILE").help("Output executable name for --build"))
        .arg(Arg::new("build-aot").long("build-aot").value_name("{cranelift|llvm}").help("AOT backend for --build"))
        .arg(Arg::new("build-profile").long("profile").value_name("{release|debug}").help("Cargo profile for --build"))
        .arg(Arg::new("build-target").long("target").value_name("TRIPLE").help("Target triple for --build"))
        // Phase 288 P1: REPL mode
        .arg(Arg::new("repl")
            .long("repl")
            .short('i')
            .help("Start interactive REPL (Read-Eval-Print Loop)")
            .action(clap::ArgAction::SetTrue))
}

fn hex_encode_utf8(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut out = String::with_capacity(bytes.len() * 2);
    const HEX: &[u8; 16] = b"0123456789abcdef";
    for &b in bytes {
        out.push(HEX[(b >> 4) as usize] as char);
        out.push(HEX[(b & 0x0f) as usize] as char);
    }
    out
}

pub fn from_matches(matches: &ArgMatches) -> CliConfig {
    if matches.get_flag("stage3") {
        std::env::set_var("NYASH_NY_COMPILER_STAGE3", "1");
    }
    if let Some(a) = matches.get_one::<String>("ny-compiler-args") {
        std::env::set_var("NYASH_NY_COMPILER_CHILD_ARGS", a);
    }
    let hako_emit_program_path = matches.get_one::<String>("hako-emit-program-json").cloned();
    let hako_emit_mir_path = matches.get_one::<String>("hako-emit-mir-json").cloned();
    let cfg = CliConfig {
        file: matches.get_one::<String>("file").cloned(),
        debug_fuel: parse_debug_fuel(matches.get_one::<String>("debug-fuel").unwrap()),
        dump_ast: matches.get_flag("dump-ast"),
        dump_mir: matches.get_flag("dump-mir"),
        verify_mir: matches.get_flag("verify"),
        mir_verbose: matches.get_flag("mir-verbose"),
        mir_verbose_effects: matches.get_flag("mir-verbose-effects"),
        no_optimize: matches.get_flag("no-optimize"),
        backend: matches.get_one::<String>("backend").unwrap().clone(),
        compile_wasm: matches.get_flag("compile-wasm"),
        compile_native: matches.get_flag("compile-native") || matches.get_flag("aot"),
        output_file: matches.get_one::<String>("output").cloned(),
        benchmark: matches.get_flag("benchmark"),
        iterations: matches
            .get_one::<String>("iterations")
            .unwrap()
            .parse()
            .unwrap_or(10),
        vm_stats: matches.get_flag("vm-stats"),
        vm_stats_json: matches.get_flag("vm-stats-json"),
        jit_exec: matches.get_flag("jit-exec"),
        jit_stats: matches.get_flag("jit-stats"),
        jit_stats_json: matches.get_flag("jit-stats-json"),
        jit_dump: matches.get_flag("jit-dump"),
        jit_events: matches.get_flag("jit-events"),
        jit_events_compile: matches.get_flag("jit-events-compile"),
        jit_events_runtime: matches.get_flag("jit-events-runtime"),
        jit_events_path: matches.get_one::<String>("jit-events-path").cloned(),
        jit_threshold: matches
            .get_one::<String>("jit-threshold")
            .and_then(|s| s.parse::<u32>().ok()),
        jit_phi_min: matches.get_flag("jit-phi-min"),
        jit_hostcall: matches.get_flag("jit-hostcall"),
        jit_handle_debug: matches.get_flag("jit-handle-debug"),
        jit_native_f64: matches.get_flag("jit-native-f64"),
        jit_native_bool: matches.get_flag("jit-native-bool"),
        emit_cfg: matches.get_one::<String>("emit-cfg").cloned(),
        emit_wat: matches.get_one::<String>("emit-wat").cloned(),
        jit_only: matches.get_flag("jit-only"),
        jit_direct: matches.get_flag("jit-direct"),
        cli_verbose: matches.get_flag("verbose"),
        run_task: matches.get_one::<String>("run-task").cloned(),
        load_ny_plugins: matches.get_flag("load-ny-plugins"),
        gc_mode: matches.get_one::<String>("gc").cloned(),
        ny_parser_pipe: matches.get_flag("ny-parser-pipe"),
        json_file: matches.get_one::<String>("json-file").cloned(),
        mir_json_file: matches.get_one::<String>("mir-json-file").cloned(),
        build_path: matches.get_one::<String>("build").cloned(),
        build_app: matches.get_one::<String>("build-app").cloned(),
        build_out: matches.get_one::<String>("build-out").cloned(),
        build_aot: matches.get_one::<String>("build-aot").cloned(),
        build_profile: matches.get_one::<String>("build-profile").cloned(),
        build_target: matches.get_one::<String>("build-target").cloned(),
        cli_usings: matches
            .get_many::<String>("using")
            .map(|v| v.cloned().collect())
            .unwrap_or_else(|| Vec::new()),
        emit_mir_json: matches.get_one::<String>("emit-mir-json").cloned(),
        emit_ast_json: matches
            .get_one::<String>("emit-ast-json")
            .cloned()
            .or_else(|| matches.get_one::<String>("emit-program-json").cloned()),
        emit_program_json_v0: matches.get_one::<String>("emit-program-json-v0").cloned(),
        hako_emit_program_json: hako_emit_program_path.is_some(),
        hako_emit_mir_json: hako_emit_mir_path.is_some(),
        hako_run: matches.get_flag("hako-run"),
        program_json_to_mir: matches.get_one::<String>("program-json-to-mir").cloned(),
        emit_exe: matches.get_one::<String>("emit-exe").cloned(),
        emit_exe_nyrt: matches.get_one::<String>("emit-exe-nyrt").cloned(),
        emit_exe_libs: matches.get_one::<String>("emit-exe-libs").cloned(),
        macro_expand_child: matches.get_one::<String>("macro-expand-child").cloned(),
        dump_expanded_ast_json: matches.get_flag("dump-expanded-ast-json"),
        macro_ctx_json: matches.get_one::<String>("macro-ctx-json").cloned(),
        // Phase 288 P1: REPL mode
        repl: matches.get_flag("repl"),
    };

    if cfg.cli_verbose {
        std::env::set_var("NYASH_CLI_VERBOSE", "1");
    }
    if cfg.vm_stats {
        std::env::set_var("NYASH_VM_STATS", "1");
    }
    if cfg.vm_stats_json {
        std::env::set_var("NYASH_VM_STATS_JSON", "1");
    }
    // hako-prefixed Stage-1 stub routes
    if cfg.hako_emit_program_json {
        std::env::set_var("NYASH_USE_STAGE1_CLI", "1");
        std::env::set_var("HAKO_STAGE1_MODE", "emit-program");
        std::env::set_var("HAKO_EMIT_PROGRAM_JSON", "1");
        std::env::set_var("STAGE1_EMIT_PROGRAM_JSON", "1");
        if let Some(out) = hako_emit_program_path.as_ref() {
            std::env::set_var("NYASH_STAGE1_EMIT_PROGRAM_OUT", out);
        }
        if let Some(f) = cfg.file.as_ref() {
            std::env::set_var("HAKO_STAGE1_INPUT", f);
            std::env::set_var("NYASH_STAGE1_INPUT", f);
        }
    }
    if cfg.hako_emit_mir_json {
        std::env::set_var("NYASH_USE_STAGE1_CLI", "1");
        std::env::set_var("HAKO_STAGE1_MODE", "emit-mir");
        std::env::set_var("HAKO_EMIT_MIR_JSON", "1");
        std::env::set_var("STAGE1_EMIT_MIR_JSON", "1");
        if let Some(out) = hako_emit_mir_path.as_ref() {
            std::env::set_var("NYASH_STAGE1_EMIT_MIR_OUT", out);
        }
        if let Some(f) = cfg.file.as_ref() {
            std::env::set_var("HAKO_STAGE1_INPUT", f);
            std::env::set_var("NYASH_STAGE1_INPUT", f);
        }
    }
    if cfg.hako_run {
        std::env::set_var("NYASH_USE_STAGE1_CLI", "1");
        std::env::set_var("HAKO_STAGE1_MODE", "run");
        if let Some(f) = cfg.file.as_ref() {
            std::env::set_var("HAKO_STAGE1_INPUT", f);
            std::env::set_var("NYASH_STAGE1_INPUT", f);
        }
    }
    if cfg.jit_exec {
        std::env::set_var("NYASH_JIT_EXEC", "1");
    }
    if cfg.jit_stats {
        std::env::set_var("NYASH_JIT_STATS", "1");
    }
    if cfg.jit_stats_json {
        std::env::set_var("NYASH_JIT_STATS_JSON", "1");
    }
    if cfg.jit_dump {
        std::env::set_var("NYASH_JIT_DUMP", "1");
    }
    if cfg.jit_events {
        std::env::set_var("NYASH_JIT_EVENTS", "1");
    }
    if cfg.jit_events_compile {
        std::env::set_var("NYASH_JIT_EVENTS_COMPILE", "1");
    }
    if cfg.jit_events_runtime {
        std::env::set_var("NYASH_JIT_EVENTS_RUNTIME", "1");
    }
    if let Some(p) = &cfg.jit_events_path {
        std::env::set_var("NYASH_JIT_EVENTS_PATH", p);
    }
    if let Some(t) = cfg.jit_threshold {
        std::env::set_var("NYASH_JIT_THRESHOLD", t.to_string());
    }
    if cfg.jit_phi_min {
        std::env::set_var("NYASH_JIT_PHI_MIN", "1");
    }
    if cfg.jit_hostcall {
        std::env::set_var("NYASH_JIT_HOSTCALL", "1");
    }
    if cfg.jit_handle_debug {
        std::env::set_var("NYASH_JIT_HANDLE_DEBUG", "1");
    }
    if cfg.jit_native_f64 {
        std::env::set_var("NYASH_JIT_NATIVE_F64", "1");
    }
    if cfg.jit_native_bool {
        std::env::set_var("NYASH_JIT_NATIVE_BOOL", "1");
    }
    if cfg.jit_only {
        std::env::set_var("NYASH_JIT_ONLY", "1");
    }
    if cfg.jit_direct {
        std::env::set_var("NYASH_JIT_DIRECT", "1");
    }
    if let Some(gc) = &cfg.gc_mode {
        std::env::set_var("NYASH_GC_MODE", gc);
    }

    if matches.get_flag("run-tests") {
        std::env::set_var("NYASH_RUN_TESTS", "1");
        if let Some(filter) = matches.get_one::<String>("test-filter") {
            std::env::set_var("NYASH_TEST_FILTER", filter);
        }
        if let Some(entry) = matches.get_one::<String>("test-entry") {
            let v = entry.as_str();
            if v == "wrap" || v == "override" {
                std::env::set_var("NYASH_TEST_ENTRY", v);
            }
        }
        if let Some(ret) = matches.get_one::<String>("test-return") {
            let v = ret.as_str();
            if v == "tests" || v == "original" {
                std::env::set_var("NYASH_TEST_RETURN", v);
            }
        }
    }
    if matches.get_flag("macro-preexpand") {
        std::env::set_var("NYASH_MACRO_SELFHOST_PRE_EXPAND", "1");
    }
    if matches.get_flag("macro-preexpand-auto") {
        std::env::set_var("NYASH_MACRO_SELFHOST_PRE_EXPAND", "auto");
    }
    if matches.get_flag("macro-top-level-allow") {
        std::env::set_var("NYASH_MACRO_TOPLEVEL_ALLOW", "1");
    }
    if let Some(p) = matches.get_one::<String>("macro-profile") {
        match p.as_str() {
            "dev" | "ci-fast" | "strict" => {
                std::env::set_var("NYASH_MACRO_ENABLE", "1");
                std::env::set_var("NYASH_MACRO_STRICT", "1");
                std::env::set_var("NYASH_MACRO_TOPLEVEL_ALLOW", "0");
                std::env::set_var("NYASH_MACRO_SELFHOST_PRE_EXPAND", "auto");
            }
            _ => {}
        }
    }

    // --dev flag (or NYASH_DEV=1) enables safe development defaults
    // - AST using ON (NYASH_USING_AST=1)
    // - Operator Boxes observe ON (Stringify/Compare/Add) and prelude injection (NYASH_OPERATOR_BOX_ALL=1)
    //   (Adopt/tolerate are explicit; builder-call OFF)
    // - Keep production behavior otherwise (no plugin/trace changes here)
    if matches.get_flag("dev") || std::env::var("NYASH_DEV").ok().as_deref() == Some("1") {
        // Profile hint
        std::env::set_var("NYASH_USING_PROFILE", "dev");
        // AST prelude merge
        std::env::set_var("NYASH_USING_AST", "1");
        // Using grammar is mainline; keep explicit enable for clarity (default is ON; this makes intent obvious in dev)
        std::env::set_var("NYASH_ENABLE_USING", "1");
        // Allow top-level main resolution in dev for convenience (prod default remains OFF)
        std::env::set_var("NYASH_ENTRY_ALLOW_TOPLEVEL_MAIN", "1");
        // Ensure project root is available for prelude injection
        if std::env::var("NYASH_ROOT").is_err() {
            if let Ok(cwd) = std::env::current_dir() {
                std::env::set_var("NYASH_ROOT", cwd.display().to_string());
            }
        }
        // Operator Boxes: observe + prelude injection
        std::env::set_var("NYASH_OPERATOR_BOX_ALL", "1");
        std::env::set_var("NYASH_OPERATOR_BOX_STRINGIFY", "1");
        std::env::set_var("NYASH_OPERATOR_BOX_COMPARE", "1");
        std::env::set_var("NYASH_OPERATOR_BOX_ADD", "1");
        // Builder-call ALL is still OFF here to keep MIR shape stable.
    }

    cfg
}

#[cfg(test)]
mod tests {
    use super::*;

    fn clear_stage1_env() {
        for key in [
            "NYASH_USE_STAGE1_CLI",
            "HAKO_STAGE1_MODE",
            "HAKO_EMIT_PROGRAM_JSON",
            "STAGE1_EMIT_PROGRAM_JSON",
            "HAKO_EMIT_MIR_JSON",
            "STAGE1_EMIT_MIR_JSON",
            "NYASH_STAGE1_EMIT_PROGRAM_OUT",
            "NYASH_STAGE1_EMIT_MIR_OUT",
            "HAKO_STAGE1_INPUT",
            "NYASH_STAGE1_INPUT",
        ] {
            std::env::remove_var(key);
        }
    }

    #[test]
    fn hako_emit_program_does_not_alias_emit_program_json_v0() {
        clear_stage1_env();
        let matches = build_command()
            .try_get_matches_from(["hakorune", "--hako-emit-program-json", "/tmp/out.json"])
            .expect("hako emit program args should parse");

        let cfg = from_matches(&matches);
        assert!(cfg.hako_emit_program_json);
        assert_eq!(cfg.emit_program_json_v0, None);
    }

    #[test]
    fn hako_emit_mir_conflicts_with_emit_mir_json() {
        let result = build_command().try_get_matches_from([
            "hakorune",
            "--hako-emit-mir-json",
            "/tmp/hako.json",
            "--emit-mir-json",
            "/tmp/rust.json",
        ]);
        assert!(result.is_err(), "conflicting emit routes must be rejected");
    }

    #[test]
    fn emit_wat_route_parses_and_sets_output_path() {
        let matches = build_command()
            .try_get_matches_from(["hakorune", "--emit-wat", "/tmp/out.wat", "apps/min.hako"])
            .expect("emit-wat args should parse");

        let cfg = from_matches(&matches);
        assert_eq!(cfg.emit_wat.as_deref(), Some("/tmp/out.wat"));
        assert_eq!(cfg.file.as_deref(), Some("apps/min.hako"));
    }

    #[test]
    fn emit_wat_conflicts_with_compile_wasm() {
        let result = build_command().try_get_matches_from([
            "hakorune",
            "--emit-wat",
            "/tmp/out.wat",
            "--compile-wasm",
            "apps/min.hako",
        ]);
        assert!(result.is_err(), "emit-wat and compile-wasm must conflict");
    }
}
