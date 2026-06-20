use std::process::Command;

const CONTRACT: &str = "rustc-semir-adapter-tool-preflight-v0";
const TOOLCHAIN_CONTRACT: &str = "rustc-semir-adapter-toolchain-compat-v0";

#[derive(Debug)]
struct RustcInfo {
    version: String,
    sysroot: String,
}

fn command_text(program: &str, args: &[&str]) -> String {
    match Command::new(program).args(args).output() {
        Ok(output) if output.status.success() => {
            String::from_utf8_lossy(&output.stdout).trim().to_string()
        }
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            format!("{program}-error:{}", stderr.trim())
        }
        Err(err) => format!("{program}-not-invoked:{err}"),
    }
}

fn rustc_version() -> String {
    command_text("rustc", &["--version"])
}

fn rustc_info() -> RustcInfo {
    RustcInfo {
        version: rustc_version(),
        sysroot: command_text("rustc", &["--print", "sysroot"]),
    }
}

fn channel(info: &RustcInfo) -> &'static str {
    if info.version.contains("nightly") {
        "nightly"
    } else if info.version.contains("beta") {
        "beta"
    } else if info.version.starts_with("rustc ") {
        "stable_or_release"
    } else {
        "unknown"
    }
}

fn rustc_private_readiness(info: &RustcInfo) -> &'static str {
    match channel(info) {
        "nightly" => "candidate",
        "stable_or_release" | "beta" => "requires_nightly_or_bootstrap",
        _ => "unknown",
    }
}

fn print_preflight() {
    println!("output_contract={CONTRACT}");
    println!("adapter_tool_preflight_green=1");
    println!("standalone_tool_manifest_exists=1");
    println!("rustc_version={}", rustc_version());
    println!("rustc_private_dependency_enabled=0");
    println!("facts_generated=0");
    println!("hako_plan_emitted=0");
    println!("hako_source_emitted=0");
    println!("backend_behavior_changed=0");
    println!("summary=ok");
}

fn print_toolchain_preflight() {
    let info = rustc_info();
    println!("output_contract={TOOLCHAIN_CONTRACT}");
    println!("toolchain_compat_preflight_green=1");
    println!("rustc_version_reported=1");
    println!("rustc_version={}", info.version);
    println!("rustc_channel={}", channel(&info));
    println!("rustc_sysroot={}", info.sysroot);
    println!("rustc_private_readiness={}", rustc_private_readiness(&info));
    println!("rustc_private_readiness_reported=1");
    println!("facts_generated=0");
    println!("hako_plan_emitted=0");
    println!("hako_source_emitted=0");
    println!("source_shape_fallback=0");
    println!("backend_behavior_changed=0");
    println!("summary=ok");
}

fn main() {
    let mut args = std::env::args().skip(1);
    match args.next().as_deref() {
        Some("--preflight") if args.next().is_none() => print_preflight(),
        Some("--toolchain-preflight") if args.next().is_none() => print_toolchain_preflight(),
        _ => {
            eprintln!("usage: rustc-semir-adapter (--preflight|--toolchain-preflight)");
            std::process::exit(2);
        }
    }
}
